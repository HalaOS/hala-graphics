use super::{Fill, Layer, Marker, Stroke, Transform, ViewBox};

/// Effect scope of one instruction.
pub enum EffectScope {
    /// multiline instruction effect
    Multiline,
    /// singleline instruction effect
    Singleline,
}

/// A type that representation a cotai script instruction.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IR {
    /// Pop the top(n) instructions from the `multiline instruction stack`.
    Pop(usize),
    Transform(Box<Transform>),
    ViewBox(Box<ViewBox>),
    Fill(Box<Fill>),
    Stroke(Box<Stroke>),
    Maker(Box<Marker>),
    Layer(Box<Layer>),
    /// Defines a entity with id.
    Declare(Box<String>),
    /// A reference to entity by id.
    Ref(Box<String>),
    /// expression of if condition.
    ///
    /// The value of position `0` indicates the register num, the type of that must be bool.
    If(usize),
    /// Keyword `else`
    Else,
}

impl IR {
    /// Returns the effect scope of the instruction.
    pub fn scope(&self) -> EffectScope {
        match self {
            IR::Transform(_)
            | IR::ViewBox(_)
            | IR::Fill(_)
            | IR::Stroke(_)
            | IR::Maker(_)
            | IR::Declare(_)
            | IR::Layer(_)
            | IR::If(_)
            | IR::Else => EffectScope::Multiline,
            _ => EffectScope::Singleline,
        }
    }
}
