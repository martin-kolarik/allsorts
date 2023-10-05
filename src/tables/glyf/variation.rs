use std::collections::BTreeMap;
use std::ops::RangeInclusive;

use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::transform2d::{Matrix2x2F, Transform2F};
use pathfinder_geometry::vector::{vec2f, Vector2F};
use tinyvec::{tiny_vec, TinyVec};

use crate::error::ParseError;
use crate::tables::glyf::{
    calculate_phantom_points, BoundingBox, ComponentOffsets, CompositeGlyph,
    CompositeGlyphArgument, CompositeGlyphComponent, CompositeGlyphFlag, EmptyGlyph, GlyfRecord,
    GlyfTable, Glyph, PhantomPoints, Point, SimpleGlyph, SimpleGlyphFlag,
};
use crate::tables::os2::Os2;
use crate::tables::variable_fonts::gvar::{GvarTable, NumPoints};
use crate::tables::variable_fonts::{
    Gvar, OwnedTuple, Tuple, TupleVariationHeader, TupleVariationStore,
};
use crate::tables::{F2Dot14, HheaTable, HmtxTable};
use crate::SafeFrom;

enum Coordinates<'a> {
    Tuple(Tuple<'a>),
    Array(TinyVec<[F2Dot14; 4]>),
}

struct CoordinatesIter<'a, 'data> {
    coords: &'a Coordinates<'data>,
    index: usize,
}

impl<'data> Coordinates<'data> {
    pub fn iter(&self) -> CoordinatesIter<'_, 'data> {
        CoordinatesIter {
            coords: self,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Coordinates::Tuple(coords) => coords.0.len(),
            Coordinates::Array(coords) => coords.len(),
        }
    }
}

impl Iterator for CoordinatesIter<'_, '_> {
    type Item = F2Dot14;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.coords.len() {
            return None;
        }

        let index = self.index;
        self.index += 1;
        match self.coords {
            Coordinates::Tuple(coords) => Some(coords.0.get_item(index)),
            Coordinates::Array(coords) => Some(coords[index]),
        }
    }
}

impl<'a> Glyph<'a> {
    /// Apply glyph variation to the supplied glyph according to the variation instance
    /// `user_instance`.
    pub(crate) fn apply_variations(
        &mut self,
        glyph_index: u16,
        instance: &OwnedTuple,
        gvar: &GvarTable<'a>,
        hmtx: &HmtxTable<'a>,
        vmtx: Option<&HmtxTable<'a>>,
        os2: Option<&Os2>,
        hhea: &HheaTable,
    ) -> Result<(), ParseError> {
        let Some(deltas) = glyph_deltas(self, glyph_index, instance, gvar)? else {
            // The glyph has no deltas but we still need to populate the phantom points
            let phantom_points =
                calculate_phantom_points(glyph_index, self.bounding_box(), hmtx, vmtx, os2, hhea)?;
            self.set_phantom_points(phantom_points);
            return Ok(())
        };

        match self {
            Glyph::Empty(empty) => {
                let mut phantom_points =
                    calculate_phantom_points(glyph_index, None, hmtx, vmtx, os2, hhea)?;
                apply_phantom_point_deltas(&mut phantom_points, &deltas);
                empty.phantom_points = Some(phantom_points);
                Ok(())
            }
            Glyph::Simple(simple_glyph) => {
                // Calculate the phantom points before variations are applied
                let mut phantom_points = calculate_phantom_points(
                    glyph_index,
                    Some(simple_glyph.bounding_box),
                    hmtx,
                    vmtx,
                    os2,
                    hhea,
                )?;

                // Apply the deltas to the coordinates of the glyph and calculate the updated
                // bounding box as we go.
                let first_point = simple_glyph.coordinates[0].1;
                let mut bbox = BoundingBox {
                    x_min: first_point.0,
                    x_max: first_point.0,
                    y_min: first_point.1,
                    y_max: first_point.1,
                };
                simple_glyph
                    .coordinates
                    .iter_mut()
                    .zip(deltas.iter().copied())
                    .for_each(|((_flag, point), delta)| {
                        // NOTE(cast): Since Rust 1.45.0 floating point casts like these are
                        // saturating casts.
                        // https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#fixing-unsoundness-in-casts
                        point.0 = (point.0 as f32 + delta.x()).round() as i16;
                        point.1 = (point.1 as f32 + delta.y()).round() as i16;
                        bbox.add(*point)
                    });
                simple_glyph.bounding_box = bbox;

                // Apply deltas to the phantom points of the glyph
                apply_phantom_point_deltas(
                    &mut phantom_points,
                    &deltas[simple_glyph.coordinates.len()..],
                );
                simple_glyph.phantom_points = Some(Box::new(phantom_points));

                // TODO: Update flag 1 in head?
                Ok(())
            }
            Glyph::Composite(composite) => {
                // Calculate the phantom points before variations are applied
                let mut phantom_points = calculate_phantom_points(
                    glyph_index,
                    Some(composite.bounding_box),
                    hmtx,
                    vmtx,
                    os2,
                    hhea,
                )?;

                // Use the deltas to reposition the sub-glyphs of the composite glyph
                composite
                    .glyphs
                    .iter_mut()
                    .zip(deltas.iter().copied())
                    .for_each(|(composite_glyph, delta)| {
                        add_composite_glyph_delta(composite_glyph, delta)
                    });

                // Apply deltas to phantom  points
                apply_phantom_point_deltas(&mut phantom_points, &deltas[composite.glyphs.len()..]);
                composite.phantom_points = Some(Box::new(phantom_points));

                Ok(())
            }
        }
    }

