use std::fmt::Display;

use super::Unit;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub unit: Unit,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "point({}{},{}{})", self.x, self.unit, self.y, self.unit)
    }
}

impl Point {
    /// Create a point with `em` unit identifier.
    pub fn em(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Em,
        }
    }

    /// Create a point with `em` unit identifier.
    pub fn ex(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Ex,
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn px(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Px,
        }
    }

    /// Create a point with `inch` unit identifier.
    pub fn inch(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::In,
        }
    }
    /// Create a point with `cm` unit identifier.
    pub fn cm(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Cm,
        }
    }
    /// Create a point with `mm` unit identifier.
    pub fn mm(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Mm,
        }
    }
    /// Create a point with `pt` unit identifier.
    pub fn pt(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Pt,
        }
    }
    /// Create a point with `pc` unit identifier.
    pub fn pc(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Pc,
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn percentage(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Unit::Percentages,
        }
    }
}
