/// The ‘rect’ element defines a rectangle which is axis-aligned with the current user coordinate system.
/// Rounded rectangles can be achieved by setting appropriate values for attributes ‘rx’ and ‘ry’.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {}
