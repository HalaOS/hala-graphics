use std::{fmt::Display, str::FromStr};

use regex::RegexBuilder;

/// A color structure repesents as RGBA, the storage value is normalized.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rgba(f32, f32, f32, f32);

impl From<Rgba> for [f32; 4] {
    fn from(value: Rgba) -> Self {
        [value.0, value.1, value.2, value.3]
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({},{},{},{})", self.0, self.1, self.2, self.3)
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

impl FromStr for Rgba {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = Recognized::from_str(s) {
            return Ok(value.into());
        }

        let r = RegexBuilder::new(
            r"(#(?<hex>[0-9A-Fa-f]{3,6}))|(rgb\(\s*(?<r>\d{1,3})\s*,\s*(?<g>\d{1,3})\s*,\s*(?<b>\d{1,3})\s*\))|(rgb\(\s*(?<rp>\d{1,3})%\s*,\s*(?<gp>\d{1,3})%\s*,\s*(?<bp>\d{1,3})%\s*\))",
        )
        .case_insensitive(true)
        .build()
        .unwrap();

        let capture = r
            .captures(s)
            .ok_or(crate::Error::UnrecognizedColor(s.to_string()))?;

        if let Some(rgba) = capture.name("hex") {
            let rgba = if rgba.len() == 3 {
                let rgba = rgba.as_str();

                format!(
                    "{}{}{}{}{}{}",
                    &rgba[0..1],
                    &rgba[0..1],
                    &rgba[1..2],
                    &rgba[1..2],
                    &rgba[2..],
                    &rgba[2..],
                )
            } else {
                rgba.as_str().to_string()
            };
            let hex = u32::from_str_radix(rgba.as_str(), 16)?;

            Ok(Rgba::rgb(
                ((hex >> 16) & 0xff) as u8,
                ((hex >> 8) & 0xff) as u8,
                (hex & 0xff) as u8,
            ))
        } else if capture.name("r").is_some() {
            let r = u8::from_str(capture.name("r").unwrap().as_str())?;
            let g = u8::from_str(capture.name("g").unwrap().as_str())?;
            let b = u8::from_str(capture.name("b").unwrap().as_str())?;

            Ok(Rgba::rgb(r, g, b))
        } else if capture.name("rp").is_some() {
            let r = u8::from_str(capture.name("rp").unwrap().as_str())? as f32 / 100f32;
            let g = u8::from_str(capture.name("gp").unwrap().as_str())? as f32 / 100f32;
            let b = u8::from_str(capture.name("bp").unwrap().as_str())? as f32 / 100f32;

            Ok(Rgba::rgbf(r, g, b))
        } else {
            return Err(crate::Error::UnrecognizedColor(s.to_owned()));
        }
    }
}

/// Recognized color keyword names, compliant with svg 1.1.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Recognized {
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

impl FromStr for Recognized {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aliceblue" => Ok(Self::aliceblue),
            "antiquewhite" => Ok(Self::antiquewhite),
            "aqua" => Ok(Self::aqua),
            "aquamarine" => Ok(Self::aquamarine),
            "azure" => Ok(Self::azure),
            "beige" => Ok(Self::beige),
            "bisque" => Ok(Self::bisque),
            "black" => Ok(Self::black),
            "blanchedalmond" => Ok(Self::blanchedalmond),
            "blue" => Ok(Self::blue),
            "blueviolet" => Ok(Self::blueviolet),
            "brown" => Ok(Self::brown),
            "burlywood" => Ok(Self::burlywood),
            "cadetblue" => Ok(Self::cadetblue),
            "chartreuse" => Ok(Self::chartreuse),
            "chocolate" => Ok(Self::chocolate),
            "coral" => Ok(Self::coral),
            "cornflowerblue" => Ok(Self::cornflowerblue),
            "cornsilk" => Ok(Self::cornsilk),
            "crimson" => Ok(Self::crimson),
            "cyan" => Ok(Self::cyan),
            "darkblue" => Ok(Self::darkblue),
            "darkcyan" => Ok(Self::darkcyan),
            "darkgoldenrod" => Ok(Self::darkgoldenrod),
            "darkgray" => Ok(Self::darkgray),
            "darkgreen" => Ok(Self::darkgreen),
            "darkgrey" => Ok(Self::darkgrey),
            "darkkhaki" => Ok(Self::darkkhaki),
            "darkmagenta" => Ok(Self::darkmagenta),
            "darkolivegreen" => Ok(Self::darkolivegreen),
            "darkorange" => Ok(Self::darkorange),
            "darkorchid" => Ok(Self::darkorchid),
            "darkred" => Ok(Self::darkred),
            "darksalmon" => Ok(Self::darksalmon),
            "darkseagreen" => Ok(Self::darkseagreen),
            "darkslateblue" => Ok(Self::darkslateblue),
            "darkslategray" => Ok(Self::darkslategray),
            "darkslategrey" => Ok(Self::darkslategrey),
            "darkturquoise" => Ok(Self::darkturquoise),
            "darkviolet" => Ok(Self::darkviolet),
            "deeppink" => Ok(Self::deeppink),
            "deepskyblue" => Ok(Self::deepskyblue),
            "dimgray" => Ok(Self::dimgray),
            "dimgrey" => Ok(Self::dimgrey),
            "dodgerblue" => Ok(Self::dodgerblue),
            "firebrick" => Ok(Self::firebrick),
            "floralwhite" => Ok(Self::floralwhite),
            "forestgreen" => Ok(Self::forestgreen),
            "fuchsia" => Ok(Self::fuchsia),
            "gainsboro" => Ok(Self::gainsboro),
            "ghostwhite" => Ok(Self::ghostwhite),
            "gold" => Ok(Self::gold),
            "goldenrod" => Ok(Self::goldenrod),
            "gray" => Ok(Self::gray),
            "grey" => Ok(Self::grey),
            "green" => Ok(Self::green),
            "greenyellow" => Ok(Self::greenyellow),
            "honeydew" => Ok(Self::honeydew),
            "hotpink" => Ok(Self::hotpink),
            "indianred" => Ok(Self::indianred),
            "indigo" => Ok(Self::indigo),
            "ivory" => Ok(Self::ivory),
            "khaki" => Ok(Self::khaki),
            "lavender" => Ok(Self::lavender),
            "lavenderblush" => Ok(Self::lavenderblush),
            "lawngreen" => Ok(Self::lawngreen),
            "lemonchiffon" => Ok(Self::lemonchiffon),
            "lightblue" => Ok(Self::lightblue),
            "lightcoral" => Ok(Self::lightcoral),
            "lightcyan" => Ok(Self::lightcyan),
            "lightgoldenrodyellow" => Ok(Self::lightgoldenrodyellow),
            "lightgray" => Ok(Self::lightgray),
            "lightgreen" => Ok(Self::lightgreen),
            "lightgrey" => Ok(Self::lightgrey),
            "lightpink" => Ok(Self::lightpink),
            "lightsalmon" => Ok(Self::lightsalmon),
            "lightseagreen" => Ok(Self::lightseagreen),
            "lightskyblue" => Ok(Self::lightskyblue),
            "lightslategray" => Ok(Self::lightslategray),
            "lightslategrey" => Ok(Self::lightslategrey),
            "lightsteelblue" => Ok(Self::lightsteelblue),
            "lightyellow" => Ok(Self::lightyellow),
            "lime" => Ok(Self::lime),
            "limegreen" => Ok(Self::limegreen),
            "linen" => Ok(Self::linen),
            "magenta" => Ok(Self::magenta),
            "maroon" => Ok(Self::maroon),
            "mediumaquamarine" => Ok(Self::mediumaquamarine),
            "mediumblue" => Ok(Self::mediumblue),
            "mediumorchid" => Ok(Self::mediumorchid),
            "mediumpurple" => Ok(Self::mediumpurple),
            "mediumseagreen" => Ok(Self::mediumseagreen),
            "mediumslateblue" => Ok(Self::mediumslateblue),
            "mediumspringgreen" => Ok(Self::mediumspringgreen),
            "mediumturquoise" => Ok(Self::mediumturquoise),
            "mediumvioletred" => Ok(Self::mediumvioletred),
            "midnightblue" => Ok(Self::midnightblue),
            "mintcream" => Ok(Self::mintcream),
            "mistyrose" => Ok(Self::mistyrose),
            "moccasin" => Ok(Self::moccasin),
            "navajowhite" => Ok(Self::navajowhite),
            "navy" => Ok(Self::navy),
            "oldlace" => Ok(Self::oldlace),
            "olive" => Ok(Self::olive),
            "olivedrab" => Ok(Self::olivedrab),
            "orange" => Ok(Self::orange),
            "orangered" => Ok(Self::orangered),
            "orchid" => Ok(Self::orchid),
            "palegoldenrod" => Ok(Self::palegoldenrod),
            "palegreen" => Ok(Self::palegreen),
            "paleturquoise" => Ok(Self::paleturquoise),
            "palevioletred" => Ok(Self::palevioletred),
            "papayawhip" => Ok(Self::papayawhip),
            "peachpuff" => Ok(Self::peachpuff),
            "peru" => Ok(Self::peru),
            "pink" => Ok(Self::pink),
            "plum" => Ok(Self::plum),
            "powderblue" => Ok(Self::powderblue),
            "purple" => Ok(Self::purple),
            "red" => Ok(Self::red),
            "rosybrown" => Ok(Self::rosybrown),
            "royalblue" => Ok(Self::royalblue),
            "saddlebrown" => Ok(Self::saddlebrown),
            "salmon" => Ok(Self::salmon),
            "sandybrown" => Ok(Self::sandybrown),
            "seagreen" => Ok(Self::seagreen),
            "seashell" => Ok(Self::seashell),
            "sienna" => Ok(Self::sienna),
            "silver" => Ok(Self::silver),
            "skyblue" => Ok(Self::skyblue),
            "slateblue" => Ok(Self::slateblue),
            "slategray" => Ok(Self::slategray),
            "slategrey" => Ok(Self::slategrey),
            "snow" => Ok(Self::snow),
            "springgreen" => Ok(Self::springgreen),
            "steelblue" => Ok(Self::steelblue),
            "tan" => Ok(Self::tan),
            "teal" => Ok(Self::teal),
            "thistle" => Ok(Self::thistle),
            "tomato" => Ok(Self::tomato),
            "turquoise" => Ok(Self::turquoise),
            "violet" => Ok(Self::violet),
            "wheat" => Ok(Self::wheat),
            "white" => Ok(Self::white),
            "whitesmoke" => Ok(Self::whitesmoke),
            "yellow" => Ok(Self::yellow),
            "yellowgreen" => Ok(Self::yellowgreen),
            _ => return Err(crate::Error::UnrecognizedColor(s.to_owned())),
        }
    }
}

