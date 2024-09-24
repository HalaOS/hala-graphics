use std::fmt::Display;

/// The unit identifier.
#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LengthUnit {
    /// The 'font-size' of the relevant font
    em,
    /// The 'ex' unit is defined by the font's 'x-height'.
    ex,
    /// pixels, relative to the viewing device
    px,
    /// 1 inch is equal to 2.54 centimeters.
    inch,
    /// centimeters
    cm,
    /// millimeters
    mm,
    /// the points used by CSS2 are equal to 1/72th of an inch.
    pt,
    /// 1 pica is equal to 12 points.
    pc,
    /// The meaning of a percentage length value depends on the attribute for which the percentage length value has been specified.
    percentage,
}

/// A length is a distance measurement, given as a number along with a unit which may be optional.
/// The unit identifier, if present, must be in lower case; if not present,
/// the length value represents a distance in the current user coordinate system.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Length(pub u32, pub LengthUnit);

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.0, self.1)
    }
}

impl Length {
    /// Create a length with `em` unit identifier.
    pub fn em(value: u32) -> Self {
        Self(value, LengthUnit::em)
    }

    /// Create a length with `em` unit identifier.
    pub fn ex(value: u32) -> Self {
        Self(value, LengthUnit::ex)
    }

    /// Create a length with `px` unit identifier.
    pub fn px(value: u32) -> Self {
        Self(value, LengthUnit::px)
    }

    /// Create a length with `inch` unit identifier.
    pub fn inch(value: u32) -> Self {
        Self(value, LengthUnit::inch)
    }
    /// Create a length with `cm` unit identifier.
    pub fn cm(value: u32) -> Self {
        Self(value, LengthUnit::cm)
    }
    /// Create a length with `mm` unit identifier.
    pub fn mm(value: u32) -> Self {
        Self(value, LengthUnit::mm)
    }
    /// Create a length with `pt` unit identifier.
    pub fn pt(value: u32) -> Self {
        Self(value, LengthUnit::pt)
    }
    /// Create a length with `pc` unit identifier.
    pub fn pc(value: u32) -> Self {
        Self(value, LengthUnit::pc)
    }

    /// Create a length with `px` unit identifier.
    pub fn percentage(value: u32) -> Self {
        Self(value, LengthUnit::percentage)
    }
}

#[cfg(test)]
mod tests {
    use crate::primitives::Length;

    #[test]
    fn display() {
        println!("{}", Length::cm(100));
    }
}
