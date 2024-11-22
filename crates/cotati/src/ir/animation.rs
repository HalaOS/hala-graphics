use crate::errors::{Error, Result};

/// This is a marker trait that a type with this trait can be used as `context variant type`.
pub trait Animatable {}

impl<T> Animatable for Vec<T> where T: Animatable {}

/// An [`Animatable`] value container that indicate this variable is animatable.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Animation<T>
where
    T: Animatable,
{
    /// `Animation Frame Register` variable.
    Animatable(String),
    /// Variant is a constant value.
    Constant(T),
}

impl<T> Animatable for Animation<T> where T: Animatable {}

impl<T> Animation<T>
where
    T: Animatable,
{
    /// Convert self into [`Result<T>`].
    ///
    /// * returns [`Error::UnsatisfiedVariant`] if this variant is a [`register`](Variant::Register) value.
    /// * returns [`Ok(T)`](Ok) if this variant is a [`constant`](Variant::Constant) value
    pub fn ok(self) -> Result<T> {
        match self {
            Animation::Animatable(n) => Err(Error::UnsatisfiedVariant(n)),
            Animation::Constant(v) => Ok(v),
        }
    }
}

impl<T> From<T> for Animation<T>
where
    T: Animatable,
{
    fn from(value: T) -> Self {
        Self::Constant(value)
    }
}

impl<T> From<&str> for Animation<T>
where
    T: Animatable,
{
    fn from(value: &str) -> Self {
        Self::Animatable(value.to_string())
    }
}

impl<T> From<String> for Animation<T>
where
    T: Animatable,
{
    fn from(value: String) -> Self {
        Self::Animatable(value)
    }
}

impl<T> Default for Animation<T>
where
    T: Default + Animatable,
{
    fn default() -> Self {
        Self::Constant(T::default())
    }
}
