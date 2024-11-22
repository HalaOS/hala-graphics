use crate::errors::{Error, Result};

/// This is a marker trait that a type with this trait can be used as frame register variable.
pub trait FrameVariable {}

impl<T> FrameVariable for Vec<T> where T: FrameVariable {}

/// An variable container, indicates that this variable can be used as animation frame variable.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Animatable<T>
where
    T: FrameVariable,
{
    /// a reference variable to `frame register`.
    Frame(String),
    /// const value
    Constant(T),
}

impl<T> FrameVariable for Animatable<T> where T: FrameVariable {}

impl<T> Animatable<T>
where
    T: FrameVariable,
{
    /// Convert self into [`Result<T>`].
    ///
    /// * returns [`Error::UnsatisfiedVariant`] if this variant is a [`register`](Variant::Register) value.
    /// * returns [`Ok(T)`](Ok) if this variant is a [`constant`](Variant::Constant) value
    pub fn ok(self) -> Result<T> {
        match self {
            Animatable::Frame(n) => Err(Error::UnsatisfiedVariant(n)),
            Animatable::Constant(v) => Ok(v),
        }
    }
}

impl<T> From<T> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T> From<&str> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: &str) -> Self {
        Self::Frame(value.to_string())
    }
}

impl<T> From<String> for Animatable<T>
where
    T: FrameVariable,
{
    fn from(value: String) -> Self {
        Self::Frame(value)
    }
}

impl<T> Default for Animatable<T>
where
    T: Default + FrameVariable,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}
