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

/// Defines the coordinate system for the contents
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Units {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FrameVariable for Units {}

/// A sequence of Unicode characters
///
/// See [`description of ranges of Unicode characters in CSS2`](https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#dataqual)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnicodeRange {
    Char(char),

    Range { from: char, to: char },
}

impl FrameVariable for UnicodeRange {}

impl Default for UnicodeRange {
    fn default() -> Self {
        Self::Range {
            from: '\u{0}',
            to: '\u{10ffff}',
        }
    }
}
