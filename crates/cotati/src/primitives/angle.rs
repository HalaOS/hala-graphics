use std::{fmt::Display, str::FromStr};

use nom::{branch::alt, bytes::complete::tag, combinator::map, Err};

use crate::Error;

/// Angles are specified in one of two ways depending upon
/// whether they are used in CSS property syntax or SVG
/// presentation attribute syntax:
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Angle {
    deg(f32),
    grad(f32),
    rad(f32),
}

impl FromStr for Angle {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, value) = nom::number::complete::float(s)
            .map_err(|_: Err<nom::error::Error<&str>>| Error::Angle(s.to_owned()))?;

        let (_, value) = alt((
            map(tag("deg"), |_| Angle::deg(value)),
            map(tag("grad"), |_| Angle::grad(value)),
            map(tag("rad"), |_| Angle::rad(value)),
        ))(input)
        .map_err(|_: Err<nom::error::Error<&str>>| Error::Angle(s.to_owned()))?;

        Ok(value)
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Angle::deg(v) => write!(f, "{}deg", v),
            Angle::grad(v) => write!(f, "{}grad", v),
            Angle::rad(v) => write!(f, "{}rad", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!("360deg".parse::<Angle>().unwrap(), Angle::deg(360.0));

        assert_eq!("14.24deg".parse::<Angle>().unwrap(), Angle::deg(14.24));

        assert_eq!("400grad".parse::<Angle>().unwrap(), Angle::grad(400.0));

        assert_eq!("38.8grad".parse::<Angle>().unwrap(), Angle::grad(38.8));

        assert_eq!("6.2832rad".parse::<Angle>().unwrap(), Angle::rad(6.2832));

        assert_eq!("1rad".parse::<Angle>().unwrap(), Angle::rad(1.0));
    }
}
