use std::{fmt::Display, str::FromStr};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::tuple,
    Err,
};

use crate::Error;

use super::Unit;

/// A length is a distance measurement, given as a number along with a unit.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Length(pub f32, pub Option<Unit>);

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unit) = self.1 {
            write!(f, "{}{}", self.0, unit)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl FromStr for Length {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (value, unit)) = tuple((
            nom::number::complete::float,
            opt(alt((
                map(tag("em"), |_| Unit::Em),
                map(tag("ex"), |_| Unit::Ex),
                map(tag("px"), |_| Unit::Px),
                map(tag("in"), |_| Unit::In),
                map(tag("cm"), |_| Unit::Cm),
                map(tag("mm"), |_| Unit::Mm),
                map(tag("pt"), |_| Unit::Pt),
                map(tag("pc"), |_| Unit::Pc),
                map(tag("%"), |_| Unit::Percentages),
            ))),
        ))(s)
        .map_err(|_: Err<nom::error::Error<&str>>| Error::LengthStr(s.to_owned()))?;

        Ok(Self(value, unit))
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Self(value, None)
    }
}

impl Length {
    /// Create a length with `em` unit identifier.
    pub fn em(value: f32) -> Self {
        Self(value, Some(Unit::Em))
    }

    /// Create a length with `em` unit identifier.
    pub fn ex(value: f32) -> Self {
        Self(value, Some(Unit::Ex))
    }

    /// Create a length with `px` unit identifier.
    pub fn px(value: f32) -> Self {
        Self(value, Some(Unit::Px))
    }

    /// Create a length with `inch` unit identifier.
    pub fn inch(value: f32) -> Self {
        Self(value, Some(Unit::In))
    }
    /// Create a length with `cm` unit identifier.
    pub fn cm(value: f32) -> Self {
        Self(value, Some(Unit::Cm))
    }
    /// Create a length with `mm` unit identifier.
    pub fn mm(value: f32) -> Self {
        Self(value, Some(Unit::Mm))
    }
    /// Create a length with `pt` unit identifier.
    pub fn pt(value: f32) -> Self {
        Self(value, Some(Unit::Pt))
    }
    /// Create a length with `pc` unit identifier.
    pub fn pc(value: f32) -> Self {
        Self(value, Some(Unit::Pc))
    }

    /// Create a length with `px` unit identifier.
    pub fn percentage(value: f32) -> Self {
        Self(value, Some(Unit::Percentages))
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Length;

    #[test]
    fn display() {
        let str = Length::cm(100.111).to_string();

        assert_eq!(str.parse::<Length>().unwrap(), Length::cm(100.111));

        assert_eq!("99.1%".parse::<Length>().unwrap(), Length::percentage(99.1));
    }
}
