use crate::tag;

// https://learn.microsoft.com/en-us/typography/opentype/spec/featurelist
pub(crate) const REGISTERED_FEATURES: &[(u32, &str)] = &[
    (tag!(b"aalt"), "Access All Alternates"),
    (tag!(b"abvf"), "Above-base Forms"),
    (tag!(b"abvm"), "Above-base Mark Positioning"),
    (tag!(b"abvs"), "Above-base Substitutions"),
    (tag!(b"afrc"), "Alternative Fractions"),
    (tag!(b"akhn"), "Akhand"),
    (tag!(b"blwf"), "Below-base Forms"),
    (tag!(b"blwm"), "Below-base Mark Positioning"),
    (tag!(b"blws"), "Below-base Substitutions"),
    (tag!(b"calt"), "Contextual Alternates"),
    (tag!(b"case"), "Case-Sensitive Forms"),
    (tag!(b"ccmp"), "Glyph Composition / Decomposition"),
    (tag!(b"cfar"), "Conjunct Form After Ro"),
    (tag!(b"chws"), "Contextual Half-width Spacing"),
    (tag!(b"cjct"), "Conjunct Forms"),
    (tag!(b"clig"), "Contextual Ligatures"),
    (tag!(b"cpct"), "Centered CJK Punctuation"),
    (tag!(b"cpsp"), "Capital Spacing"),
    (tag!(b"cswh"), "Contextual Swash"),
    (tag!(b"curs"), "Cursive Positioning"),
    // 'cv01' – 'cv99' - Character Variants are handled in the code
    (tag!(b"c2pc"), "Petite Capitals From Capitals"),
    (tag!(b"c2sc"), "Small Capitals From Capitals"),
    (tag!(b"dist"), "Distances"),
    (tag!(b"dlig"), "Discretionary Ligatures"),
    (tag!(b"dnom"), "Denominators"),
    (tag!(b"dtls"), "Dotless Forms"),
    (tag!(b"expt"), "Expert Forms"),
    (tag!(b"falt"), "Final Glyph on Line Alternates"),
    (tag!(b"fin2"), "Terminal Forms #2"),
    (tag!(b"fin3"), "Terminal Forms #3"),
    (tag!(b"fina"), "Terminal Forms"),
    (tag!(b"flac"), "Flattened accent forms"),
    (tag!(b"frac"), "Fractions"),
    (tag!(b"fwid"), "Full Widths"),
    (tag!(b"half"), "Half Forms"),
    (tag!(b"haln"), "Halant Forms"),
    (tag!(b"halt"), "Alternate Half Widths"),
    (tag!(b"hist"), "Historical Forms"),
    (tag!(b"hkna"), "Horizontal Kana Alternates"),
    (tag!(b"hlig"), "Historical Ligatures"),
    (tag!(b"hngl"), "Hangul"),
    (
        tag!(b"hojo"),
        "Hojo Kanji Forms (JIS X 0212-1990 Kanji Forms)",
    ),
    (tag!(b"hwid"), "Half Widths"),
    (tag!(b"init"), "Initial Forms"),
    (tag!(b"isol"), "Isolated Forms"),
    (tag!(b"ital"), "Italics"),
    (tag!(b"jalt"), "Justification Alternates"),
    (tag!(b"jp78"), "JIS78 Forms"),
    (tag!(b"jp83"), "JIS83 Forms"),
    (tag!(b"jp90"), "JIS90 Forms"),
    (tag!(b"jp04"), "JIS2004 Forms"),
    (tag!(b"kern"), "Kerning"),
    (tag!(b"lfbd"), "Left Bounds"),
    (tag!(b"liga"), "Standard Ligatures"),
    (tag!(b"ljmo"), "Leading Jamo Forms"),
    (tag!(b"lnum"), "Lining Figures"),
    (tag!(b"locl"), "Localized Forms"),
    (tag!(b"ltra"), "Left-to-right alternates"),
    (tag!(b"ltrm"), "Left-to-right mirrored forms"),
    (tag!(b"mark"), "Mark Positioning"),
    (tag!(b"med2"), "Medial Forms #2"),
    (tag!(b"medi"), "Medial Forms"),
    (tag!(b"mgrk"), "Mathematical Greek"),
    (tag!(b"mkmk"), "Mark to Mark Positioning"),
    (tag!(b"mset"), "Mark Positioning via Substitution"),
    (tag!(b"nalt"), "Alternate Annotation Forms"),
    (tag!(b"nlck"), "NLC Kanji Forms"),
    (tag!(b"nukt"), "Nukta Forms"),
    (tag!(b"numr"), "Numerators"),
    (tag!(b"onum"), "Oldstyle Figures"),
    (tag!(b"opbd"), "Optical Bounds"),
    (tag!(b"ordn"), "Ordinals"),
    (tag!(b"ornm"), "Ornaments"),
    (tag!(b"palt"), "Proportional Alternate Widths"),
    (tag!(b"pcap"), "Petite Capitals"),
    (tag!(b"pkna"), "Proportional Kana"),
    (tag!(b"pnum"), "Proportional Figures"),
    (tag!(b"pref"), "Pre-Base Forms"),
    (tag!(b"pres"), "Pre-base Substitutions"),
    (tag!(b"pstf"), "Post-base Forms"),
    (tag!(b"psts"), "Post-base Substitutions"),
    (tag!(b"pwid"), "Proportional Widths"),
    (tag!(b"qwid"), "Quarter Widths"),
    (tag!(b"rand"), "Randomize"),
    (tag!(b"rclt"), "Required Contextual Alternates"),
    (tag!(b"rkrf"), "Rakar Forms"),
    (tag!(b"rlig"), "Required Ligatures"),
    (tag!(b"rphf"), "Reph Forms"),
    (tag!(b"rtbd"), "Right Bounds"),
    (tag!(b"rtla"), "Right-to-left alternates"),
    (tag!(b"rtlm"), "Right-to-left mirrored forms"),
    (tag!(b"ruby"), "Ruby Notation Forms"),
    (tag!(b"rvrn"), "Required Variation Alternates"),
    (tag!(b"salt"), "Stylistic Alternates"),
    (tag!(b"sinf"), "Scientific Inferiors"),
    (tag!(b"size"), "Optical size"),
    (tag!(b"smcp"), "Small Capitals"),
    (tag!(b"smpl"), "Simplified Forms"),
    (tag!(b"ss01"), "Stylistic Set 1"),
    (tag!(b"ss02"), "Stylistic Set 2"),
    (tag!(b"ss03"), "Stylistic Set 3"),
    (tag!(b"ss04"), "Stylistic Set 4"),
    (tag!(b"ss05"), "Stylistic Set 5"),
    (tag!(b"ss06"), "Stylistic Set 6"),
    (tag!(b"ss07"), "Stylistic Set 7"),
    (tag!(b"ss08"), "Stylistic Set 8"),
    (tag!(b"ss09"), "Stylistic Set 9"),
    (tag!(b"ss10"), "Stylistic Set 10"),
    (tag!(b"ss11"), "Stylistic Set 11"),
    (tag!(b"ss12"), "Stylistic Set 12"),
    (tag!(b"ss13"), "Stylistic Set 13"),
    (tag!(b"ss14"), "Stylistic Set 14"),
    (tag!(b"ss15"), "Stylistic Set 15"),
    (tag!(b"ss16"), "Stylistic Set 16"),
    (tag!(b"ss17"), "Stylistic Set 17"),
    (tag!(b"ss18"), "Stylistic Set 18"),
    (tag!(b"ss19"), "Stylistic Set 19"),
    (tag!(b"ss20"), "Stylistic Set 20"),
    (tag!(b"ssty"), "Math script style alternates"),
    (tag!(b"stch"), "Stretching Glyph Decomposition"),
    (tag!(b"subs"), "Subscript"),
    (tag!(b"sups"), "Superscript"),
    (tag!(b"swsh"), "Swash"),
    (tag!(b"titl"), "Titling"),
    (tag!(b"tjmo"), "Trailing Jamo Forms"),
    (tag!(b"tnam"), "Traditional Name Forms"),
    (tag!(b"tnum"), "Tabular Figures"),
    (tag!(b"trad"), "Traditional Forms"),
    (tag!(b"twid"), "Third Widths"),
    (tag!(b"unic"), "Unicase"),
    (tag!(b"valt"), "Alternate Vertical Metrics"),
    (tag!(b"vatu"), "Vattu Variants"),
    (tag!(b"vchw"), "Vertical Contextual Half-width Spacing"),
    (tag!(b"vert"), "Vertical Writing"),
    (tag!(b"vhal"), "Alternate Vertical Half Metrics"),
    (tag!(b"vjmo"), "Vowel Jamo Forms"),
    (tag!(b"vkna"), "Vertical Kana Alternates"),
    (tag!(b"vkrn"), "Vertical Kerning"),
    (tag!(b"vpal"), "Proportional Alternate Vertical Metrics"),
    (tag!(b"vrt2"), "Vertical Alternates and Rotation"),
    (tag!(b"vrtr"), "Vertical Alternates for Rotation"),
    (tag!(b"zero"), "Slashed Zero"),
];
