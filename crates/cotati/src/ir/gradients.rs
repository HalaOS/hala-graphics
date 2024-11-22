use super::{Animatable, Animation, Measurement, RecognizedColor, Rgba, Transform};

/// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
///
/// If attribute ‘gradientUnits’ is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
///
/// Animatable: yes.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GradientUnits {
    /// If gradientUnits="userSpaceOnUse", ‘x1’, ‘y1’, ‘x2’ and ‘y2’ represent values in the coordinate system
    /// that results from taking the current user coordinate system in place at the time when the gradient element
    /// is referenced (i.e., the user coordinate system for the element referencing the gradient element via a ‘fill’
    /// or ‘stroke’ property) and then applying the transform specified by attribute ‘gradientTransform’.
    UserSpaceOnUse,
    /// If gradientUnits="objectBoundingBox", the user coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’ is
    /// established using the bounding box of the element to which the gradient is applied (see Object bounding box units)
    /// and then applying the transform specified by attribute ‘gradientTransform’.
    ///
    /// When gradientUnits="objectBoundingBox" and ‘gradientTransform’ is the identity matrix, the normal of the linear
    /// gradient is perpendicular to the gradient vector in object bounding box space (i.e., the abstract coordinate
    /// system where (0,0) is at the top/left of the object bounding box and (1,1) is at the bottom/right of the object bounding box).
    /// When the object's bounding box is not square, the gradient normal which is initially perpendicular to the gradient vector
    /// within object bounding box space may render non-perpendicular relative to the gradient vector in user space. If the gradient
    /// vector is parallel to one of the axes of the bounding box, the gradient normal will remain perpendicular. This transformation
    /// is due to application of the non-uniform scaling transformation from bounding box space to user space.
    ObjectBoundingBox,
}

impl Default for GradientUnits {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}

impl Animatable for GradientUnits {}

/// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
/// Possible values are: 'pad', which says to use the terminal colors of the gradient to fill the remainder of the target region,
/// 'reflect', which says to reflect the gradient pattern start-to-end, end-to-start, start-to-end, etc. continuously until the
/// target rectangle is filled, and repeat, which says to repeat the gradient pattern start-to-end, start-to-end, start-to-end,
/// etc. continuously until the target region is filled.
/// If the attribute is not specified, the effect is as if a value of 'pad' were specified.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpreadMethod {
    Pad,
    Reflect,
    Repeat,
}

impl Default for SpreadMethod {
    fn default() -> Self {
        Self::Pad
    }
}

impl Animatable for SpreadMethod {}

/// Linear gradients are defined by a ‘linearGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub unit: Animation<GradientUnits>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Animatable: yes.
    pub transform: Animation<Transform>,

    /// ‘x1’, ‘y1’, ‘x2’ and ‘y2’ define a gradient vector for the linear gradient.
    /// This gradient vector provides starting and ending points onto which the gradient stops are mapped. The values
    /// of ‘x1’, ‘y1’, ‘x2’ and ‘y2’ can be either numbers or percentages.
    ///
    /// If the attribute is not specified, the effect is as if a value of '0%' were specified.
    ///
    /// Animatable: yes.
    pub x1: Animation<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub y1: Animation<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub x2: Animation<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub y2: Animation<Measurement>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Animation<SpreadMethod>,
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self {
            unit: Animation::Constant(GradientUnits::default()),
            transform: Animation::Constant(Transform::identity()),
            x1: Animation::Constant(Measurement::percentage(0.0)),
            y1: Animation::Constant(Measurement::percentage(0.0)),
            x2: Animation::Constant(Measurement::percentage(100.0)),
            y2: Animation::Constant(Measurement::percentage(100.0)),
            spread: Animation::Constant(SpreadMethod::default()),
        }
    }
}

/// Radial gradients are defined by a ‘radialGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub unit: Animation<GradientUnits>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Animatable: yes.
    pub transform: Animation<Transform>,

    /// ‘cx’, ‘cy’ and ‘r’ define the largest (i.e., outermost) circle for the radial gradient.
    /// The gradient will be drawn such that the 100% gradient stop is mapped to the perimeter
    /// of this largest (i.e., outermost) circle.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Animatable: yes.
    pub cx: Animation<Measurement>,

    /// See [`cx`](RadialGradient::cx)
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    pub cy: Animation<Measurement>,

    /// See [`cx`](RadialGradient::cx)
    ///
    /// A negative value is an error (see Error processing). A value of zero will cause the area to be painted as a single color
    /// using the color and opacity of the last gradient stop.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Animatable: yes.
    pub r: Animation<Measurement>,

    /// ‘fx’ and ‘fy’ define the focal point for the radial gradient. The gradient will be drawn such that the
    /// 0% gradient stop is mapped to (fx, fy).
    ///
    /// If attribute ‘fx’ is not specified, ‘fx’ will coincide with the presentational value of ‘cx’ for the element whether the value
    /// for 'cx' was inherited or not. If the element references an element that specifies a value for 'fx', then the value of 'fx'
    /// is inherited from the referenced element.
    ///
    /// Animatable: yes.
    pub fx: Animation<Measurement>,

    /// See [`fx`](RadialGradient::fx)
    ///
    /// If attribute ‘fy’ is not specified, ‘fy’ will coincide with the presentational value of ‘cy’ for the element whether the value
    /// for 'cy' was inherited or not. If the element references an element that specifies a value for 'fy', then the value of 'fy'
    /// is inherited from the referenced element.
    ///
    /// Animatable: yes.
    pub fy: Animation<Measurement>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Animation<SpreadMethod>,
}

impl Default for RadialGradient {
    fn default() -> Self {
        Self {
            unit: Animation::Constant(GradientUnits::default()),
            transform: Animation::Constant(Transform::identity()),
            cx: Animation::Constant(Measurement::percentage(50.0)),
            cy: Animation::Constant(Measurement::percentage(50.0)),
            r: Animation::Constant(Measurement::percentage(50.0)),
            fx: Animation::Constant(Measurement::percentage(50.0)),
            fy: Animation::Constant(Measurement::percentage(50.0)),
            spread: Animation::Constant(SpreadMethod::default()),
        }
    }
}

/// The ramp of colors to use on a gradient is defined by the ‘stop’ elements that are child elements
/// to either the ‘linearGradient’ element or the ‘radialGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    /// The ‘offset’ attribute is either a `<number>` (usually ranging from 0 to 1) or a `<percentage>`
    /// (usually ranging from 0% to 100%) which indicates where the gradient stop is placed.
    /// For linear gradients, the ‘offset’ attribute represents a location along the gradient vector.
    /// For radial gradients, it represents a percentage distance from (fx,fy) to the edge of the
    /// outermost/largest circle.
    ///
    /// Animatable: yes.
    pub offset: Animation<Measurement>,

    /// indicates what color to use at that gradient stop
    pub color: Animation<Rgba>,
}

impl Default for GradientStop {
    fn default() -> Self {
        Self {
            offset: Animation::Constant(Default::default()),
            color: Animation::Constant(RecognizedColor::black.into()),
        }
    }
}
