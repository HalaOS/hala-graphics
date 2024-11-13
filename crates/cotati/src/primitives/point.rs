use std::fmt::Display;

use super::Unit;

/// A 2d coordinate point.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub unit: Option<Unit>,
}

/// Create a point from (f32,f32) with default unit `px`.
impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            unit: None,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unit) = self.unit {
            write!(f, "({}{},{}{})", self.x, unit, self.y, unit)
        } else {
            write!(f, "({},{})", self.x, self.y)
        }
    }
}

impl Point {
    /// Create a point with `em` unit identifier.
    pub fn em(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Em),
        }
    }

    /// Create a point with `em` unit identifier.
    pub fn ex(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Ex),
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn px(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Px),
        }
    }

    /// Create a point with `inch` unit identifier.
    pub fn inch(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::In),
        }
    }
    /// Create a point with `cm` unit identifier.
    pub fn cm(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Cm),
        }
    }
    /// Create a point with `mm` unit identifier.
    pub fn mm(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Mm),
        }
    }
    /// Create a point with `pt` unit identifier.
    pub fn pt(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Pt),
        }
    }
    /// Create a point with `pc` unit identifier.
    pub fn pc(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Pc),
        }
    }

    /// Create a point with `px` unit identifier.
    pub fn percentage(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            unit: Some(Unit::Percentages),
        }
    }
}
