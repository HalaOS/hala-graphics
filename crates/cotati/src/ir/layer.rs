use super::{Measurement, Variant, ViewBox};

/// Create a new layer into which the backend render child elements.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    /// a number (usually an integer) that represents the width of the rendering layer.
    pub width: Variant<Measurement>,
    /// a number (usually an integer) that represents the height of the rendering layer.
    pub height: Variant<Measurement>,
    /// stretch to fit a particular container element.
    pub viewbox: Option<Variant<ViewBox>>,
}

impl Layer {
    /// Reset width property.
    pub fn width<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.width = Variant::Constant(value.into());
        self
    }

    /// Reset width property to register variant.
    pub fn width_variable(mut self, id: usize) -> Self {
        self.width = Variant::Register(id);
        self
    }

    /// Reset height property.
    pub fn height<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.height = Variant::Constant(value.into());
        self
    }

    /// Reset height property to register variant.
    pub fn height_variable(mut self, id: usize) -> Self {
        self.height = Variant::Register(id);
        self
    }

    /// Reset viewbox property.
    pub fn viewbox<V>(mut self, value: V) -> Self
    where
        ViewBox: From<V>,
    {
        self.viewbox = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset viewbox property to register variant.
    pub fn viewbox_variable(mut self, id: usize) -> Self {
        self.viewbox = Some(Variant::Register(id));
        self
    }
}
