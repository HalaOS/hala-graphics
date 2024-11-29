use super::{Animatable, FrameVariable, Measurement, Rect, Units};

/// The ‘overflow’ property has the same parameter values and has the same meaning [`as defined in CSS2`](https://www.w3.org/TR/2008/REC-CSS2-20080411/visufx.html#overflow)
///
/// See [`overflow`](https://www.w3.org/TR/SVG11/masking.html#OverflowAndClipProperties)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl FrameVariable for Overflow {}

/// The ‘clip’ property has the same parameter values as defined in CSS2 ([CSS2], section 11.1.2).
/// Unitless values, which indicate current user coordinates, are permitted on the coordinate values on the `shape`.
/// The value of auto defines a clipping path along the bounds of the viewport created by the given element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Clip {
    Shape(Rect),
    Auto,
}

impl Default for Clip {
    fn default() -> Self {
        Self::Auto
    }
}

impl FrameVariable for Clip {}

/// A clipping path is defined with a ‘clipPath’ element.
/// A clipping path is used/referenced using the ‘clip-path’ property.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClipPath(
    /// Defines the coordinate system for the contents of the ‘clipPath’.
    ///
    /// If clipPathUnits="userSpaceOnUse", the contents of the ‘clipPath’ represent values in the current user coordinate
    /// system in place at the time when the ‘clipPath’ element is referenced (i.e., the user coordinate system for the
    /// element referencing the ‘clipPath’ element via the ‘clip-path’ property).
    ///
    /// If clipPathUnits="objectBoundingBox", then the user coordinate system for the contents of the ‘clipPath’ element
    /// is established using the bounding box of the element to which the clipping path is applied (see Object bounding
    /// box units).
    ///
    /// If attribute ‘clipPathUnits’ is not specified, then the effect is as if a value of 'userSpaceOnUse' were specified.
    pub Animatable<Units>,
);

impl Default for ClipPath {
    fn default() -> Self {
        Self(Units::UserSpaceOnUse.into())
    }
}

/// The  property only applies to graphics elements that are contained within a [`ClipPath`] element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClipRule {
    Nonzero,
    EvenOdd,
}

/// used as an alpha mask for compositing the current object into the background.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mask {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If maskUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the current user coordinate system
    /// in place at the time when the ‘mask’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘mask’ element via the ‘mask’ property).
    ///
    /// If maskUnits="objectBoundingBox", ‘x’, ‘y’, ‘width’ and ‘height’ represent fractions or percentages of the bounding box
    /// of the element to which the mask is applied. (See Object bounding box units.)
    ///
    /// If attribute ‘maskUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
    pub units: Animatable<Units>,

    /// Defines the coordinate system for the contents of the ‘mask’.
    ///
    /// If maskContentUnits="userSpaceOnUse", the user coordinate system for the contents of the ‘mask’ element is the current user
    /// coordinate system in place at the time when the ‘mask’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘mask’ element via the ‘mask’ property).
    ///
    /// If maskContentUnits="objectBoundingBox", the user coordinate system for the contents of the ‘mask’ is established using the
    /// bounding box of the element to which the mask is applied. (See Object bounding box units.)
    ///
    /// If attribute ‘maskContentUnits’ is not specified, then the effect is as if a value of 'userSpaceOnUse' were specified.
    pub content_units: Animatable<Units>,

    /// The x-axis coordinate of one corner of the rectangle for the largest possible offscreen buffer. Note that the clipping
    /// path used to render any graphics within the mask will consist of the intersection of the current clipping path
    /// associated with the given object and the rectangle defined by ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If the attribute is not specified, the effect is as if a value of '-10%' were specified.
    pub x: Animatable<Measurement>,

    /// The y-axis coordinate of one corner of the rectangle for the largest possible offscreen buffer.
    ///
    /// If the attribute is not specified, the effect is as if a value of '-10%' were specified.
    pub y: Animatable<Measurement>,

    /// The width of the largest possible offscreen buffer. Note that the clipping path used to render any graphics within the
    /// mask will consist of the intersection of the current clipping path associated with the given object and the rectangle
    /// defined by ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// If the attribute is not specified, the effect is as if a value of '120%' were specified.
    pub width: Animatable<Measurement>,

    /// The height of the largest possible offscreen buffer.
    ///
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// If the attribute is not specified, the effect is as if a value of '120%' were specified.
    pub height: Animatable<Measurement>,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            units: Units::ObjectBoundingBox.into(),
            content_units: Units::UserSpaceOnUse.into(),
            x: Measurement::percentage(-10.0).into(),
            y: Measurement::percentage(-10.0).into(),
            width: Measurement::percentage(120f32).into(),
            height: Measurement::percentage(120f32).into(),
        }
    }
}

/// Except for object/group opacity (described just below), all other opacity properties are involved in intermediate
/// rendering operations. Object/group opacity can be thought of conceptually as a postprocessing operation. Conceptually,
/// after the object/group is rendered into an RGBA offscreen image, the object/group opacity setting specifies how
/// to blend the offscreen image into the current background.
///
/// See [`opacity`](https://www.w3.org/TR/SVG11/masking.html#OpacityProperty)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Opacity(f32);

impl Default for Opacity {
    fn default() -> Self {
        Self(1.0)
    }
}
