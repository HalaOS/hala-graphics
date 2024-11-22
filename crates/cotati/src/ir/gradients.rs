use super::{Measurement, RecognizedColor, Rgba, Transform, Variable, Variant};

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

impl Variable for GradientUnits {}

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

impl Variable for SpreadMethod {}

/// Linear gradients are defined by a ‘linearGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinearGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub unit: Variant<GradientUnits>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Animatable: yes.
    pub transform: Variant<Transform>,

    /// ‘x1’, ‘y1’, ‘x2’ and ‘y2’ define a gradient vector for the linear gradient.
    /// This gradient vector provides starting and ending points onto which the gradient stops are mapped. The values
    /// of ‘x1’, ‘y1’, ‘x2’ and ‘y2’ can be either numbers or percentages.
    ///
    /// If the attribute is not specified, the effect is as if a value of '0%' were specified.
    ///
    /// Animatable: yes.
    pub x1: Variant<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub y1: Variant<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub x2: Variant<Measurement>,

    /// See [`x1`](LinearGradient::x1)
    pub y2: Variant<Measurement>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Variant<SpreadMethod>,
}

impl Default for LinearGradient {
    fn default() -> Self {
        Self {
            unit: Variant::Constant(GradientUnits::default()),
            transform: Variant::Constant(Transform::identity()),
            x1: Variant::Constant(Measurement::percentage(0.0)),
            y1: Variant::Constant(Measurement::percentage(0.0)),
            x2: Variant::Constant(Measurement::percentage(100.0)),
            y2: Variant::Constant(Measurement::percentage(100.0)),
            spread: Variant::Constant(SpreadMethod::default()),
        }
    }
}

impl LinearGradient {
    /// Reset unit property.
    pub fn unit<V>(mut self, value: V) -> Self
    where
        GradientUnits: From<V>,
    {
        self.unit = Variant::Constant(value.into());
        self
    }

    /// Reset unit property to register variant.
    pub fn unit_variable(mut self, id: usize) -> Self {
        self.unit = Variant::Register(id);
        self
    }

    /// Reset transform property.
    pub fn transform<V>(mut self, value: V) -> Self
    where
        Transform: From<V>,
    {
        self.transform = Variant::Constant(value.into());
        self
    }

    /// Reset transform property to register variant.
    pub fn transform_variable(mut self, id: usize) -> Self {
        self.transform = Variant::Register(id);
        self
    }

    /// Reset x1 property.
    pub fn x1<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.x1 = Variant::Constant(value.into());
        self
    }

    /// Reset x1 property to register variant.
    pub fn x1_variable(mut self, id: usize) -> Self {
        self.x1 = Variant::Register(id);
        self
    }

    /// Reset y1 property.
    pub fn y1<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.y1 = Variant::Constant(value.into());
        self
    }

    /// Reset y1 property to register variant.
    pub fn y1_variable(mut self, id: usize) -> Self {
        self.y1 = Variant::Register(id);
        self
    }

    /// Reset x2 property.
    pub fn x2<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.x2 = Variant::Constant(value.into());
        self
    }

    /// Reset x2 property to register variant.
    pub fn x2_variable(mut self, id: usize) -> Self {
        self.x2 = Variant::Register(id);
        self
    }

    /// Reset y2 property.
    pub fn y2<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.y2 = Variant::Constant(value.into());
        self
    }

    /// Reset y2 property to register variant.
    pub fn y2_variable(mut self, id: usize) -> Self {
        self.y2 = Variant::Register(id);
        self
    }

    /// Reset spread property.
    pub fn spread<V>(mut self, value: V) -> Self
    where
        SpreadMethod: From<V>,
    {
        self.spread = Variant::Constant(value.into());
        self
    }

    /// Reset spread property to register variant.
    pub fn spread_variable(mut self, id: usize) -> Self {
        self.spread = Variant::Register(id);
        self
    }
}

/// Radial gradients are defined by a ‘radialGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadialGradient {
    /// Defines the coordinate system for attributes ‘x1’, ‘y1’, ‘x2’ and ‘y2’.
    pub unit: Variant<GradientUnits>,

    /// Contains the definition of an optional additional transformation from the gradient coordinate system onto the
    /// target coordinate system (i.e., userSpaceOnUse or objectBoundingBox). This allows for things such as skewing
    /// the gradient. This additional transformation matrix is post-multiplied to (i.e., inserted to the right of)
    /// any previously defined transformations, including the implicit transformation necessary to convert from object
    /// bounding box units to user space.
    ///
    /// If attribute ‘gradientTransform’ is not specified, then the effect is as if an identity transform were specified.
    ///
    /// Animatable: yes.
    pub transform: Option<Variant<Transform>>,

    /// ‘cx’, ‘cy’ and ‘r’ define the largest (i.e., outermost) circle for the radial gradient.
    /// The gradient will be drawn such that the 100% gradient stop is mapped to the perimeter
    /// of this largest (i.e., outermost) circle.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Animatable: yes.
    pub cx: Variant<Measurement>,

    /// See [`cx`](RadialGradient::cx)
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    pub cy: Variant<Measurement>,

    /// See [`cx`](RadialGradient::cx)
    ///
    /// A negative value is an error (see Error processing). A value of zero will cause the area to be painted as a single color
    /// using the color and opacity of the last gradient stop.
    ///
    /// If the attribute is not specified, the effect is as if a value of '50%' were specified.
    ///
    /// Animatable: yes.
    pub r: Variant<Measurement>,

    /// ‘fx’ and ‘fy’ define the focal point for the radial gradient. The gradient will be drawn such that the
    /// 0% gradient stop is mapped to (fx, fy).
    ///
    /// If attribute ‘fx’ is not specified, ‘fx’ will coincide with the presentational value of ‘cx’ for the element whether the value
    /// for 'cx' was inherited or not. If the element references an element that specifies a value for 'fx', then the value of 'fx'
    /// is inherited from the referenced element.
    ///
    /// Animatable: yes.
    pub fx: Variant<Measurement>,

    /// See [`fx`](RadialGradient::fx)
    ///
    /// If attribute ‘fy’ is not specified, ‘fy’ will coincide with the presentational value of ‘cy’ for the element whether the value
    /// for 'cy' was inherited or not. If the element references an element that specifies a value for 'fy', then the value of 'fy'
    /// is inherited from the referenced element.
    ///
    /// Animatable: yes.
    pub fy: Variant<Measurement>,

    /// Indicates what happens if the gradient starts or ends inside the bounds of the target rectangle.
    pub spread: Variant<SpreadMethod>,
}