    /// Calculate the bounding box from the points of this glyph.
    ///
    /// For simple glyphs this just returns the bounding box of the glyph. For composite glyphs the
    /// sub-glyphs are traversed to calculate the bounding box that contains them all.
    pub(crate) fn calculate_bounding_box(&self, glyf: &GlyfTable<'a>) -> Result<RectF, ParseError> {
        match self {
            Glyph::Empty(glyph) => glyph.calculate_bounding_box(),
            Glyph::Simple(glyph) => glyph.calculate_bounding_box(),
            Glyph::Composite(glyph) => glyph.calculate_bounding_box(glyf),
        }
    }

    fn set_phantom_points(&mut self, phantom_points: [Point; 4]) {
        match self {
            Glyph::Empty(empty) => empty.phantom_points = Some(phantom_points),
            Glyph::Simple(simple) => simple.phantom_points = Some(Box::new(phantom_points)),
            Glyph::Composite(composite) => {
                composite.phantom_points = Some(Box::new(phantom_points))
            }
        }
    }
}

fn apply_phantom_point_deltas(phantom_points: &mut PhantomPoints, deltas: &[Vector2F]) {
    phantom_points
        .iter_mut()
        .zip(deltas.iter().copied())
        .for_each(|(point, delta)| {
            // NOTE(cast): saturating
            point.0 = (point.0 as f32 + delta.x()).round() as i16;
            point.1 = (point.1 as f32 + delta.y()).round() as i16;
        });
}

impl EmptyGlyph {
    pub(crate) fn calculate_bounding_box(&self) -> Result<RectF, ParseError> {
        Ok(RectF::default())
    }
}

impl<'a> SimpleGlyph<'a> {
    pub(crate) fn calculate_bounding_box(&self) -> Result<RectF, ParseError> {
        Ok(RectF::from_points(
            vec2f(
                self.bounding_box.x_min as f32,
                self.bounding_box.y_min as f32,
            ),
            vec2f(
                self.bounding_box.x_max as f32,
                self.bounding_box.y_max as f32,
            ),
        ))
    }
}

impl<'a> CompositeGlyph<'a> {
    pub(crate) fn calculate_bounding_box(&self, glyf: &GlyfTable<'a>) -> Result<RectF, ParseError> {
        let mut bbox: Option<RectF> = None;
        for child in &self.glyphs {
            let record: &GlyfRecord<'_> = glyf
                .records
                .get(usize::from(child.glyph_index))
                .ok_or(ParseError::BadIndex)?;
            let GlyfRecord::Parsed(child_glyph) = record else {
                panic!("glyph is not parsed");
            };
            let mut child_bbox = child_glyph.calculate_bounding_box(glyf)?;

            // Scale the bbox
            let offset = Vector2F::new(
                i32::from(child.argument1) as f32,
                i32::from(child.argument2) as f32,
            );
            match child.scale {
                Some(scale) => {
                    let scale = Matrix2x2F::from(scale);
                    match child.flags.component_offsets() {
                        // translate, then scale
                        ComponentOffsets::Scaled => {
                            let transform = Transform2F {
                                matrix: scale,
                                vector: Vector2F::zero(),
                            };
                            child_bbox = transform * (child_bbox + offset);
                        }
                        // scale, then translate - this the default for Transform2F
                        ComponentOffsets::Unscaled => {
                            let transform = Transform2F {
                                matrix: scale,
                                vector: offset,
                            };
                            child_bbox = transform * child_bbox;
                        }
                    }
                }
                // just translate
                None => child_bbox = child_bbox + offset,
            }

            // combine the scaled bbox with the overall bbox
            match bbox.as_mut() {
                Some(rect) => *rect = rect.union_rect(child_bbox),
                None => bbox = Some(child_bbox),
            }
        }
        Ok(bbox.unwrap_or_default())
    }
}

