use super::FrameVariable;

/// A pair of `number`s, where the second `number` is optional.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber {
    pub dx: f32,
    pub dy: Option<f32>,
}

impl FrameVariable for NumberOptNumber {}
