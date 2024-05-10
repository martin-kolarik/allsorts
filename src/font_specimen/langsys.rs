use crate::tag;

// https://learn.microsoft.com/en-us/typography/opentype/spec/languagetags
pub(crate) const LANGSYS: &[(u32, &str)] = &[
    (tag!(b"ABA "), "Abaza"),
    (tag!(b"ABK "), "Abkhazian"),
    (tag!(b"ACH "), "Acholi"),
    (tag!(b"ACR "), "Achi"),
    (tag!(b"ADY "), "Adyghe"),
    (tag!(b"AFK "), "Afrikaans"),
    (tag!(b"AFR "), "Afar"),
    (tag!(b"AGW "), "Agaw"),
    (tag!(b"AIO "), "Aiton"),
    (tag!(b"AKA "), "Akan"),
    (tag!(b"AKB "), "Batak Angkola"),
    (tag!(b"ALS "), "Alsatian"),
    (tag!(b"ALT "), "Altai"),
    (tag!(b"AMH "), "Amharic"),
    (tag!(b"ANG "), "Anglo-Saxon"),
    (
        tag!(b"APPH"),
        "Phonetic transcription—Americanist conventions",
    ),
    (tag!(b"ARA "), "Arabic"),
    (tag!(b"ARG "), "Aragonese"),
    (tag!(b"ARI "), "Aari"),
    (tag!(b"ARK "), "Rakhine"),
    (tag!(b"ASM "), "Assamese"),
    (tag!(b"AST "), "Asturian"),
    (tag!(b"ATH "), "Athapaskan languages"),
    (tag!(b"AVN "), "Avatime"),
    (tag!(b"AVR "), "Avar"),
    (tag!(b"AWA "), "Awadhi"),
    (tag!(b"AYM "), "Aymara"),
    (tag!(b"AZB "), "Torki"),
    (tag!(b"AZE "), "Azerbaijani"),
    (tag!(b"BAD "), "Badaga"),
    (tag!(b"BAD0"), "Banda"),
    (tag!(b"BAG "), "Baghelkhandi"),
    (tag!(b"BAL "), "Balkar"),
    (tag!(b"BAN "), "Balinese"),
    (tag!(b"BAR "), "Bavarian"),
    (tag!(b"BAU "), "Baulé"),
    (tag!(b"BBC "), "Batak Toba"),
    (tag!(b"BBR "), "Berber"),
    (tag!(b"BCH "), "Bench"),
    (tag!(b"BCR "), "Bible Cree"),
    (tag!(b"BDY "), "Bandjalang"),
    (tag!(b"BEL "), "Belarussian"),
    (tag!(b"BEM "), "Bemba"),
    (tag!(b"BEN "), "Bangla"),
    (tag!(b"BGC "), "Haryanvi"),
    (tag!(b"BGQ "), "Bagri"),
    (tag!(b"BGR "), "Bulgarian"),
    (tag!(b"BHI "), "Bhili"),
    (tag!(b"BHO "), "Bhojpuri"),
    (tag!(b"BIK "), "Bikol"),
    (tag!(b"BIL "), "Bilen"),
    (tag!(b"BIS "), "Bislama"),
    (tag!(b"BJJ "), "Kanauji"),
    (tag!(b"BKF "), "Blackfoot"),
    (tag!(b"BLI "), "Baluchi"),
    (tag!(b"BLK "), "Pa’o Karen"),
    (tag!(b"BLN "), "Balante"),
    (tag!(b"BLT "), "Balti"),
    (tag!(b"BMB "), "Bambara (Bamanankan)"),
    (tag!(b"BML "), "Bamileke"),
    (tag!(b"BOS "), "Bosnian"),
    (tag!(b"BPY "), "Bishnupriya Manipuri"),
    (tag!(b"BRE "), "Breton"),
    (tag!(b"BRH "), "Brahui"),
    (tag!(b"BRI "), "Braj Bhasha"),
    (tag!(b"BRM "), "Burmese"),
    (tag!(b"BRX "), "Bodo"),
    (tag!(b"BSH "), "Bashkir"),
    (tag!(b"BSK "), "Burushaski"),
    (tag!(b"BTD "), "Batak Dairi (Pakpak)"),
    (tag!(b"BTI "), "Beti"),
    (tag!(b"BTK "), "Batak languages"),
    (tag!(b"BTM "), "Batak Mandailing"),
    (tag!(b"BTS "), "Batak Simalungun"),
    (tag!(b"BTX "), "Batak Karo"),
    (tag!(b"BTZ "), "Batak Alas-Kluet"),
    (tag!(b"BUG "), "Bugis"),
    (tag!(b"BYV "), "Medumba"),
    (tag!(b"CAK "), "Kaqchikel"),
    (tag!(b"CAT "), "Catalan"),
    (tag!(b"CBK "), "Zamboanga Chavacano"),
    (tag!(b"CCHN"), "Chinantec"),
    (tag!(b"CEB "), "Cebuano"),
    (tag!(b"CGG "), "Chiga"),
    (tag!(b"CHA "), "Chamorro"),
    (tag!(b"CHE "), "Chechen"),
    (tag!(b"CHG "), "Chaha Gurage"),
    (tag!(b"CHH "), "Chattisgarhi"),
    (tag!(b"CHI "), "Chichewa (Chewa, Nyanja)"),
    (tag!(b"CHK "), "Chukchi"),
    (tag!(b"CHK0"), "Chuukese"),
    (tag!(b"CHO "), "Choctaw"),
    (tag!(b"CHP "), "Chipewyan"),
    (tag!(b"CHR "), "Cherokee"),
    (tag!(b"CHU "), "Chuvash"),
    (tag!(b"CHY "), "Cheyenne"),
    (tag!(b"CJA "), "Western Cham"),
    (tag!(b"CJM "), "Eastern Cham"),
    (tag!(b"CMR "), "Comorian"),
    (tag!(b"COP "), "Coptic"),
    (tag!(b"COR "), "Cornish"),
    (tag!(b"COS "), "Corsican"),
    (tag!(b"CPP "), "Creoles"),
    (tag!(b"CRE "), "Cree"),
    (tag!(b"CRR "), "Carrier"),
    (tag!(b"CRT "), "Crimean Tatar"),
    (tag!(b"CSB "), "Kashubian"),
    (tag!(b"CSL "), "Church Slavonic"),
    (tag!(b"CSY "), "Czech"),
    (tag!(b"CTG "), "Chittagonian"),
    (tag!(b"CTT "), "Wayanad Chetti"),
    (tag!(b"CUK "), "San Blas Kuna"),
    (tag!(b"DAG "), "Dagbani"),
    (tag!(b"DAN "), "Danish"),
    (tag!(b"DAR "), "Dargwa"),
    (tag!(b"DAX "), "Dayi"),
    (tag!(b"DCR "), "Woods Cree"),
    (tag!(b"DEU "), "German"),
    (tag!(b"DGO "), "Dogri (individual language)"),
    (tag!(b"DGR "), "Dogri (macrolanguage)"),
    (tag!(b"DHG "), "Dhangu"),
    (tag!(b"DHV "), "Divehi (Dhivehi, Maldivian)"), // (deprecated)
    (tag!(b"DIQ "), "Dimli"),
    (tag!(b"DIV "), "Divehi (Dhivehi, Maldivian)"),
    (tag!(b"DJR "), "Zarma"),
    (tag!(b"DJR0"), "Djambarrpuyngu"),
    (tag!(b"DNG "), "Dangme"),
    (tag!(b"DNJ "), "Dan"),
    (tag!(b"DNK "), "Dinka"),
    (tag!(b"DRI "), "Dari"),
    (tag!(b"DUJ "), "Dhuwal"),
    (tag!(b"DUN "), "Dungan"),
    (tag!(b"DZN "), "Dzongkha"),
    (tag!(b"EBI "), "Ebira"),
    (tag!(b"ECR "), "Eastern Cree"),
    (tag!(b"EDO "), "Edo"),
    (tag!(b"EFI "), "Efik"),
    (tag!(b"ELL "), "Greek"),
    (tag!(b"EMK "), "Eastern Maninkakan"),
    (tag!(b"ENG "), "English"),
    (tag!(b"ERZ "), "Erzya"),
    (tag!(b"ESP "), "Spanish"),
    (tag!(b"ESU "), "Central Yupik"),
    (tag!(b"ETI "), "Estonian"),
    (tag!(b"EUQ "), "Basque"),
    (tag!(b"EVK "), "Evenki"),
    (tag!(b"EVN "), "Even"),
    (tag!(b"EWE "), "Ewe"),
    (tag!(b"FAN "), "French Antillean"),
    (tag!(b"FAN0"), "Fang"),
    (tag!(b"FAR "), "Persian"),
    (tag!(b"FAT "), "Fanti"),
    (tag!(b"FIN "), "Finnish"),
    (tag!(b"FJI "), "Fijian"),
    (tag!(b"FLE "), "Dutch (Flemish)"),
    (tag!(b"FMP "), "Fe’fe’"),
    (tag!(b"FNE "), "Forest Enets"),
    (tag!(b"FON "), "Fon"),
    (tag!(b"FOS "), "Faroese"),
    (tag!(b"FRA "), "French"),
    (tag!(b"FRC "), "Cajun French"),
    (tag!(b"FRI "), "Frisian"),
    (tag!(b"FRL "), "Friulian"),
    (tag!(b"FRP "), "Arpitan"),
    (tag!(b"FTA "), "Futa"),
    (tag!(b"FUL "), "Fulah"),
    (tag!(b"FUV "), "Nigerian Fulfulde"),
    (tag!(b"GAD "), "Ga"),
    (tag!(b"GAE "), "Scottish Gaelic"),
    (tag!(b"GAG "), "Gagauz"),
    (tag!(b"GAL "), "Galician"),
    (tag!(b"GAR "), "Garshuni"),
    (tag!(b"GAW "), "Garhwali"),
    (tag!(b"GEZ "), "Geez"),
    (tag!(b"GIH "), "Githabul"),
    (tag!(b"GIL "), "Gilyak"),
    (tag!(b"GIL0"), "Kiribati (Gilbertese)"),
    (tag!(b"GKP "), "Kpelle (Guinea)"),
    (tag!(b"GLK "), "Gilaki"),
    (tag!(b"GMZ "), "Gumuz"),
    (tag!(b"GNN "), "Gumatj"),
    (tag!(b"GOG "), "Gogo"),
    (tag!(b"GON "), "Gondi"),
    (tag!(b"GRN "), "Greenlandic"),
    (tag!(b"GRO "), "Garo"),
    (tag!(b"GUA "), "Guarani"),
    (tag!(b"GUC "), "Wayuu"),
    (tag!(b"GUF "), "Gupapuyngu"),
    (tag!(b"GUJ "), "Gujarati"),
    (tag!(b"GUZ "), "Gusii"),
    (tag!(b"HAI "), "Haitian (Haitian Creole)"),
    (tag!(b"HAI0"), "Haida"),
    (tag!(b"HAL "), "Halam (Falam Chin)"),
    (tag!(b"HAR "), "Harauti"),
    (tag!(b"HAU "), "Hausa"),
    (tag!(b"HAW "), "Hawaiian"),
    (tag!(b"HAY "), "Haya"),
    (tag!(b"HAZ "), "Hazaragi"),
    (tag!(b"HBN "), "Hammer-Banna"),
    (tag!(b"HEI "), "Heiltsuk"),
    (tag!(b"HER "), "Herero"),
    (tag!(b"HIL "), "Hiligaynon"),
    (tag!(b"HIN "), "Hindi"),
    (tag!(b"HMA "), "High Mari"),
    (tag!(b"HMD "), "A-Hmao"),
    (tag!(b"HMN "), "Hmong"),
    (tag!(b"HMO "), "Hiri Motu"),
    (tag!(b"HMZ "), "Hmong Shuat"),
    (tag!(b"HND "), "Hindko"),
    (tag!(b"HO '"), "Ho"),
    (tag!(b"HRI "), "Harari"),
    (tag!(b"HRV "), "Croatian"),
    (tag!(b"HUN "), "Hungarian"),
    (tag!(b"HYE "), "Armenian"),
    (tag!(b"HYE0"), "Armenian East"),
    (tag!(b"IBA "), "Iban"),
    (tag!(b"IBB "), "Ibibio"),
    (tag!(b"IBO "), "Igbo"),
    (tag!(b"IDO "), "Ido"),
    (tag!(b"IJO "), "Ijo languages"),
    (tag!(b"ILE "), "Interlingue"),
    (tag!(b"ILO "), "Ilokano"),
    (tag!(b"INA "), "Interlingua"),
    (tag!(b"IND "), "Indonesian"),
    (tag!(b"ING "), "Ingush"),
    (tag!(b"INU "), "Inuktitut"),
    (tag!(b"INUK"), "Nunavik Inuktitut"),
    (tag!(b"IPK "), "Inupiat"),
    (tag!(b"IPPH"), "Phonetic transcription—IPA conventions"),
    (tag!(b"IRI "), "Irish"),
    (tag!(b"IRT "), "Irish Traditional"),
    (tag!(b"IRU "), "Irula"),
    (tag!(b"ISL "), "Icelandic"),
    (tag!(b"ISM "), "Inari Sami"),
    (tag!(b"ITA "), "Italian"),
    (tag!(b"IWR "), "Hebrew"),
    (tag!(b"JAM "), "Jamaican Creole"),
    (tag!(b"JAN "), "Japanese"),
    (tag!(b"JAV "), "Javanese"),
    (tag!(b"JBO "), "Lojban"),
    (tag!(b"JCT "), "Krymchak"),
    (tag!(b"JII "), "Yiddish"),
    (tag!(b"JUD "), "Ladino"),
    (tag!(b"JUL "), "Jula"),
    (tag!(b"KAB "), "Kabardian"),
    (tag!(b"KAB0"), "Kabyle"),
    (tag!(b"KAC "), "Kachchi"),
    (tag!(b"KAL "), "Kalenjin"),
    (tag!(b"KAN "), "Kannada"),
    (tag!(b"KAR "), "Karachay"),
    (tag!(b"KAT "), "Georgian"),
    (tag!(b"KAW "), "Kawi (Old Javanese)"),
    (tag!(b"KAZ "), "Kazakh"),
    (tag!(b"KDE "), "Makonde"),
    (tag!(b"KEA "), "Kabuverdianu (Crioulo)"),
    (tag!(b"KEB "), "Kebena"),
    (tag!(b"KEK "), "Kekchi"),
    (tag!(b"KGE "), "Khutsuri Georgian"),
    (tag!(b"KHA "), "Khakass"),
    (tag!(b"KHK "), "Khanty-Kazim"),
    (tag!(b"KHM "), "Khmer"),
    (tag!(b"KHS "), "Khanty-Shurishkar"),
    (tag!(b"KHT "), "Khamti Shan"),
    (tag!(b"KHV "), "Khanty-Vakhi"),
    (tag!(b"KHW "), "Khowar"),
    (tag!(b"KIK "), "Kikuyu (Gikuyu)"),
    (tag!(b"KIR "), "Kirghiz (Kyrgyz)"),
    (tag!(b"KIS "), "Kisii"),
    (tag!(b"KIU "), "Kirmanjki"),
    (tag!(b"KJD "), "Southern Kiwai"),
    (tag!(b"KJP "), "Eastern Pwo Karen"),
    (tag!(b"KJZ "), "Bumthangkha"),
    (tag!(b"KKN "), "Kokni"),
    (tag!(b"KLM "), "Kalmyk"),
    (tag!(b"KMB "), "Kamba"),
    (tag!(b"KMN "), "Kumaoni"),
    (tag!(b"KMO "), "Komo"),
    (tag!(b"KMS "), "Komso"),
    (tag!(b"KMZ "), "Khorasani Turkic"),
    (tag!(b"KNR "), "Kanuri"),
    (tag!(b"KOD "), "Kodagu"),
    (tag!(b"KOH "), "Korean Old Hangul"),
    (tag!(b"KOK "), "Konkani"),
    (tag!(b"KOM "), "Komi"),
    (tag!(b"KON "), "Kikongo"),
    (tag!(b"KON0"), "Kongo"),
    (tag!(b"KOP "), "Komi-Permyak"),
    (tag!(b"KOR "), "Korean"),
    (tag!(b"KOS "), "Kosraean"),
    (tag!(b"KOZ "), "Komi-Zyrian"),
    (tag!(b"KPL "), "Kpelle"),
    (tag!(b"KRI "), "Krio"),
    (tag!(b"KRK "), "Karakalpak"),
    (tag!(b"KRL "), "Karelian"),
    (tag!(b"KRM "), "Karaim"),
    (tag!(b"KRN "), "Karen"),
    (tag!(b"KRT "), "Koorete"),
    (tag!(b"KSH "), "Kashmiri"),
    (tag!(b"KSH0"), "Ripuarian"),
    (tag!(b"KSI "), "Khasi"),
    (tag!(b"KSM "), "Kildin Sami"),
    (tag!(b"KSW "), "S’gaw Karen"),
    (tag!(b"KUA "), "Kuanyama"),
    (tag!(b"KUI "), "Kui"),
    (tag!(b"KUL "), "Kulvi"),
    (tag!(b"KUM "), "Kumyk"),
    (tag!(b"KUR "), "Kurdish"),
    (tag!(b"KUU "), "Kurukh"),
    (tag!(b"KUY "), "Kuy"),
    (tag!(b"KWK "), "Kwakʼwala"),
    (tag!(b"KYK "), "Koryak"),
    (tag!(b"KYU "), "Western Kayah"),
    (tag!(b"LAD "), "Ladin"),
    (tag!(b"LAH "), "Lahuli"),
    (tag!(b"LAK "), "Lak"),
    (tag!(b"LAM "), "Lambani"),
    (tag!(b"LAO "), "Lao"),
    (tag!(b"LAT "), "Latin"),
    (tag!(b"LAZ "), "Laz"),
    (tag!(b"LCR "), "L-Cree"),
    (tag!(b"LDK "), "Ladakhi"),
    (tag!(b"LEF "), "Lelemi"),
    (tag!(b"LEZ "), "Lezgi"),
    (tag!(b"LIJ "), "Ligurian"),
    (tag!(b"LIM "), "Limburgish"),
    (tag!(b"LIN "), "Lingala"),
    (tag!(b"LIS "), "Lisu"),
    (tag!(b"LJP "), "Lampung"),
    (tag!(b"LKI "), "Laki"),
    (tag!(b"LMA "), "Low Mari"),
    (tag!(b"LMB "), "Limbu"),
    (tag!(b"LMO "), "Lombard"),
    (tag!(b"LMW "), "Lomwe"),
    (tag!(b"LOM "), "Loma"),
    (tag!(b"LPO "), "Lipo"),
    (tag!(b"LRC "), "Luri"),
    (tag!(b"LSB "), "Lower Sorbian"),
    (tag!(b"LSM "), "Lule Sami"),
    (tag!(b"LTH "), "Lithuanian"),
    (tag!(b"LTZ "), "Luxembourgish"),
    (tag!(b"LUA "), "Luba-Lulua"),
    (tag!(b"LUB "), "Luba-Katanga"),
    (tag!(b"LUG "), "Ganda"),
    (tag!(b"LUH "), "Luyia"),
    (tag!(b"LUO "), "Luo"),
    (tag!(b"LVI "), "Latvian"),
    (tag!(b"MAD "), "Madura"),
    (tag!(b"MAG "), "Magahi"),
    (tag!(b"MAH "), "Marshallese"),
    (tag!(b"MAJ "), "Majang"),
    (tag!(b"MAK "), "Makhuwa"),
    (tag!(b"MAL "), "Malayalam"),
    (tag!(b"MAM "), "Mam"),
    (tag!(b"MAN "), "Mansi"),
    (tag!(b"MAP "), "Mapudungun"),
    (tag!(b"MAR "), "Marathi"),
    (tag!(b"MAW "), "Marwari"),
    (tag!(b"MBN "), "Mbundu"),
    (tag!(b"MBO "), "Mbo"),
    (tag!(b"MCH "), "Manchu"),
    (tag!(b"MCR "), "Moose Cree"),
    (tag!(b"MDE "), "Mende"),
    (tag!(b"MDR "), "Mandar"),
    (tag!(b"MEN "), "Me’en"),
    (tag!(b"MER "), "Meru"),
    (tag!(b"MFA "), "Pattani Malay"),
    (tag!(b"MFE "), "Morisyen"),
    (tag!(b"MIN "), "Minangkabau"),
    (tag!(b"MIZ "), "Mizo"),
    (tag!(b"MKD "), "Macedonian"),
    (tag!(b"MKR "), "Makasar"),
    (tag!(b"MKW "), "Kituba"),
    (tag!(b"MLE "), "Male"),
    (tag!(b"MLG "), "Malagasy"),
    (tag!(b"MLN "), "Malinke"),
    (tag!(b"MLR "), "Malayalam Reformed"),
    (tag!(b"MLY "), "Malay"),
    (tag!(b"MND "), "Mandinka"),
    (tag!(b"MNG "), "Mongolian"),
    (tag!(b"MNI "), "Manipuri"),
    (tag!(b"MNK "), "Maninka"),
    (tag!(b"MNX "), "Manx"),
    (tag!(b"MOH "), "Mohawk"),
    (tag!(b"MOK "), "Moksha"),
    (tag!(b"MOL "), "Romanian (Moldova)"),
    (tag!(b"MON "), "Mon"),
    (tag!(b"MONT"), "Thailand Mon"),
    (tag!(b"MOR "), "Moroccan"),
    (tag!(b"MOS "), "Mossi"),
    (tag!(b"MRI "), "Maori"),
    (tag!(b"MTH "), "Maithili"),
    (tag!(b"MTS "), "Maltese"),
    (tag!(b"MUN "), "Mundari"),
    (tag!(b"MUS "), "Muscogee"),
    (tag!(b"MWL "), "Mirandese"),
    (tag!(b"MWW "), "Hmong Daw"),
    (tag!(b"MYN "), "Mayan"),
    (tag!(b"MZN "), "Mazanderani"),
    (tag!(b"NAG "), "Naga-Assamese"),
    (tag!(b"NAH "), "Nahuatl"),
    (tag!(b"NAN "), "Nanai"),
    (tag!(b"NAP "), "Neapolitan"),
    (tag!(b"NAS "), "Naskapi"),
    (tag!(b"NAU "), "Nauruan"),
    (tag!(b"NAV "), "Navajo"),
    (tag!(b"NCR "), "N-Cree"),
    (tag!(b"NDB "), "Ndebele"),
    (tag!(b"NDC "), "Ndau"),
    (tag!(b"NDG "), "Ndonga"),
    (tag!(b"NDS "), "Low Saxon"),
    (tag!(b"NEP "), "Nepali"),
    (tag!(b"NEW "), "Newari"),
    (tag!(b"NGA "), "Ngbaka"),
    (tag!(b"NGR "), "Nagari"),
    (tag!(b"NHC "), "Norway House Cree"),
    (tag!(b"NIS "), "Nisi"),
    (tag!(b"NIU "), "Niuean"),
    (tag!(b"NKL "), "Nyankole"),
    (tag!(b"NKO "), "N’Ko"),
    (tag!(b"NLD "), "Dutch"),
    (tag!(b"NOE "), "Nimadi"),
    (tag!(b"NOG "), "Nogai"),
    (tag!(b"NOR "), "Norwegian"),
    (tag!(b"NOV "), "Novial"),
    (tag!(b"NSM "), "Northern Sami"),
    (tag!(b"NSO "), "Northern Sotho"),
    (tag!(b"NTA "), "Northern Tai"),
    (tag!(b"NTO "), "Esperanto"),
    (tag!(b"NYM "), "Nyamwezi"),
    (tag!(b"NYN "), "Norwegian Nynorsk (Nynorsk, Norwegian)"),
    (tag!(b"NZA "), "Mbembe Tigon"),
    (tag!(b"OCI "), "Occitan"),
    (tag!(b"OCR "), "Oji-Cree"),
    (tag!(b"OJB "), "Ojibway"),
    (tag!(b"ORI "), "Odia"),
    (tag!(b"ORO "), "Oromo"),
    (tag!(b"OSS "), "Ossetian"),
    (tag!(b"PAA "), "Palestinian Aramaic"),
    (tag!(b"PAG "), "Pangasinan"),
    (tag!(b"PAL "), "Pali"),
    (tag!(b"PAM "), "Pampangan"),
    (tag!(b"PAN "), "Punjabi"),
    (tag!(b"PAP "), "Palpa"),
    (tag!(b"PAP0"), "Papiamentu"),
    (tag!(b"PAS "), "Pashto"),
    (tag!(b"PAU "), "Palauan"),
    (tag!(b"PCC "), "Bouyei"),
    (tag!(b"PCD "), "Picard"),
    (tag!(b"PDC "), "Pennsylvania German"),
    (tag!(b"PGR "), "Polytonic Greek"),
    (tag!(b"PHK "), "Phake"),
    (tag!(b"PIH "), "Norfolk"),
    (tag!(b"PIL "), "Filipino"),
    (tag!(b"PLG "), "Palaung"),
    (tag!(b"PLK "), "Polish"),
    (tag!(b"PMS "), "Piemontese"),
    (tag!(b"PNB "), "Western Panjabi"),
    (tag!(b"POH "), "Pocomchi"),
    (tag!(b"PON "), "Pohnpeian"),
    (tag!(b"PRO "), "Provençal / Old Provençal"),
    (tag!(b"PTG "), "Portuguese"),
    (tag!(b"PWO "), "Western Pwo Karen"),
    (tag!(b"QIN "), "Chin"),
    (tag!(b"QUC "), "K’iche’"),
    (tag!(b"QUH "), "Quechua (Bolivia)"),
    (tag!(b"QUZ "), "Quechua"),
    (tag!(b"QVI "), "Quechua (Ecuador)"),
    (tag!(b"QWH "), "Quechua (Peru)"),
    (tag!(b"RAJ "), "Rajasthani"),
    (tag!(b"RAR "), "Rarotongan"),
    (tag!(b"RBU "), "Russian Buriat"),
    (tag!(b"RCR "), "R-Cree"),
    (tag!(b"REJ "), "Rejang"),
    (tag!(b"RIA "), "Riang"),
    (tag!(b"RHG "), "Rohingya"),
    (tag!(b"RIF "), "Tarifit"),
    (tag!(b"RIT "), "Ritarungo"),
    (tag!(b"RKW "), "Arakwal"),
    (tag!(b"RMS "), "Romansh"),
    (tag!(b"RMY "), "Vlax Romani"),
    (tag!(b"ROM "), "Romanian"),
    (tag!(b"ROY "), "Romany"),
    (tag!(b"RSY "), "Rusyn"),
    (tag!(b"RTM "), "Rotuman"),
    (tag!(b"RUA "), "Kinyarwanda"),
    (tag!(b"RUN "), "Rundi"),
    (tag!(b"RUP "), "Aromanian"),
    (tag!(b"RUS "), "Russian"),
    (tag!(b"SAD "), "Sadri"),
    (tag!(b"SAN "), "Sanskrit"),
    (tag!(b"SAS "), "Sasak"),
    (tag!(b"SAT "), "Santali"),
    (tag!(b"SAY "), "Sayisi"),
    (tag!(b"SCN "), "Sicilian"),
    (tag!(b"SCO "), "Scots"),
    (tag!(b"SCS "), "North Slavey"),
    (tag!(b"SEK "), "Sekota"),
    (tag!(b"SEL "), "Selkup"),
    (tag!(b"SFM "), "Small Flowery Miao"),
    (tag!(b"SGA "), "Old Irish"),
    (tag!(b"SGO "), "Sango"),
    (tag!(b"SGS "), "Samogitian"),
    (tag!(b"SHI "), "Tachelhit"),
    (tag!(b"SHN "), "Shan"),
    (tag!(b"SIB "), "Sibe"),
    (tag!(b"SID "), "Sidamo"),
    (tag!(b"SIG "), "Silte Gurage"),
    (tag!(b"SKS "), "Skolt Sami"),
    (tag!(b"SKY "), "Slovak"),
    (tag!(b"SLA "), "Slavey"),
    (tag!(b"SLV "), "Slovenian"),
    (tag!(b"SML "), "Somali"),
    (tag!(b"SMO "), "Samoan"),
    (tag!(b"SNA "), "Sena"),
    (tag!(b"SNA0"), "Shona"),
    (tag!(b"SND "), "Sindhi"),
    (tag!(b"SNH "), "Sinhala (Sinhalese)"),
    (tag!(b"SNK "), "Soninke"),
    (tag!(b"SOG "), "Sodo Gurage"),
    (tag!(b"SOP "), "Songe"),
    (tag!(b"SOT "), "Southern Sotho"),
    (tag!(b"SQI "), "Albanian"),
    (tag!(b"SRB "), "Serbian"),
    (tag!(b"SRD "), "Sardinian"),
    (tag!(b"SRK "), "Saraiki"),
    (tag!(b"SRR "), "Serer"),
    (tag!(b"SSL "), "South Slavey"),
    (tag!(b"SSM "), "Southern Sami"),
    (tag!(b"STQ "), "Saterland Frisian"),
    (tag!(b"SUK "), "Sukuma"),
    (tag!(b"SUN "), "Sundanese"),
    (tag!(b"SUR "), "Suri"),
    (tag!(b"SVA "), "Svan"),
    (tag!(b"SVE "), "Swedish"),
    (tag!(b"SWA "), "Swadaya Aramaic"),
    (tag!(b"SWK "), "Swahili"),
    (tag!(b"SWZ "), "Swati"),
    (tag!(b"SXT "), "Sutu"),
    (tag!(b"SXU "), "Upper Saxon"),
    (tag!(b"SYL "), "Sylheti"),
    (tag!(b"SYR "), "Syriac"),
    (
        tag!(b"SYRE"),
        "Syriac, Estrangela script-variant (equivalent to ISO 15924 'Syre')",
    ),
    (
        tag!(b"SYRJ"),
        "Syriac, Western script-variant (equivalent to ISO 15924 'Syrj')",
    ),
    (
        tag!(b"SYRN"),
        "Syriac, Eastern script-variant (equivalent to ISO 15924 'Syrn')",
    ),
    (tag!(b"SZL "), "Silesian"),
    (tag!(b"TAB "), "Tabasaran"),
    (tag!(b"TAJ "), "Tajiki"),
    (tag!(b"TAM "), "Tamil"),
    (tag!(b"TAT "), "Tatar"),
    (tag!(b"TCR "), "TH-Cree"),
    (tag!(b"TDD "), "Dehong Dai"),
    (tag!(b"TEL "), "Telugu"),
    (tag!(b"TET "), "Tetum"),
    (tag!(b"TGL "), "Tagalog"),
    (tag!(b"TGN "), "Tongan"),
    (tag!(b"TGR "), "Tigre"),
    (tag!(b"TGY "), "Tigrinya"),
    (tag!(b"THA "), "Thai"),
    (tag!(b"THT "), "Tahitian"),
    (tag!(b"TIB "), "Tibetan"),
    (tag!(b"TIV "), "Tiv"),
    (tag!(b"TJL "), "Tai Laing"),
    (tag!(b"TKM "), "Turkmen"),
    (tag!(b"TLI "), "Tlingit"),
    (tag!(b"TMH "), "Tamashek"),
    (tag!(b"TMN "), "Temne"),
    (tag!(b"TNA "), "Tswana"),
    (tag!(b"TNE "), "Tundra Enets"),
    (tag!(b"TNG "), "Tonga"),
    (tag!(b"TOD "), "Todo"),
    (tag!(b"TOD0"), "Toma"),
    (tag!(b"TPI "), "Tok Pisin"),
    (tag!(b"TRK "), "Turkish"),
    (tag!(b"TSG "), "Tsonga"),
    (tag!(b"TSJ "), "Tshangla"),
    (tag!(b"TUA "), "Turoyo Aramaic"),
    (tag!(b"TUL "), "Tulu"),
    (tag!(b"TUM "), "Tumbuka"),
    (tag!(b"TUV "), "Tuvin"),
    (tag!(b"TVL "), "Tuvalu"),
    (tag!(b"TWI "), "Twi"),
    (tag!(b"TYZ "), "Tày"),
    (tag!(b"TZM "), "Tamazight"),
    (tag!(b"TZO "), "Tzotzil"),
    (tag!(b"UDM "), "Udmurt"),
    (tag!(b"UKR "), "Ukrainian"),
    (tag!(b"UMB "), "Umbundu"),
    (tag!(b"URD "), "Urdu"),
    (tag!(b"USB "), "Upper Sorbian"),
    (tag!(b"UYG "), "Uyghur"),
    (tag!(b"UZB "), "Uzbek"),
    (tag!(b"VEC "), "Venetian"),
    (tag!(b"VEN "), "Venda"),
    (tag!(b"VIT "), "Vietnamese"),
    (tag!(b"VOL "), "Volapük"),
    (tag!(b"VRO "), "Võro"),
    (tag!(b"WA '"), "Wa"),
    (tag!(b"WAG "), "Wagdi"),
    (tag!(b"WAR "), "Waray-Waray"),
    (tag!(b"WCI "), "Waci Gbe"),
    (tag!(b"WCR "), "West-Cree"),
    (tag!(b"WEL "), "Welsh"),
    (tag!(b"WLF "), "Wolof"),
    (tag!(b"WLN "), "Walloon"),
    (tag!(b"WTM "), "Mewati"),
    (tag!(b"XBD "), "Lü"),
    (tag!(b"XHS "), "Xhosa"),
    (tag!(b"XJB "), "Minjangbal"),
    (tag!(b"XKF "), "Khengkha"),
    (tag!(b"XOG "), "Soga"),
    (tag!(b"XPE "), "Kpelle (Liberia)"),
    (tag!(b"XUB "), "Bette Kuruma"),
    (tag!(b"XUJ "), "Jennu Kuruma"),
    (tag!(b"YAK "), "Sakha"),
    (tag!(b"YAO "), "Yao"),
    (tag!(b"YAP "), "Yapese"),
    (tag!(b"YBA "), "Yoruba"),
    (tag!(b"YCR "), "Y-Cree"),
    (tag!(b"YGP "), "Gepo"),
    (tag!(b"YIC "), "Yi Classic"),
    (tag!(b"YIM "), "Yi Modern"),
    (tag!(b"YNA "), "Aluo"),
    (tag!(b"YWQ "), "Wuding-Luquan Yi"),
    (tag!(b"ZEA "), "Zealandic"),
    (tag!(b"ZGH "), "Standard Moroccan Tamazight"),
    (tag!(b"ZHA "), "Zhuang"),
    (tag!(b"ZHH "), "Chinese, Traditional, Hong Kong SAR"),
    (tag!(b"ZHP "), "Chinese, Phonetic"),
    (tag!(b"ZHS "), "Chinese, Simplified"),
    (tag!(b"ZHT "), "Chinese, Traditional"),
    (tag!(b"ZHTM"), "Chinese, Traditional, Macao SAR"),
    (tag!(b"ZND "), "Zande"),
    (tag!(b"ZUL "), "Zulu"),
    (tag!(b"ZZA "), "Zazaki"),
];
