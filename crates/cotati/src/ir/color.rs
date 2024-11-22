use super::FrameVariable;

/// A color structure repesents as RGBA, the storage value is normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

/// Rgba can be used as context variant type.
impl FrameVariable for Rgba {}

impl From<Rgba> for [f32; 4] {
    fn from(value: Rgba) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl From<u32> for Rgba {
    fn from(hex: u32) -> Self {
        Rgba::rgb(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            (hex & 0xff) as u8,
        )
    }
}

impl From<(u32, u8)> for Rgba {
    fn from(value: (u32, u8)) -> Self {
        let (hex, opacity) = value;

        Rgba::new(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            (hex & 0xff) as u8,
            opacity,
        )
    }
}
impl From<(u8, u8, u8)> for Rgba {
    fn from(value: (u8, u8, u8)) -> Self {
        let (r, g, b) = value;

        Rgba::rgb(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    fn from(value: (u8, u8, u8, u8)) -> Self {
        let (r, g, b, a) = value;

        Rgba::new(r, g, b, a)
    }
}

impl Rgba {
    /// Create a `Rgba` with opaque alpha channel.
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }

    /// Create a `Rgba` with normalized rgb values.
    pub const fn rgbf(red: f32, green: f32, blue: f32) -> Self {
        Self::newf(red, green, blue, 1.0)
    }

    /// Create a `Rgba` with normalized rgba values.
    pub const fn newf(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(red, green, blue, alpha)
    }

    /// Create a new `Rgba` isntance.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(
            red as f32 / 255f32,
            green as f32 / 255f32,
            blue as f32 / 255f32,
            alpha as f32 / 255f32,
        )
    }
}

/// Recognized color keyword names, compliant with svg 1.1.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RecognizedColor {
    aliceblue,
    antiquewhite,
    aqua,
    aquamarine,
    azure,
    beige,
    bisque,
    black,
    blanchedalmond,
    blue,
    blueviolet,
    brown,
    burlywood,
    cadetblue,
    chartreuse,
    chocolate,
    coral,
    cornflowerblue,
    cornsilk,
    crimson,
    cyan,
    darkblue,
    darkcyan,
    darkgoldenrod,
    darkgray,
    darkgreen,
    darkgrey,
    darkkhaki,
    darkmagenta,
    darkolivegreen,
    darkorange,
    darkorchid,
    darkred,
    darksalmon,
    darkseagreen,
    darkslateblue,
    darkslategray,
    darkslategrey,
    darkturquoise,
    darkviolet,
    deeppink,
    deepskyblue,
    dimgray,
    dimgrey,
    dodgerblue,
    firebrick,
    floralwhite,
    forestgreen,
    fuchsia,
    gainsboro,
    ghostwhite,
    gold,
    goldenrod,
    gray,
    grey,
    green,
    greenyellow,
    honeydew,
    hotpink,
    indianred,
    indigo,
    ivory,
    khaki,
    lavender,
    lavenderblush,
    lawngreen,
    lemonchiffon,
    lightblue,
    lightcoral,
    lightcyan,
    lightgoldenrodyellow,
    lightgray,
    lightgreen,
    lightgrey,
    lightpink,
    lightsalmon,
    lightseagreen,
    lightskyblue,
    lightslategray,
    lightslategrey,
    lightsteelblue,
    lightyellow,
    lime,
    limegreen,
    linen,
    magenta,
    maroon,
    mediumaquamarine,
    mediumblue,
    mediumorchid,
    mediumpurple,
    mediumseagreen,
    mediumslateblue,
    mediumspringgreen,
    mediumturquoise,
    mediumvioletred,
    midnightblue,
    mintcream,
    mistyrose,
    moccasin,
    navajowhite,
    navy,
    oldlace,
    olive,
    olivedrab,
    orange,
    orangered,
    orchid,
    palegoldenrod,
    palegreen,
    paleturquoise,
    palevioletred,
    papayawhip,
    peachpuff,
    peru,
    pink,
    plum,
    powderblue,
    purple,
    red,
    rosybrown,
    royalblue,
    saddlebrown,
    salmon,
    sandybrown,
    seagreen,
    seashell,
    sienna,
    silver,
    skyblue,
    slateblue,
    slategray,
    slategrey,
    snow,
    springgreen,
    steelblue,
    tan,
    teal,
    thistle,
    tomato,
    turquoise,
    violet,
    wheat,
    white,
    whitesmoke,
    yellow,
    yellowgreen,
}

