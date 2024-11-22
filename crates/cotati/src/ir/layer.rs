use super::{Measurement, Animation, ViewBox};

/// Create a new layer into which the backend render child elements.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Layer {
    /// a number (usually an integer) that represents the width of the rendering layer.
    pub width: Animation<Measurement>,
    /// a number (usually an integer) that represents the height of the rendering layer.
    pub height: Animation<Measurement>,
    /// stretch to fit a particular container element.
    pub viewbox: Option<Animation<ViewBox>>,
}