fn add_composite_glyph_delta(composite_glyph: &mut CompositeGlyphComponent, delta: Vector2F) {
    // > if ARGS_ARE_XY_VALUES (bit 1) is set, then X and Y offsets are used; if that bit is clear,
    // > then point numbers are used. If the position of a component is represented using X and Y
    // > offsets — the ARGS_ARE_XY_VALUES flag is set — then adjustment deltas can be applied to
    // > those offsets. However, if the position of a component is represented using point numbers —
    // > the ARGS_ARE_XY_VALUES flag is not set — then adjustment deltas have no effect on that
    // > component and should not be specified.
    //
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs
    if composite_glyph.flags.args_are_xy_values() {
        composite_glyph.argument1 = add_delta(composite_glyph.argument1, delta.x());
        composite_glyph.argument2 = add_delta(composite_glyph.argument2, delta.y());
        // add_delta always uses I16 so ensure the ARG_1_AND_2_ARE_WORDS flag is set
        composite_glyph.flags |= CompositeGlyphFlag::ARG_1_AND_2_ARE_WORDS;
    }
}

fn add_delta(arg: CompositeGlyphArgument, delta: f32) -> CompositeGlyphArgument {
    // If ARGS_ARE_XY_VALUES is set we should only get I8 or I16 values in practice but handle them
    // all nonetheless.
    let adjusted = match arg {
        CompositeGlyphArgument::U8(val) => val as f32 + delta,
        CompositeGlyphArgument::I8(val) => val as f32 + delta,
        CompositeGlyphArgument::U16(val) => val as f32 + delta,
        CompositeGlyphArgument::I16(val) => val as f32 + delta,
    };
    let adjusted = adjusted.round();

    // TODO: use smaller types when appropriate
    CompositeGlyphArgument::I16(adjusted as i16)
}

/// Calculate the point deltas for the supplied glyph according to the variation instance `instance`.
///
/// If deltas are present the resulting vector will include a delta for each coordinate in the
/// glyph, including the four phantom points.
///
/// If the glyph has no variation data then `Ok(None)` is returned.
fn glyph_deltas(
    glyph: &Glyph<'_>,
    glyph_index: u16,
    instance: &OwnedTuple,
    gvar: &GvarTable<'_>,
) -> Result<Option<Vec<Vector2F>>, ParseError> {
    let num_points = NumPoints::new(glyph.number_of_points()?);
    let Some(variations) = gvar.glyph_variation_data(glyph_index, num_points)? else {
        return Ok(None)
    };

    let applicable = determine_applicable(gvar, &instance, &variations);

    // Now the deltas need to be calculated for each point.
    // The delta is multiplied by the scalar. The sum of deltas is applied to the default position
    let mut final_deltas = vec![Vector2F::zero(); usize::safe_from(num_points.get())];
    let mut region_deltas = vec![Vector2F::zero(); usize::safe_from(num_points.get())];
    for (scale, region) in applicable {
        let variation_data =
            region.variation_data(num_points, variations.shared_point_numbers())?;
        // This is the output for this region, by the end every point needs to have a delta assigned.
        // Either explicitly or inferred. This buffer is reused between regions so we re-fill it
        // with zeros for each new region.
        region_deltas.fill(Vector2F::zero());

        // This maps point numbers to deltas, in order. It allows direct lookup of deltas for a
        // point as well as navigating between explicit points.
        let explicit_deltas = variation_data.iter().collect::<BTreeMap<_, _>>();

        // Fill in the explicit deltas
        for (number, delta) in &explicit_deltas {
            let region_delta = region_deltas
                .get_mut(usize::safe_from(*number))
                .ok_or(ParseError::BadIndex)?;
            *region_delta = Vector2F::new(delta.0 as f32, delta.1 as f32);
        }

        // > Calculation of inferred deltas is done for a given glyph and a given region on a
        // > contour-by-contour basis.
        // >
        // > For a given contour, if the point number list does not include any of the points in
        // > that contour, then none of the points in the contour are affected and no inferred deltas
        // > need to be computed.
        // >
        // > If the point number list includes some but not all of the points in a given contour,
        // > then inferred deltas must be derived for the points that were not included in the point
        // > number list, as follows.

        // Only need to do this for simple glyphs
        match glyph {
            Glyph::Simple(simple_glyph) => {
                // Deltas need to be inferred if not all points were assigned explicit deltas
                if explicit_deltas.len() != usize::safe_from(num_points.get()) {
                    infer_unreferenced_points(&mut region_deltas, &explicit_deltas, simple_glyph)?;
                }
            }
            _ => {}
        }

        // Scale and accumulate the deltas from this variation region onto the final deltas
        final_deltas
            .iter_mut()
            .zip(region_deltas.iter().copied())
            .for_each(|(out, delta)| {
                *out += delta * scale
            })
    }

    // Now all the deltas need to be applied to the glyph points
    Ok(Some(final_deltas))
}

