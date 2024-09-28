use std::{fmt::Display, str::FromStr};

/// The unit identifier.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LengthUnit {
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

impl FromStr for LengthUnit {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "em" => Ok(LengthUnit::Em),
            "ex" => Ok(LengthUnit::Ex),
            "px" => Ok(LengthUnit::Px),
            "in" => Ok(LengthUnit::In),
            "cm" => Ok(LengthUnit::Cm),
            "mm" => Ok(LengthUnit::Mm),
            "pt" => Ok(LengthUnit::Pt),
            "pc" => Ok(LengthUnit::Pc),
            "%" => Ok(LengthUnit::Percentages),
            _ => Err(crate::Error::LengthStr(s.to_owned())),
        }
    }
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl AsRef<str> for LengthUnit {
    fn as_ref(&self) -> &str {
        match self {
            LengthUnit::Em => "em",
            LengthUnit::Ex => "ex",
            LengthUnit::Px => "px",
            LengthUnit::In => "in",
            LengthUnit::Cm => "cm",
            LengthUnit::Mm => "mm",
            LengthUnit::Pt => "pt",
            LengthUnit::Pc => "pc",
            LengthUnit::Percentages => "%",
        }
    }
}

/// A length is a distance measurement, given as a number along with a unit which may be optional.
/// The unit identifier, if present, must be in lower case; if not present,
/// the length value represents a distance in the current user coordinate system.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Length(pub f32, pub LengthUnit);

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1.as_ref())
    }
}

impl FromStr for Length {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn is_digit(c: char) -> bool {
            if c < '0' {
                return false;
            }

            if c > '9' {
                return false;
            }

            true
        }

        let suffix_offset = s
            .chars()
            .enumerate()
            .find_map(|(index, c)| if is_digit(c) { None } else { Some(index) })
            .unwrap_or(s.len());

        let value = f32::from_str(&s[..suffix_offset])?;
        let unit = LengthUnit::from_str(&s[suffix_offset..])?;

        Ok(Self(value, unit))
    }
}

impl Length {
    /// Create a length with `em` unit identifier.
    pub fn em(value: f32) -> Self {
        Self(value, LengthUnit::Em)
    }

    /// Create a length with `em` unit identifier.
    pub fn ex(value: f32) -> Self {
        Self(value, LengthUnit::Ex)
    }

    /// Create a length with `px` unit identifier.
    pub fn px(value: f32) -> Self {
        Self(value, LengthUnit::Px)
    }

    /// Create a length with `inch` unit identifier.
    pub fn inch(value: f32) -> Self {
        Self(value, LengthUnit::In)
    }
    /// Create a length with `cm` unit identifier.
    pub fn cm(value: f32) -> Self {
        Self(value, LengthUnit::Cm)
    }
    /// Create a length with `mm` unit identifier.
    pub fn mm(value: f32) -> Self {
        Self(value, LengthUnit::Mm)
    }
    /// Create a length with `pt` unit identifier.
    pub fn pt(value: f32) -> Self {
        Self(value, LengthUnit::Pt)
    }
    /// Create a length with `pc` unit identifier.
    pub fn pc(value: f32) -> Self {
        Self(value, LengthUnit::Pc)
    }

    /// Create a length with `px` unit identifier.
    pub fn percentage(value: f32) -> Self {
        Self(value, LengthUnit::Percentages)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Length;

    #[test]
    fn display() {
        let str = Length::cm(100.0).to_string();

        assert_eq!(str.parse::<Length>().unwrap(), Length::cm(100.0));

        assert_eq!("99%".parse::<Length>().unwrap(), Length::percentage(99.0));
    }
}