impl From<RecognizedColor> for Rgba {
    fn from(value: RecognizedColor) -> Self {
        match value {
            RecognizedColor::aliceblue => Rgba::rgb(240, 248, 255),
            RecognizedColor::antiquewhite => Rgba::rgb(250, 235, 215),
            RecognizedColor::aqua => Rgba::rgb(0, 255, 255),
            RecognizedColor::aquamarine => Rgba::rgb(127, 255, 212),
            RecognizedColor::azure => Rgba::rgb(240, 255, 255),
            RecognizedColor::beige => Rgba::rgb(245, 245, 220),
            RecognizedColor::bisque => Rgba::rgb(255, 228, 196),
            RecognizedColor::black => Rgba::rgb(0, 0, 0),
            RecognizedColor::blanchedalmond => Rgba::rgb(255, 235, 205),
            RecognizedColor::blue => Rgba::rgb(0, 0, 255),
            RecognizedColor::blueviolet => Rgba::rgb(138, 43, 226),
            RecognizedColor::brown => Rgba::rgb(165, 42, 42),
            RecognizedColor::burlywood => Rgba::rgb(222, 184, 135),
            RecognizedColor::cadetblue => Rgba::rgb(95, 158, 160),
            RecognizedColor::chartreuse => Rgba::rgb(127, 255, 0),
            RecognizedColor::chocolate => Rgba::rgb(210, 105, 30),
            RecognizedColor::coral => Rgba::rgb(255, 127, 80),
            RecognizedColor::cornflowerblue => Rgba::rgb(100, 149, 237),
            RecognizedColor::cornsilk => Rgba::rgb(255, 248, 220),
            RecognizedColor::crimson => Rgba::rgb(220, 20, 60),
            RecognizedColor::cyan => Rgba::rgb(0, 255, 255),
            RecognizedColor::darkblue => Rgba::rgb(0, 0, 139),
            RecognizedColor::darkcyan => Rgba::rgb(0, 139, 139),
            RecognizedColor::darkgoldenrod => Rgba::rgb(184, 134, 11),
            RecognizedColor::darkgray => Rgba::rgb(169, 169, 169),
            RecognizedColor::darkgreen => Rgba::rgb(0, 100, 0),
            RecognizedColor::darkgrey => Rgba::rgb(169, 169, 169),
            RecognizedColor::darkkhaki => Rgba::rgb(189, 183, 107),
            RecognizedColor::darkmagenta => Rgba::rgb(139, 0, 139),
            RecognizedColor::darkolivegreen => Rgba::rgb(85, 107, 47),
            RecognizedColor::darkorange => Rgba::rgb(255, 140, 0),
            RecognizedColor::darkorchid => Rgba::rgb(153, 50, 204),
            RecognizedColor::darkred => Rgba::rgb(139, 0, 0),
            RecognizedColor::darksalmon => Rgba::rgb(233, 150, 122),
            RecognizedColor::darkseagreen => Rgba::rgb(143, 188, 143),
            RecognizedColor::darkslateblue => Rgba::rgb(72, 61, 139),
            RecognizedColor::darkslategray => Rgba::rgb(47, 79, 79),
            RecognizedColor::darkslategrey => Rgba::rgb(47, 79, 79),
            RecognizedColor::darkturquoise => Rgba::rgb(0, 206, 209),
            RecognizedColor::darkviolet => Rgba::rgb(148, 0, 211),
            RecognizedColor::deeppink => Rgba::rgb(255, 20, 147),
            RecognizedColor::deepskyblue => Rgba::rgb(0, 191, 255),
            RecognizedColor::dimgray => Rgba::rgb(105, 105, 105),
            RecognizedColor::dimgrey => Rgba::rgb(105, 105, 105),
            RecognizedColor::dodgerblue => Rgba::rgb(30, 144, 255),
            RecognizedColor::firebrick => Rgba::rgb(178, 34, 34),
            RecognizedColor::floralwhite => Rgba::rgb(255, 250, 240),
            RecognizedColor::forestgreen => Rgba::rgb(34, 139, 34),
            RecognizedColor::fuchsia => Rgba::rgb(255, 0, 255),
            RecognizedColor::gainsboro => Rgba::rgb(220, 220, 220),
            RecognizedColor::ghostwhite => Rgba::rgb(248, 248, 255),
            RecognizedColor::gold => Rgba::rgb(255, 215, 0),
            RecognizedColor::goldenrod => Rgba::rgb(218, 165, 32),
            RecognizedColor::gray => Rgba::rgb(128, 128, 128),
            RecognizedColor::grey => Rgba::rgb(128, 128, 128),
            RecognizedColor::green => Rgba::rgb(0, 128, 0),
            RecognizedColor::greenyellow => Rgba::rgb(173, 255, 47),
            RecognizedColor::honeydew => Rgba::rgb(240, 255, 240),
            RecognizedColor::hotpink => Rgba::rgb(255, 105, 180),
            RecognizedColor::indianred => Rgba::rgb(205, 92, 92),
            RecognizedColor::indigo => Rgba::rgb(75, 0, 130),
            RecognizedColor::ivory => Rgba::rgb(255, 255, 240),
            RecognizedColor::khaki => Rgba::rgb(240, 230, 140),
            RecognizedColor::lavender => Rgba::rgb(230, 230, 250),
            RecognizedColor::lavenderblush => Rgba::rgb(255, 240, 245),
            RecognizedColor::lawngreen => Rgba::rgb(124, 252, 0),
            RecognizedColor::lemonchiffon => Rgba::rgb(255, 250, 205),
            RecognizedColor::lightblue => Rgba::rgb(173, 216, 230),
            RecognizedColor::lightcoral => Rgba::rgb(240, 128, 128),
            RecognizedColor::lightcyan => Rgba::rgb(224, 255, 255),
            RecognizedColor::lightgoldenrodyellow => Rgba::rgb(250, 250, 210),
            RecognizedColor::lightgray => Rgba::rgb(211, 211, 211),
            RecognizedColor::lightgreen => Rgba::rgb(144, 238, 144),
            RecognizedColor::lightgrey => Rgba::rgb(211, 211, 211),
            RecognizedColor::lightpink => Rgba::rgb(255, 182, 193),
            RecognizedColor::lightsalmon => Rgba::rgb(255, 160, 122),
            RecognizedColor::lightseagreen => Rgba::rgb(32, 178, 170),
            RecognizedColor::lightskyblue => Rgba::rgb(135, 206, 250),
            RecognizedColor::lightslategray => Rgba::rgb(119, 136, 153),
            RecognizedColor::lightslategrey => Rgba::rgb(119, 136, 153),
            RecognizedColor::lightsteelblue => Rgba::rgb(176, 196, 222),
            RecognizedColor::lightyellow => Rgba::rgb(255, 255, 224),
            RecognizedColor::lime => Rgba::rgb(0, 255, 0),
            RecognizedColor::limegreen => Rgba::rgb(50, 205, 50),
            RecognizedColor::linen => Rgba::rgb(250, 240, 230),
            RecognizedColor::magenta => Rgba::rgb(255, 0, 255),
            RecognizedColor::maroon => Rgba::rgb(128, 0, 0),
            RecognizedColor::mediumaquamarine => Rgba::rgb(102, 205, 170),
            RecognizedColor::mediumblue => Rgba::rgb(0, 0, 205),
            RecognizedColor::mediumorchid => Rgba::rgb(186, 85, 211),
            RecognizedColor::mediumpurple => Rgba::rgb(147, 112, 219),
            RecognizedColor::mediumseagreen => Rgba::rgb(60, 179, 113),
            RecognizedColor::mediumslateblue => Rgba::rgb(123, 104, 238),
            RecognizedColor::mediumspringgreen => Rgba::rgb(0, 250, 154),
            RecognizedColor::mediumturquoise => Rgba::rgb(72, 209, 204),
            RecognizedColor::mediumvioletred => Rgba::rgb(199, 21, 133),
            RecognizedColor::midnightblue => Rgba::rgb(25, 25, 112),
            RecognizedColor::mintcream => Rgba::rgb(245, 255, 250),
            RecognizedColor::mistyrose => Rgba::rgb(255, 228, 225),
            RecognizedColor::moccasin => Rgba::rgb(255, 228, 181),
            RecognizedColor::navajowhite => Rgba::rgb(255, 222, 173),
            RecognizedColor::navy => Rgba::rgb(0, 0, 128),
            RecognizedColor::oldlace => Rgba::rgb(253, 245, 230),
            RecognizedColor::olive => Rgba::rgb(128, 128, 0),
            RecognizedColor::olivedrab => Rgba::rgb(107, 142, 35),
            RecognizedColor::orange => Rgba::rgb(255, 165, 0),
            RecognizedColor::orangered => Rgba::rgb(255, 69, 0),
            RecognizedColor::orchid => Rgba::rgb(218, 112, 214),
            RecognizedColor::palegoldenrod => Rgba::rgb(238, 232, 170),
            RecognizedColor::palegreen => Rgba::rgb(152, 251, 152),
            RecognizedColor::paleturquoise => Rgba::rgb(175, 238, 238),
            RecognizedColor::palevioletred => Rgba::rgb(219, 112, 147),
            RecognizedColor::papayawhip => Rgba::rgb(255, 239, 213),
            RecognizedColor::peachpuff => Rgba::rgb(255, 218, 185),
            RecognizedColor::peru => Rgba::rgb(205, 133, 63),
            RecognizedColor::pink => Rgba::rgb(255, 192, 203),
            RecognizedColor::plum => Rgba::rgb(221, 160, 221),
            RecognizedColor::powderblue => Rgba::rgb(176, 224, 230),
            RecognizedColor::purple => Rgba::rgb(128, 0, 128),
            RecognizedColor::red => Rgba::rgb(255, 0, 0),
            RecognizedColor::rosybrown => Rgba::rgb(188, 143, 143),
            RecognizedColor::royalblue => Rgba::rgb(65, 105, 225),
            RecognizedColor::saddlebrown => Rgba::rgb(139, 69, 19),
            RecognizedColor::salmon => Rgba::rgb(250, 128, 114),
            RecognizedColor::sandybrown => Rgba::rgb(244, 164, 96),
            RecognizedColor::seagreen => Rgba::rgb(46, 139, 87),
            RecognizedColor::seashell => Rgba::rgb(255, 245, 238),
            RecognizedColor::sienna => Rgba::rgb(160, 82, 45),
            RecognizedColor::silver => Rgba::rgb(192, 192, 192),
            RecognizedColor::skyblue => Rgba::rgb(135, 206, 235),
            RecognizedColor::slateblue => Rgba::rgb(106, 90, 205),
            RecognizedColor::slategray => Rgba::rgb(112, 128, 144),
            RecognizedColor::slategrey => Rgba::rgb(112, 128, 144),
            RecognizedColor::snow => Rgba::rgb(255, 250, 250),
            RecognizedColor::springgreen => Rgba::rgb(0, 255, 127),
            RecognizedColor::steelblue => Rgba::rgb(70, 130, 180),
            RecognizedColor::tan => Rgba::rgb(210, 180, 140),
            RecognizedColor::teal => Rgba::rgb(0, 128, 128),
            RecognizedColor::thistle => Rgba::rgb(216, 191, 216),
            RecognizedColor::tomato => Rgba::rgb(255, 99, 71),
            RecognizedColor::turquoise => Rgba::rgb(64, 224, 208),
            RecognizedColor::violet => Rgba::rgb(238, 130, 238),
            RecognizedColor::wheat => Rgba::rgb(245, 222, 179),
            RecognizedColor::white => Rgba::rgb(255, 255, 255),
            RecognizedColor::whitesmoke => Rgba::rgb(245, 245, 245),
            RecognizedColor::yellow => Rgba::rgb(255, 255, 0),
            RecognizedColor::yellowgreen => Rgba::rgb(154, 205, 50),
        }
    }
}