fn infer_unreferenced_points(
    deltas: &mut [Vector2F],
    raw_deltas: &BTreeMap<u32, (i16, i16)>,
    simple_glyph: &SimpleGlyph<'_>,
) -> Result<(), ParseError> {
    // Iterate over the contours of the glyph and ensure that all points of the contour have a delta
    let mut begin = 0;
    for end in simple_glyph.end_pts_of_contours.iter().copied() {
        let start = begin;
        let end = u32::from(end);
        begin = end + 1;
        let range = start..=end;
        let range_len = usize::safe_from(end.saturating_sub(start));

        let explicit_count = raw_deltas.range(range.clone()).count();
        match explicit_count {
            0 => {
                // No points in this contour were referenced; no inferred deltas need to
                // be computed.
                continue;
            }
            1 => {
                // If exactly one point from the contour is referenced in the point number list,
                // then every point in that contour uses the same X and Y delta values as that point.
                // Find the one referenced point and use it to update the others
                // NOTE(unwrap): Safe as we confirmed we have one delta to get into this block
                let (_referenced_point_number, reference_delta) = raw_deltas
                    .range(range.clone())
                    .next()
                    .map(|(n, (x, y))| (*n, Vector2F::new(*x as f32, *y as f32)))
                    .unwrap();
                // Get the delta for this point
                let usize_range = usize::safe_from(*range.start())..=usize::safe_from(*range.end());
                // Set all the deltas in this contour to `reference_delta`
                deltas[usize_range].fill(reference_delta);
                continue;
            }
            n if n == range_len => {
                // All points in this contour were referenced; no inferred deltas need to
                // be computed.
                continue;
            }
            _ => {
                // If the point number list includes some but not all of the points in a given
                // contour, then inferred deltas must be derived for the points that were not
                // included in the point number list.
                infer_contour(&range, deltas, raw_deltas, simple_glyph)?;
            }
        }
    }
    Ok(())
}

fn infer_contour(
    contour_range: &RangeInclusive<u32>,
    deltas: &mut [Vector2F],
    explicit_deltas: &BTreeMap<u32, (i16, i16)>,
    simple_glyph: &SimpleGlyph<'_>,
) -> Result<(), ParseError> {
    for target in contour_range.clone() {
        if explicit_deltas.contains_key(&target) {
            continue;
        }

        // This is an unreferenced point
        //
        // > First, for any un-referenced point, identify the nearest points before and after, in
        // > point number order, that are referenced. Note that the same referenced points will be
        // > used for calculating both X and Y inferred deltas. If there is no lower point number
        // > from that contour that was referenced, then the highest, referenced point number from
        // > that contour is used. Similarly, if no higher point number from that contour was
        // > referenced, then the lowest, referenced point number is used.

        // NOTE(unwrap): Due to checks above regarding the number of referenced points we should
        // always find a next/prev point
        let next = explicit_deltas
            .range(target..=*contour_range.end())
            .chain(explicit_deltas.range(*contour_range.start()..target))
            .next()
            .unwrap();
        let prev = explicit_deltas
            .range(target..=*contour_range.end())
            .chain(explicit_deltas.range(*contour_range.start()..target))
            .rev()
            .next()
            .unwrap();

        let target = usize::safe_from(target);
        deltas[target] = infer_delta(target, prev, next, &simple_glyph.coordinates)?;
    }
    Ok(())
}

// > Once the adjacent, referenced points are identified, then inferred-delta calculation is done
// > separately for X and Y directions.
fn infer_delta(
    target: usize,
    (prev_number, prev_delta): (&u32, &(i16, i16)),
    (next_number, next_delta): (&u32, &(i16, i16)),
    coordinates: &[(SimpleGlyphFlag, Point)],
) -> Result<Vector2F, ParseError> {
    // https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#inferred-deltas-for-un-referenced-point-numbers
    let prev_coord = coordinates
        .get(usize::safe_from(*prev_number))
        .ok_or(ParseError::BadIndex)?
        .1;
    let target_coord = coordinates.get(target).ok_or(ParseError::BadIndex)?.1;
    let next_coord = coordinates
        .get(usize::safe_from(*next_number))
        .ok_or(ParseError::BadIndex)?
        .1;

    let delta_x = do_infer(
        prev_coord.0,
        target_coord.0,
        next_coord.0,
        prev_delta.0,
        next_delta.0,
    );
    let delta_y = do_infer(
        prev_coord.1,
        target_coord.1,
        next_coord.1,
        prev_delta.1,
        next_delta.1,
    );
    Ok(Vector2F::new(delta_x, delta_y))
}

