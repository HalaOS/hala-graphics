use std::{fmt::Display, str::FromStr};

/// The unit identifier.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Unit {
    /// The 'font-size' of the relevant font
    Em,
    /// The 'ex' unit is defined by the font's 'x-height'.
    Ex,
    /// pixels, relative to the viewing device
    Px,
    /// 1 inch is equal to 2.54 centimeters.
    In,
    /// centimeters
    Cm,
    /// millimeters
    Mm,
    /// the points used by CSS2 are equal to 1/72th of an inch.
    Pt,
    /// 1 pica is equal to 12 points.
    Pc,
    /// The meaning of a percentage length value depends on the attribute for which the percentage length value has been specified.
    Percentages,
}

impl FromStr for Unit {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "em" => Ok(Unit::Em),
            "ex" => Ok(Unit::Ex),
            "px" => Ok(Unit::Px),
            "in" => Ok(Unit::In),
            "cm" => Ok(Unit::Cm),
            "mm" => Ok(Unit::Mm),
            "pt" => Ok(Unit::Pt),
            "pc" => Ok(Unit::Pc),
            "%" => Ok(Unit::Percentages),
            _ => Err(crate::Error::LengthStr(s.to_owned())),
        }
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for Unit {
    fn as_ref(&self) -> &str {
        match self {
            Unit::Em => "em",
            Unit::Ex => "ex",
            Unit::Px => "px",
            Unit::In => "in",
            Unit::Cm => "cm",
            Unit::Mm => "mm",
            Unit::Pt => "pt",
            Unit::Pc => "pc",
            Unit::Percentages => "%",
        }
    }
}
