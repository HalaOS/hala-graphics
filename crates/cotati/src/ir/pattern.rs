/// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternUnits {
    /// If patternUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the coordinate system
    /// that results from taking the current user coordinate system in place at the time when the ‘pattern’ element
    /// is referenced (i.e., the user coordinate system for the element referencing the ‘pattern’ element via a ‘fill’
    /// or ‘stroke’ property) and then applying the transform specified by attribute ‘patternTransform’.
    UserSpaceOnUse,
    /// If patternUnits="objectBoundingBox", the user coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’
    /// is established using the bounding box of the element to which the pattern is applied (see Object bounding box units)
    /// and then applying the transform specified by attribute ‘patternTransform’.
    ObjectBoundingBox,
}

impl Default for PatternUnits {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}