// > The (X or Y) grid coordinate values of the adjacent, referenced points are compared. If
// > these coordinates are the same, then the delta values for the adjacent points are compared: if
// > the delta values are the same, then this value is used as the inferred delta for the target,
// > un-referenced point. If the delta values are different, then the inferred delta for the target
// > point is zero.
fn do_infer(
    prev_coord: i16,
    target_coord: i16,
    next_coord: i16,
    prev_delta: i16,
    next_delta: i16,
) -> f32 {
    let prev_delta = prev_delta;
    let next_delta = next_delta;
    if prev_coord == next_coord {
        if prev_delta == next_delta {
            prev_delta as f32
        } else {
            0.
        }
    } else {
        // > But if the coordinate of the target point is not between the coordinates of the
        // > adjacent points, then the inferred delta is the delta for whichever of the adjacent
        // > points is closer in the given direction.
        if target_coord <= prev_coord.min(next_coord) {
            if prev_coord < next_coord {
                prev_delta as f32
            } else {
                next_delta as f32
            }
        } else if target_coord >= prev_coord.max(next_coord) {
            if prev_coord > next_coord {
                prev_delta as f32
            } else {
                next_delta as f32
            }
        } else {
            // > If the coordinate of the target point is between the coordinates of the adjacent
            // > points, then a delta is interpolated

            // > Note: The logical flow of the algorithm to this point implies that the coordinates
            // > of the two adjacent points are different. This avoids a division by zero in the
            // > following calculations that would otherwise occur.
            let proportion =
                (target_coord as f32 - prev_coord as f32) / (next_coord as f32 - prev_coord as f32);
            (1. - proportion) * prev_delta as f32 + proportion * next_delta as f32
        }
    }
}

