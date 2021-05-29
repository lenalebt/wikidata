//! Various IDs for commonly used entities/properties on Wikidata.

#![allow(clippy::unreadable_literal)]

use super::*;

macro_rules! qid_consts (
    { $($key:ident => $value:expr),+, } => {
        $(
            #[allow(missing_docs)]
            pub const $key: Qid = Qid($value);
        )+
    };
);
macro_rules! pid_consts (
    { $($key:ident => $value:expr),+, } => {
        $(
            #[allow(missing_docs)]
            pub const $key: Pid = Pid($value);
        )+
    };
);

macro_rules! qid_unit_suffixes {
    { $($key:ident => $value:expr),+, } => {
        #[must_use]
        pub(super) const fn unit_suffix(qid: Qid) -> Option<&'static str> {
            $(
                if qid.0 == ($key).0 {
                    Some($value)
                } else
            )+
            {
                None
            }
        }
    };
}

impl Qid {
    /// If the Qid is a commonly used unit on Wikidata, get it as a unit suffix.
    #[must_use]
    pub const fn unit_suffix(self) -> Option<&'static str> {
        consts::unit_suffix(self)
    }
}

qid_consts! {
    EARTH => 2,
    HUMAN => 5,
    UNIT_OF_MEASUREMENT => 47574,
    PHYSICAL_QUANTITY => 107715,
    SI_BASE_UNIT => 223662,
    LENGTH => 36253,
    METRE => 11573,
    YOTTAMETRE => 10543042,
    ZETTAMETRE => 3277915,
    EXAMETRE => 3277907,
    PETAMETRE => 3277919,
    TERAMETRE => 3267417,
    GIGAMETRE => 854546,
    MEGAMETRE => 1054140,
    MYRIAMETRE => 1970718,
    KILOMETRE => 828224,
    HECTOMETRE => 844338,
    DECAMETRE => 848856,
    DECIMETRE => 200323,
    CENTIMETRE => 174728,
    MILLIMETRE => 174789,
    MICROMETRE => 175821,
    NANOMETRE => 178674,
    PICOMETRE => 192274,
    FEMTOMETRE => 208788,
    ATTOMETRE => 6003257,
    ZEPTOMETRE => 3270676,
    YOCTOMETRE => 3221356,
    PARSEC => 12129,
    GIGAPARSEC => 14916719,
    MEGAPARSEC => 3773454,
    KILOPARSEC => 11929860,
    ATTOPARSEC => 15784325,
    LIGHT_YEAR => 531,
    LIGHT_SECOND => 909315,
    ASTRONOMICAL_UNIT => 1811,
    MILE => 253276,
    FOOT => 3710,
    INCH => 218593,
    THOU => 1165799,
    AREA => 11500,
    SQUARE_METRE => 25343,
    SQUARE_KILOMETRE => 712226,
    SQUARE_CENTIMETRE => 2489298,
    SQUARE_MILLIMETRE => 2737347,
    ARE => 185078,
    HECTARE => 35852,
    VOLUME => 39297,
    CUBIC_METRE => 25517,
    CUBIC_KILOMETRE => 4243638,
    CUBIC_DECIMETRE => 2175964,
    CUBIC_CENTIMETRE => 1022113,
    CUBIC_MILLIMETRE => 3675550,
    LITER => 11582,
    HECTOLITER => 2029519,
    DECALITER => 2637946,
    CENTILITER => 1815100,
    MILLILITER => 2332346,
    MICROLITER => 2282891,
    PICOLITER => 3902688,
    FEMTOLITER => 3312063,
    TIME => 11471,
    FREQUENCY => 11652,
    TIME_INTERVAL => 186081,
    SECOND => 11574,
    MILLISECOND => 723733,
    MICROSECOND => 842015,
    NANOSECOND => 838801,
    FEMTOSECOND => 1777507,
    ATTOSECOND => 2483628,
    PICOSECOND => 3902709,
    DAY => 573,
    WEEK => 23387,
    HOUR => 25235,
    MINUTE => 7727,
    MONTH => 5151,
    ANNUM => 1092296,
    YEAR => 577,
    TWENTY_FOUR_HOUR_CLOCK => 216589,
    HERTZ => 39369,
    KILOHERTZ => 2143992,
    MEGAHERTZ => 732707,
    GIGAHERTZ => 3276763,
    MASS => 11423,
    KILOGRAM => 11570,
    YOTTAGRAM => 613726,
    ZETTAGRAM => 14754979,
    EXAGRAM => 2655272,
    PETAGRAM => 2612219,
    TERAGRAM => 1770733,
    GIGAGRAM => 2799294,
    MEGAGRAM => 11776930,
    MYRIAGRAM => 2151240,
    HECTOGRAM => 1057069,
    DECAGRAM => 6517513,
    GRAM => 41803,
    DECIGRAM => 1772386,
    CENTIGRAM => 2691798,
    MILLIGRAM => 3241121,
    MICROGRAM => 1645498,
    NANOGRAM => 2282906,
    PICOGRAM => 3239557,
    FEMTOGRAM => 1913097,
    ATTOGRAM => 2438073,
    ZEPTOGRAM => 6171168,
    YOCTOGRAM => 6170164,
    POUND => 100995,
    DALTON => 483261,
    DENSITY => 29539,
    KILOGRAM_PER_CUBIC_METRE => 844211,
    GRAM_PER_CUBIC_CENTIMETRE => 13147228,
    CONCENTRATION => 3686031,
    GRAM_PER_LITER => 834105,
    MILLILITRE_PER_LITRE => 21075844,
    MILLIGRAM_PER_CUBIC_METER => 21077820,
    MOL_PER_KILOGRAM_OF_SOLVENT => 21064838,
    MOL_PER_LITRE_OF_SOLUTION => 21064845,
    MASS_FRACTION => 899138,
    MOLE_FRACTION => 125264,
    VOLUME_FRACTION => 909482,
    PARTS_PER_MILLION => 21006887,
    PART_PER_BILLION => 2055118,
    MILLIGRAM_PER_KILOGRAM => 21091747,
    GRAM_PER_KILOGRAM => 21061369,
    TEMPERATURE => 11466,
    DEGREE_CELSIUS => 25267,
    KELVIN => 11579,
    DEGREE_FAHRENHEIT => 42289,
    RANKINE_SCALE => 207488,
    PRESSURE => 39552,
    ATMOSPHERE => 177974,
    TECHNICAL_ATMOSPHERE => 909066,
    BAR => 103510,
    PASCAL => 44395,
    MEGAPASCAL => 21062777,
    KILOPASCAL => 21064807,
    HECTOPASCAL => 5139563,
    TORR => 185648,
    MILLIMETER_OF_MERCURY => 6859652,
    METRE_OF_WATER => 2042279,
    CENTIMETRE_OF_WATER => 1247300,
    MILLIMETRE_OF_WATER => 13479685,
    HEAT_CAPACITY => 179388,
    JOULE_PER_MOLE_KELVIN => 20966455,
    THERMAL_CONDUCTIVITY => 487005,
    WATT_PER_METRE_KELVIN => 1463969,
    SPEED => 3711325,
    KILOMETRE_PER_HOUR => 180154,
    METRE_PER_SECOND => 182429,
    KNOT => 128822,
    KINEMATIC_VISCOSITY => 15106259,
    STOKES => 1569733,
    ELECTRICAL_CONDUCTIVITY => 4593291,
    AMPERE_PER_VOLT_METRE => 20966435,
    LUMINOSITY => 105902,
    SOLAR_LUMINOSITY => 843877,
    ENTHALPY => 161064,
    JOULE_PER_MOLE => 13035094,
    KILOJOULE_PER_MOLE => 752197,
    KILOJOULE_PER_KILOGRAM => 21077849,
    CURRENCY => 8142,
    EURO => 4916,
    CRORE => 1137675,
    INFECTION => 166231,
    DEGREE => 28390,
    BUSINESS => 4830453,
    FICTIONAL_HUMAN => 15632617,
}
// only include common ones
qid_unit_suffixes! {
    METRE => " m",
    KILOMETRE => " km",
    CENTIMETRE => " cm",
    MILLIMETRE => " mm",
    SQUARE_METRE => " m²",
    SQUARE_KILOMETRE => " km²",
    SQUARE_CENTIMETRE => " cm²",
    SQUARE_MILLIMETRE => " mm²",
    CUBIC_METRE => " m³",
    CUBIC_KILOMETRE => " km³",
    CUBIC_CENTIMETRE => " cm³",
    CUBIC_MILLIMETRE => " mm³",
    GRAM => " g",
    MILLIGRAM => " mg",
    KILOGRAM_PER_CUBIC_METRE => " kg/m³",
    GRAM_PER_CUBIC_CENTIMETRE => " g/cm³",
    MILLILITRE_PER_LITRE => " ml/l",
    MILLIGRAM_PER_CUBIC_METER => " mg/cm³",
    PARTS_PER_MILLION => " ppm",
    MILLIGRAM_PER_KILOGRAM => " mg/k",
    GRAM_PER_KILOGRAM => " g/kg",
    DEGREE_CELSIUS => " °C",
    KELVIN => " °K",
    DEGREE_FAHRENHEIT => " °F",
    KILOMETRE_PER_HOUR => " km/h",
    ASTRONOMICAL_UNIT => " AU",
    DEGREE => "°",
}
pid_consts! {
    INSTANCE_OF => 31,
    REFERENCE_URL => 854,
    LANGUAGE => 407, // language of work or name fully
    TITLE => 1476,
    AUTHOR => 50,
    AUTHOR_NAME_STRING => 2093,
    STATED_IN => 248,
    HEIGHT => 2048,
    DATE_OF_BIRTH => 569,
    DATE_OF_DEATH => 570,
    NET_WORTH => 2218,
    SPOUSE => 26,
    EDUCATED_AT => 69,
    NUMBER_OF_CHILDREN => 1971,
    AWARD_RECEIVED => 166,
    OFFICIAL_NAME => 1448,
    EMAIL => 968,
    SIBLING => 3373,
    NOMINATED_FOR => 1411,
    PHONE => 1329,
    EMPLOYEES => 1128,
    INCEPTION => 571,
    CEO => 169,
    TICKER_SYMBOL => 249,
    LEGAL_FORM => 1454,
    FOUNDED_BY => 112,
    SEX_OR_GENDER => 21,
    CITIZENSHIP => 27,
    PLACE_OF_BIRTH => 19,
    PLACE_OF_DEATH => 570,
    FATHER => 22,
    UNMARRIED_PARTNER => 451,
    CHILD => 40,
    MOTHER => 25,
    EYE_COLOR => 1340,
    HAIR_COLOR => 1884,
    HANDEDNESS => 552,
    MILITARY_RANK => 410,
    PRONOUN => 6553,
    PSUEDONYM => 742,
    TWITTER_USERNAME => 2002,
    FB_ID => 2013,
    YT_CHANNEL_ID => 2397,
    IG_USERNAME => 2003,
}