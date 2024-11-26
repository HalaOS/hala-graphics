use super::FrameVariable;

/// A pair of `number`s, where the second `number` is optional.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberOptNumber {
    pub dx: f32,
    pub dy: Option<f32>,
}

impl FrameVariable for NumberOptNumber {}

/// The rgba components selector.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChannelSelector {
    R,
    G,
    B,
    A,
}

impl Default for ChannelSelector {
    fn default() -> Self {
        Self::A
    }
}

impl FrameVariable for ChannelSelector {}

/// An IRI reference
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Href(pub String);

impl FrameVariable for Href {}