fn determine_applicable<'a, 'data>(
    gvar: &'a GvarTable<'data>,
    instance: &'a OwnedTuple,
    variations: &'a TupleVariationStore<'data, Gvar>,
) -> impl Iterator<Item = (f32, &'a TupleVariationHeader<'data, Gvar>)> + 'a {
    // Ok, now we have our tuple we need to get the relevant glyph variation records
    //
    // > The tuple variation headers within the selected glyph variation data table will each
    // > specify a particular region of applicability within the font’s variation space. These will
    // > be compared with the coordinates for the selected variation instance to determine which of
    // > the tuple-variation data tables are applicable, and to calculate a scalar value for each.
    // > These comparisons and scalar calculations are done using normalized-scale coordinate values.
    // >
    // > The tuple variation headers within the selected glyph variation data table will each
    // > specify a particular region of applicability within the font’s variation space. These will
    // > be compared with the coordinates for the selected variation instance to determine which of
    // > the tuple-variation data tables are applicable, and to calculate a scalar value for each.
    // > These comparisons and scalar calculations are done using normalized-scale coordinate
    // > values.For each of the tuple-variation data tables that are applicable, the point number and
    // > delta data will be unpacked and processed. The data for applicable regions can be processed
    // > in any order. Derived delta values will correspond to particular point numbers derived from
    // > the packed point number data. For a given point number, the computed scalar is applied to
    // > the X coordinate and Y coordinate deltas as a coefficient, and then resulting delta
    // > adjustments applied to the X and Y coordinates of the point.

    // Determine which ones are applicable and return the scalar value for each one
    variations.headers().filter_map(move |header| {
        // https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview#algorithm-for-interpolation-of-instance-values
        let peak_coords = header.peak_tuple(gvar).ok()?;
        let (start_coords, end_coords) = match header.intermediate_region() {
            // NOTE(clone): Cheap as Tuple just contains ReadArray
            Some((start, end)) => (
                Coordinates::Tuple(start.clone()),
                Coordinates::Tuple(end.clone()),
            ),
            None => {
                let mut start_coords = tiny_vec!();
                let mut end_coords = tiny_vec!();
                for peak in peak_coords.0.iter() {
                    match peak.raw_value().signum() {
                        // region is from peak to zero
                        -1 => {
                            start_coords.push(peak);
                            end_coords.push(F2Dot14::from(0));
                        }
                        // When a delta is provided for a region defined by n-tuples that have
                        // a peak value of 0 for some axis, then that axis does not factor into
                        // scalar calculations.
                        0 => {
                            start_coords.push(peak);
                            end_coords.push(peak);
                        }
                        // region is from zero to peak
                        1 => {
                            start_coords.push(F2Dot14::from(0));
                            end_coords.push(peak);
                        }
                        _ => unreachable!("unknown value from signum"),
                    }
                }
                (
                    Coordinates::Array(start_coords),
                    Coordinates::Array(end_coords),
                )
            }
        };

        // Now determine the scalar:
        //
        // > In calculation of scalars (S, AS) and of interpolated values (scaledDelta,
        // > netAdjustment, interpolatedValue), at least 16 fractional bits of precision should
        // > be maintained.
        let scalar = start_coords
            .iter()
            .zip(end_coords.iter())
            .zip(instance.iter().copied())
            .zip(peak_coords.0.iter())
            .map(|(((start, end), instance), peak)| {
                // If peak is zero or not contained by the region of applicability then it does not
                if peak == F2Dot14::from(0) {
                    // If the peak is zero for some axis, then ignore the axis.
                    1.
                } else if (start..=end).contains(&instance) {
                    // The region is applicable: calculate a per-axis scalar as a proportion
                    // of the proximity of the instance to the peak within the region.
                    if instance == peak {
                        1.
                    } else if instance < peak {
                        (f32::from(instance) - f32::from(start))
                            / (f32::from(peak) - f32::from(start))
                    } else {
                        // instance > peak
                        (f32::from(end) - f32::from(instance)) / (f32::from(end) - f32::from(peak))
                    }
                } else {
                    // If the instance coordinate is out of range for some axis, then the
                    // region and its associated deltas are not applicable.
                    0.
                }
            })
            .fold(1., |scalar, axis_scalar| scalar * axis_scalar);

        (scalar != 0.).then(|| (scalar, header))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::read::ReadScope;
    use crate::error::ReadWriteError;
    use crate::font_data::FontData;
    use crate::tables::glyf::GlyfTable;
    use crate::tables::loca::LocaTable;
    use crate::tables::variable_fonts::avar::AvarTable;
    use crate::tables::variable_fonts::fvar::FvarTable;
    use crate::tables::{Fixed, FontTableProvider, HeadTable, MaxpTable, NameTable};
    use crate::tests::read_fixture;
    use crate::{assert_close, tag};

    #[test]
    fn apply_variations() -> Result<(), ReadWriteError> {
        let buffer = read_fixture("tests/fonts/opentype/NotoSans-VF.abc.ttf");
        let scope = ReadScope::new(&buffer);
        let font_file = scope.read::<FontData<'_>>()?;
        let provider = font_file.table_provider(0)?;
        let head = ReadScope::new(&provider.read_table_data(tag::HEAD)?).read::<HeadTable>()?;
        let maxp = ReadScope::new(&provider.read_table_data(tag::MAXP)?).read::<MaxpTable>()?;
        let loca_data = provider.read_table_data(tag::LOCA)?;
        let loca = ReadScope::new(&loca_data)
            .read_dep::<LocaTable<'_>>((usize::from(maxp.num_glyphs), head.index_to_loc_format))?;
        let glyf_data = provider.read_table_data(tag::GLYF)?;
        let mut glyf = ReadScope::new(&glyf_data).read_dep::<GlyfTable<'_>>(&loca)?;
        let fvar_data = provider
            .read_table_data(tag::FVAR)
            .expect("unable to read fvar table data");
        let fvar = ReadScope::new(&fvar_data).read::<FvarTable<'_>>().unwrap();
        let avar_data = provider.table_data(tag::AVAR)?;
        let avar = avar_data
            .as_ref()
            .map(|avar_data| ReadScope::new(avar_data).read::<AvarTable<'_>>())
            .transpose()?;
        let gvar_data = provider.read_table_data(tag::GVAR)?;
        let gvar = ReadScope::new(&gvar_data).read::<GvarTable<'_>>().unwrap();
        let name_table_data = provider
            .read_table_data(tag::NAME)
            .expect("unable to read name table data");
        let name_table = ReadScope::new(&name_table_data)
            .read::<NameTable<'_>>()
            .unwrap();

        // Pick a glyph. Glyph 2 is 'b'
        let glyph_index = 2u16;
        let glyph = glyf.get_parsed_glyph(glyph_index)?;

        // Pick an instance
        let mut instance = None;
        for inst in fvar.instances() {
            let inst = inst?;
            let subfamily = name_table.english_string_for_id(inst.subfamily_name_id);
            if subfamily.as_deref() == Some("Display Condensed Thin") {
                // - wght = min: 100, max: 900, default: 400
                // - wdth = min: 62.5, max: 100, default: 100
                // - CTGR = min: 0, max: 100, default: 0
                //
                // Coordinates: [100.0, 76.24969, 100.0]
                instance = Some(inst);
                break;
            }
        }
        let user_instance = instance.unwrap();
        let instance = fvar
            .normalize(user_instance.coordinates.iter(), avar.as_ref())
            .unwrap();

        let varied = glyph_deltas(glyph, glyph_index, &instance, &gvar)?
            .expect("there should be glyph deltas");

        // These values were obtained by feeding the same parameters into
        // [skrifa](https://docs.rs/crate/skrifa/0.11.0).
        let expected_deltas = &[
            (-73.86737060546875, -80.800537109375),
            (-73.86737060546875, -65.200439453125),
            (-71.24325561523438, -51.0),
            (-70.29388427734375, -50.599853515625),
            (-73.1673583984375, -50.599853515625),
            (-84.32525634765625, -30.09991455078125),
            (-88.37908935546875, -7.4000244140625),
            (-95.0008544921875, -7.4000244140625),
            (-114.1365966796875, -7.4000244140625),
            (-153.50152587890625, -5.29998779296875),
            (-153.50152587890625, 0.10003662109375),
            (-153.50152587890625, 4.1998291015625),
            (-135.71871948242188, 3.89984130859375),
            (-112.06466674804688, 0.0),
            (-102.67691040039063, 0.0),
            (-92.93865966796875, 0.0),
            (-78.8035888671875, 15.49993896484375),
            (-71.71092224121094, 31.09991455078125),
            (-66.50018310546875, 31.09991455078125),
            (-53.50018310546875, 0.0),
            (-11.8001708984375, 0.0),
            (-11.8001708984375, 0.0),
            (-73.86737060546875, 0.0),
            (-80.65133666992188, 40.5999755859375),
            (-72.0006103515625, 40.5999755859375),
            (-70.2852783203125, 27.50006103515625),
            (-73.50018310546875, 14.30023193359375),
            (-73.50018310546875, 13.70037841796875),
            (-73.50018310546875, -23.8001708984375),
            (-73.50018310546875, -41.50018310546875),
            (-66.0003662109375, -47.29998779296875),
            (-90.000732421875, -47.29998779296875),
            (-93.10113525390625, -47.29998779296875),
            (-90.10150146484375, -27.90008544921875),
            (-90.10150146484375, -0.89996337890625),
            (-90.10150146484375, 19.0),
            (-84.10113525390625, 40.5999755859375),
            (0.0, 0.0),
            (0.0, 0.0),
            (0.0, 0.0),
            (0.0, 0.0),
        ];
        assert_eq!(varied.len(), expected_deltas.len());
        // Ignore phantom points at end
        for (expected, actual) in expected_deltas[..expected_deltas.len() - 4]
            .iter()
            .copied()
            .zip(varied.iter().copied())
        {
            assert_close!(actual.x(), expected.0, 0.005);
            assert_close!(actual.y(), expected.1, 0.005);
        }

        Ok(())
    }

    #[test]
    #[cfg(feature = "prince")]
    fn apply_skia_variations_simple_glyph() -> Result<(), ReadWriteError> {
        let buffer = read_fixture("../../../tests/data/fonts/Skia.subset.ttf");
        let scope = ReadScope::new(&buffer);
        let font_file = scope.read::<FontData<'_>>()?;
        let provider = font_file.table_provider(0)?;
        let head = ReadScope::new(&provider.read_table_data(tag::HEAD)?).read::<HeadTable>()?;
        let maxp = ReadScope::new(&provider.read_table_data(tag::MAXP)?).read::<MaxpTable>()?;
        let loca_data = provider.read_table_data(tag::LOCA)?;
        let loca = ReadScope::new(&loca_data)
            .read_dep::<LocaTable<'_>>((usize::from(maxp.num_glyphs), head.index_to_loc_format))?;
        let glyf_data = provider.read_table_data(tag::GLYF)?;
        let mut glyf = ReadScope::new(&glyf_data).read_dep::<GlyfTable<'_>>(&loca)?;
        let fvar_data = provider
            .read_table_data(tag::FVAR)
            .expect("unable to read fvar table data");
        let fvar = ReadScope::new(&fvar_data).read::<FvarTable<'_>>().unwrap();
        let avar_data = provider.table_data(tag::AVAR)?;
        let avar = avar_data
            .as_ref()
            .map(|avar_data| ReadScope::new(avar_data).read::<AvarTable<'_>>())
            .transpose()?;
        let gvar_data = provider.read_table_data(tag::GVAR)?;
        let gvar = ReadScope::new(&gvar_data).read::<GvarTable<'_>>().unwrap();

        // Pick a glyph. Glyph 45 is '-', this is chosen to replicate the example in the spec:
        // https://learn.microsoft.com/en-us/typography/opentype/spec/otvaroverview#interpolation-example
        let glyph_index = 45u16;
        let glyph = glyf.get_parsed_glyph(glyph_index)?;

        // (0.2, 0.7) — a slight weight increase and a large width increase. The example gives
        // these are normalised values but we need to supply user values
        let user_instance = &[Fixed::from(1.44), Fixed::from(1.21)];
        let instance = fvar
            .normalize(user_instance.iter().copied(), avar.as_ref())
            .unwrap();

        let varied = glyph_deltas(glyph, glyph_index, &instance, &gvar)?
            .expect("there should be glyph deltas");

        let expected_deltas = &[
            (162.3, -28.4),
            (8.8, -28.4),
            (8.8, 36.4),
            (162.3, 36.4),
            (0., 0.),
            (172.7, 0.),
        ];
        for (expected, actual) in expected_deltas.iter().copied().zip(varied.iter().copied()) {
            assert_close!(actual.x(), expected.0, 0.005);
            assert_close!(actual.y(), expected.1, 0.005);
        }

        Ok(())
    }

    #[test]
    #[cfg(feature = "prince")]
    fn apply_skia_variations_composite_glyph() -> Result<(), ReadWriteError> {
        let buffer = read_fixture("../../../tests/data/fonts/Skia.subset.ttf");
        let scope = ReadScope::new(&buffer);
        let font_file = scope.read::<FontData<'_>>()?;
        let provider = font_file.table_provider(0)?;
        let head = ReadScope::new(&provider.read_table_data(tag::HEAD)?).read::<HeadTable>()?;
        let maxp = ReadScope::new(&provider.read_table_data(tag::MAXP)?).read::<MaxpTable>()?;
        let loca_data = provider.read_table_data(tag::LOCA)?;
        let loca = ReadScope::new(&loca_data)
            .read_dep::<LocaTable<'_>>((usize::from(maxp.num_glyphs), head.index_to_loc_format))?;
        let glyf_data = provider.read_table_data(tag::GLYF)?;
        let mut glyf = ReadScope::new(&glyf_data).read_dep::<GlyfTable<'_>>(&loca)?;
        let fvar_data = provider
            .read_table_data(tag::FVAR)
            .expect("unable to read fvar table data");
        let fvar = ReadScope::new(&fvar_data).read::<FvarTable<'_>>().unwrap();
        let avar_data = provider.table_data(tag::AVAR)?;
        let avar = avar_data
            .as_ref()
            .map(|avar_data| ReadScope::new(avar_data).read::<AvarTable<'_>>())
            .transpose()?;
        let gvar_data = provider.read_table_data(tag::GVAR)?;
        let gvar = ReadScope::new(&gvar_data).read::<GvarTable<'_>>().unwrap();

        // Pick a glyph. Glyph 128 of the Skia font, which is the glyph for “Ä”. The glyph entry
        // has two component entries, both with ARGS_ARE_XY_VALUES set.
        // https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#point-numbers-and-processing-for-composite-glyphs
        let glyph_index = 128u16;
        let glyph = glyf.get_parsed_glyph(glyph_index)?;

        // (0.2, 0.7) — a slight weight increase and a large width increase. The example gives
        // these are normalised values but we need to supply user values
        let user_instance = &[Fixed::from(1.44), Fixed::from(1.21)];
        let instance = fvar
            .normalize(user_instance.iter().copied(), avar.as_ref())
            .unwrap();

        let varied = glyph_deltas(glyph, glyph_index, &instance, &gvar)?
            .expect("there should be glyph deltas");

        // The example in the spec appears to be wrong, thus the final values don't match. R3 in the
        // example is supposed to correspond to the region (weight, width) of (1, 1) however they
        // seem to have used the values from the (-1, 1) region. To try to rule out the example
        // using a different version of the font I confirmed this with versions of Skia from Mac OS
        // 7.6.1 and macOS 11.7.1 (Big Sur) and they were the same.
        //
        // Tracked by: https://github.com/MicrosoftDocs/typography-issues/issues/1067
        let r1_scale = 0.2;
        let r2_scale = 0.7;
        let r3_scale = 0.14;
        let expected_deltas = &[
            (0., 0.),
            ((r1_scale * 69.) + (r2_scale * 53.) + (r3_scale * -8.), 0.),
            ((r1_scale * 58.) + (r2_scale * 38.) + (r3_scale * -30.), 0.),
            ((r1_scale * 145.) + (r2_scale * 351.) + (r3_scale * 0.), 0.),
        ];
        for (expected, actual) in expected_deltas.iter().copied().zip(varied.iter().copied()) {
            assert_close!(actual.x(), expected.0, 0.01);
            assert_close!(actual.y(), expected.1, 0.01);
        }

        Ok(())
    }
}