impl From<Recognized> for Rgba {
    fn from(value: Recognized) -> Self {
        match value {
            Recognized::aliceblue => Rgba::rgb(240, 248, 255),
            Recognized::antiquewhite => Rgba::rgb(250, 235, 215),
            Recognized::aqua => Rgba::rgb(0, 255, 255),
            Recognized::aquamarine => Rgba::rgb(127, 255, 212),
            Recognized::azure => Rgba::rgb(240, 255, 255),
            Recognized::beige => Rgba::rgb(245, 245, 220),
            Recognized::bisque => Rgba::rgb(255, 228, 196),
            Recognized::black => Rgba::rgb(0, 0, 0),
            Recognized::blanchedalmond => Rgba::rgb(255, 235, 205),
            Recognized::blue => Rgba::rgb(0, 0, 255),
            Recognized::blueviolet => Rgba::rgb(138, 43, 226),
            Recognized::brown => Rgba::rgb(165, 42, 42),
            Recognized::burlywood => Rgba::rgb(222, 184, 135),
            Recognized::cadetblue => Rgba::rgb(95, 158, 160),
            Recognized::chartreuse => Rgba::rgb(127, 255, 0),
            Recognized::chocolate => Rgba::rgb(210, 105, 30),
            Recognized::coral => Rgba::rgb(255, 127, 80),
            Recognized::cornflowerblue => Rgba::rgb(100, 149, 237),
            Recognized::cornsilk => Rgba::rgb(255, 248, 220),
            Recognized::crimson => Rgba::rgb(220, 20, 60),
            Recognized::cyan => Rgba::rgb(0, 255, 255),
            Recognized::darkblue => Rgba::rgb(0, 0, 139),
            Recognized::darkcyan => Rgba::rgb(0, 139, 139),
            Recognized::darkgoldenrod => Rgba::rgb(184, 134, 11),
            Recognized::darkgray => Rgba::rgb(169, 169, 169),
            Recognized::darkgreen => Rgba::rgb(0, 100, 0),
            Recognized::darkgrey => Rgba::rgb(169, 169, 169),
            Recognized::darkkhaki => Rgba::rgb(189, 183, 107),
            Recognized::darkmagenta => Rgba::rgb(139, 0, 139),
            Recognized::darkolivegreen => Rgba::rgb(85, 107, 47),
            Recognized::darkorange => Rgba::rgb(255, 140, 0),
            Recognized::darkorchid => Rgba::rgb(153, 50, 204),
            Recognized::darkred => Rgba::rgb(139, 0, 0),
            Recognized::darksalmon => Rgba::rgb(233, 150, 122),
            Recognized::darkseagreen => Rgba::rgb(143, 188, 143),
            Recognized::darkslateblue => Rgba::rgb(72, 61, 139),
            Recognized::darkslategray => Rgba::rgb(47, 79, 79),
            Recognized::darkslategrey => Rgba::rgb(47, 79, 79),
            Recognized::darkturquoise => Rgba::rgb(0, 206, 209),
            Recognized::darkviolet => Rgba::rgb(148, 0, 211),
            Recognized::deeppink => Rgba::rgb(255, 20, 147),
            Recognized::deepskyblue => Rgba::rgb(0, 191, 255),
            Recognized::dimgray => Rgba::rgb(105, 105, 105),
            Recognized::dimgrey => Rgba::rgb(105, 105, 105),
            Recognized::dodgerblue => Rgba::rgb(30, 144, 255),
            Recognized::firebrick => Rgba::rgb(178, 34, 34),
            Recognized::floralwhite => Rgba::rgb(255, 250, 240),
            Recognized::forestgreen => Rgba::rgb(34, 139, 34),
            Recognized::fuchsia => Rgba::rgb(255, 0, 255),
            Recognized::gainsboro => Rgba::rgb(220, 220, 220),
            Recognized::ghostwhite => Rgba::rgb(248, 248, 255),
            Recognized::gold => Rgba::rgb(255, 215, 0),
            Recognized::goldenrod => Rgba::rgb(218, 165, 32),
            Recognized::gray => Rgba::rgb(128, 128, 128),
            Recognized::grey => Rgba::rgb(128, 128, 128),
            Recognized::green => Rgba::rgb(0, 128, 0),
            Recognized::greenyellow => Rgba::rgb(173, 255, 47),
            Recognized::honeydew => Rgba::rgb(240, 255, 240),
            Recognized::hotpink => Rgba::rgb(255, 105, 180),
            Recognized::indianred => Rgba::rgb(205, 92, 92),
            Recognized::indigo => Rgba::rgb(75, 0, 130),
            Recognized::ivory => Rgba::rgb(255, 255, 240),
            Recognized::khaki => Rgba::rgb(240, 230, 140),
            Recognized::lavender => Rgba::rgb(230, 230, 250),
            Recognized::lavenderblush => Rgba::rgb(255, 240, 245),
            Recognized::lawngreen => Rgba::rgb(124, 252, 0),
            Recognized::lemonchiffon => Rgba::rgb(255, 250, 205),
            Recognized::lightblue => Rgba::rgb(173, 216, 230),
            Recognized::lightcoral => Rgba::rgb(240, 128, 128),
            Recognized::lightcyan => Rgba::rgb(224, 255, 255),
            Recognized::lightgoldenrodyellow => Rgba::rgb(250, 250, 210),
            Recognized::lightgray => Rgba::rgb(211, 211, 211),
            Recognized::lightgreen => Rgba::rgb(144, 238, 144),
            Recognized::lightgrey => Rgba::rgb(211, 211, 211),
            Recognized::lightpink => Rgba::rgb(255, 182, 193),
            Recognized::lightsalmon => Rgba::rgb(255, 160, 122),
            Recognized::lightseagreen => Rgba::rgb(32, 178, 170),
            Recognized::lightskyblue => Rgba::rgb(135, 206, 250),
            Recognized::lightslategray => Rgba::rgb(119, 136, 153),
            Recognized::lightslategrey => Rgba::rgb(119, 136, 153),
            Recognized::lightsteelblue => Rgba::rgb(176, 196, 222),
            Recognized::lightyellow => Rgba::rgb(255, 255, 224),
            Recognized::lime => Rgba::rgb(0, 255, 0),
            Recognized::limegreen => Rgba::rgb(50, 205, 50),
            Recognized::linen => Rgba::rgb(250, 240, 230),
            Recognized::magenta => Rgba::rgb(255, 0, 255),
            Recognized::maroon => Rgba::rgb(128, 0, 0),
            Recognized::mediumaquamarine => Rgba::rgb(102, 205, 170),
            Recognized::mediumblue => Rgba::rgb(0, 0, 205),
            Recognized::mediumorchid => Rgba::rgb(186, 85, 211),
            Recognized::mediumpurple => Rgba::rgb(147, 112, 219),
            Recognized::mediumseagreen => Rgba::rgb(60, 179, 113),
            Recognized::mediumslateblue => Rgba::rgb(123, 104, 238),
            Recognized::mediumspringgreen => Rgba::rgb(0, 250, 154),
            Recognized::mediumturquoise => Rgba::rgb(72, 209, 204),
            Recognized::mediumvioletred => Rgba::rgb(199, 21, 133),
            Recognized::midnightblue => Rgba::rgb(25, 25, 112),
            Recognized::mintcream => Rgba::rgb(245, 255, 250),
            Recognized::mistyrose => Rgba::rgb(255, 228, 225),
            Recognized::moccasin => Rgba::rgb(255, 228, 181),
            Recognized::navajowhite => Rgba::rgb(255, 222, 173),
            Recognized::navy => Rgba::rgb(0, 0, 128),
            Recognized::oldlace => Rgba::rgb(253, 245, 230),
            Recognized::olive => Rgba::rgb(128, 128, 0),
            Recognized::olivedrab => Rgba::rgb(107, 142, 35),
            Recognized::orange => Rgba::rgb(255, 165, 0),
            Recognized::orangered => Rgba::rgb(255, 69, 0),
            Recognized::orchid => Rgba::rgb(218, 112, 214),
            Recognized::palegoldenrod => Rgba::rgb(238, 232, 170),
            Recognized::palegreen => Rgba::rgb(152, 251, 152),
            Recognized::paleturquoise => Rgba::rgb(175, 238, 238),
            Recognized::palevioletred => Rgba::rgb(219, 112, 147),
            Recognized::papayawhip => Rgba::rgb(255, 239, 213),
            Recognized::peachpuff => Rgba::rgb(255, 218, 185),
            Recognized::peru => Rgba::rgb(205, 133, 63),
            Recognized::pink => Rgba::rgb(255, 192, 203),
            Recognized::plum => Rgba::rgb(221, 160, 221),
            Recognized::powderblue => Rgba::rgb(176, 224, 230),
            Recognized::purple => Rgba::rgb(128, 0, 128),
            Recognized::red => Rgba::rgb(255, 0, 0),
            Recognized::rosybrown => Rgba::rgb(188, 143, 143),
            Recognized::royalblue => Rgba::rgb(65, 105, 225),
            Recognized::saddlebrown => Rgba::rgb(139, 69, 19),
            Recognized::salmon => Rgba::rgb(250, 128, 114),
            Recognized::sandybrown => Rgba::rgb(244, 164, 96),
            Recognized::seagreen => Rgba::rgb(46, 139, 87),
            Recognized::seashell => Rgba::rgb(255, 245, 238),
            Recognized::sienna => Rgba::rgb(160, 82, 45),
            Recognized::silver => Rgba::rgb(192, 192, 192),
            Recognized::skyblue => Rgba::rgb(135, 206, 235),
            Recognized::slateblue => Rgba::rgb(106, 90, 205),
            Recognized::slategray => Rgba::rgb(112, 128, 144),
            Recognized::slategrey => Rgba::rgb(112, 128, 144),
            Recognized::snow => Rgba::rgb(255, 250, 250),
            Recognized::springgreen => Rgba::rgb(0, 255, 127),
            Recognized::steelblue => Rgba::rgb(70, 130, 180),
            Recognized::tan => Rgba::rgb(210, 180, 140),
            Recognized::teal => Rgba::rgb(0, 128, 128),
            Recognized::thistle => Rgba::rgb(216, 191, 216),
            Recognized::tomato => Rgba::rgb(255, 99, 71),
            Recognized::turquoise => Rgba::rgb(64, 224, 208),
            Recognized::violet => Rgba::rgb(238, 130, 238),
            Recognized::wheat => Rgba::rgb(245, 222, 179),
            Recognized::white => Rgba::rgb(255, 255, 255),
            Recognized::whitesmoke => Rgba::rgb(245, 245, 245),
            Recognized::yellow => Rgba::rgb(255, 255, 0),
            Recognized::yellowgreen => Rgba::rgb(154, 205, 50),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Recognized;

    use super::Rgba;

    #[test]
    fn color_parse() {
        assert_eq!("#fff".parse::<Rgba>().unwrap(), Rgba::rgb(255, 255, 255));

        assert_eq!("#ffffff".parse::<Rgba>().unwrap(), Rgba::rgb(255, 255, 255));

        assert_eq!("#ffff00".parse::<Rgba>().unwrap(), Rgba::rgb(255, 255, 0));

        assert_eq!("#ff0".parse::<Rgba>().unwrap(), Rgba::rgb(255, 255, 0));

        assert_eq!(
            "rgb(128,102,100)".parse::<Rgba>().unwrap(),
            Rgba::rgb(128, 102, 100)
        );

        assert_eq!(
            "rgb( 128,102,100 )".parse::<Rgba>().unwrap(),
            Rgba::rgb(128, 102, 100)
        );

        assert_eq!(
            "rGb(   128,102,100 )".parse::<Rgba>().unwrap(),
            Rgba::rgb(128, 102, 100)
        );

        assert_eq!(
            "rGb(10%,100% , 29% )".parse::<Rgba>().unwrap(),
            Rgba::rgbf(0.1, 1.0, 0.29)
        );

        assert_eq!(
            "mintcream".parse::<Rgba>().unwrap(),
            Recognized::mintcream.into()
        );
    }
}