impl Default for RadialGradient {
    fn default() -> Self {
        Self {
            unit: Variant::Constant(GradientUnits::default()),
            transform: None,
            cx: Variant::Constant(Measurement::percentage(50.0)),
            cy: Variant::Constant(Measurement::percentage(50.0)),
            r: Variant::Constant(Measurement::percentage(50.0)),
            fx: Variant::Constant(Measurement::percentage(50.0)),
            fy: Variant::Constant(Measurement::percentage(50.0)),
            spread: Variant::Constant(SpreadMethod::default()),
        }
    }
}

impl RadialGradient {
    /// Reset unit property.
    pub fn unit<V>(mut self, value: V) -> Self
    where
        GradientUnits: From<V>,
    {
        self.unit = Variant::Constant(value.into());
        self
    }

    /// Reset unit property to register variant.
    pub fn unit_variable(mut self, id: usize) -> Self {
        self.unit = Variant::Register(id);
        self
    }

    /// Reset transform property.
    pub fn transform<V>(mut self, value: V) -> Self
    where
        Transform: From<V>,
    {
        self.transform = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset transform property to register variant.
    pub fn transform_variable(mut self, id: usize) -> Self {
        self.transform = Some(Variant::Register(id));
        self
    }

    /// Reset cx property.
    pub fn cx<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.cx = Variant::Constant(value.into());
        self
    }

    /// Reset cx property to register variant.
    pub fn cx_variable(mut self, id: usize) -> Self {
        self.cx = Variant::Register(id);
        self
    }

    /// Reset y1 property.
    pub fn cy<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.cy = Variant::Constant(value.into());
        self
    }

    /// Reset y1 property to register variant.
    pub fn cy_variable(mut self, id: usize) -> Self {
        self.cy = Variant::Register(id);
        self
    }

    /// Reset r property.
    pub fn r<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.r = Variant::Constant(value.into());
        self
    }

    /// Reset r property to register variant.
    pub fn r_variable(mut self, id: usize) -> Self {
        self.r = Variant::Register(id);
        self
    }

    /// Reset fx property.
    pub fn fx<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.fx = Variant::Constant(value.into());
        self
    }

    /// Reset fx property to register variant.
    pub fn fx_variable(mut self, id: usize) -> Self {
        self.fx = Variant::Register(id);
        self
    }

    /// Reset fx property.
    pub fn fy<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.fy = Variant::Constant(value.into());
        self
    }

    /// Reset fx property to register variant.
    pub fn fy_variable(mut self, id: usize) -> Self {
        self.fy = Variant::Register(id);
        self
    }

    /// Reset spread property.
    pub fn spread<V>(mut self, value: V) -> Self
    where
        SpreadMethod: From<V>,
    {
        self.spread = Variant::Constant(value.into());
        self
    }

    /// Reset spread property to register variant.
    pub fn spread_variable(mut self, id: usize) -> Self {
        self.spread = Variant::Register(id);
        self
    }
}

/// The ramp of colors to use on a gradient is defined by the ‘stop’ elements that are child elements
/// to either the ‘linearGradient’ element or the ‘radialGradient’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GradientStop {
    /// The ‘offset’ attribute is either a `<number>` (usually ranging from 0 to 1) or a `<percentage>`
    /// (usually ranging from 0% to 100%) which indicates where the gradient stop is placed.
    /// For linear gradients, the ‘offset’ attribute represents a location along the gradient vector.
    /// For radial gradients, it represents a percentage distance from (fx,fy) to the edge of the
    /// outermost/largest circle.
    ///
    /// Animatable: yes.
    pub offset: Variant<Measurement>,

    /// indicates what color to use at that gradient stop
    pub color: Variant<Rgba>,
}

impl Default for GradientStop {
    fn default() -> Self {
        Self {
            offset: Variant::Constant(Default::default()),
            color: Variant::Constant(RecognizedColor::black.into()),
        }
    }
}

impl GradientStop {
    /// Reset offset property.
    pub fn offset<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.offset = Variant::Constant(value.into());
        self
    }

    /// Reset offset property to register variant.
    pub fn offset_variable(mut self, id: usize) -> Self {
        self.offset = Variant::Register(id);
        self
    }

    /// Reset color property.
    pub fn color<V>(mut self, value: V) -> Self
    where
        Rgba: From<V>,
    {
        self.color = Variant::Constant(value.into());
        self
    }

    /// Reset offset property to register variant.
    pub fn color_variable(mut self, id: usize) -> Self {
        self.color = Variant::Register(id);
        self
    }
}
