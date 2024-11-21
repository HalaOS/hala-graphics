use crate::{Error, Result};

use super::{
    Angle, FillRule, MarkerUnits, Measurement, Point, PreserveAspectRatio, Rgba, StrokeLineCap,
    StrokeLineJoin, StrokeMiterlimit,
};

/// This is a marker trait that a type with this trait can be used as `context variant type`.
pub trait Variable {}

impl<T> Variable for Vec<T> where T: Variable {}

/// A instruction that define a context variant.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Variant<T>
where
    T: Variable,
{
    /// Indicate the variant is store in `register(n)`
    Register(usize),
    /// Variant is a constant value.
    Constant(T),
}

impl<T> Variable for Variant<T> where T: Variable {}

impl<T> Variant<T>
where
    T: Variable,
{
    /// Convert self into [`Result<T>`].
    ///
    /// * returns [`Error::UnsatisfiedVariant`] if this variant is a [`register`](Variant::Register) value.
    /// * returns [`Ok(T)`](Ok) if this variant is a [`constant`](Variant::Constant) value
    pub fn ok(self) -> Result<T> {
        match self {
            Variant::Register(n) => Err(Error::UnsatisfiedVariant(n)),
            Variant::Constant(v) => Ok(v),
        }
    }
}

impl<T> From<T> for Variant<T>
where
    T: Variable,
{
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T> Default for Variant<T>
where
    T: Default + Variable,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}

/// A register value.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Color(Box<Rgba>),
    Measurement(Measurement),
    Aspect(PreserveAspectRatio),
    Angle(Angle),
    Point(Point),
    Points(Box<Vec<Point>>),
    FillRule(FillRule),
    StrokeLineCap(StrokeLineCap),
    StrokeLineJoin(StrokeLineJoin),
    StrokeMiterlimit(StrokeMiterlimit),
    MarkerUnits(MarkerUnits),
    DashArray(Box<Vec<Measurement>>),
    Bool(bool),
}
