use super::{Measurement, PreserveAspectRatio, Transform, Variable, Variant, ViewBox};

/// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternUnits {
    /// see [`units`](Pattern::units) and [`content_units`](Pattern::content_units) for more informations.
    UserSpaceOnUse,
    /// see [`units`](Pattern::units) and [`content_units`](Pattern::content_units) for more informations.
    ObjectBoundingBox,
}

impl Default for PatternUnits {
    fn default() -> Self {
        Self::ObjectBoundingBox
    }
}

impl Variable for PatternUnits {}

/// A pattern is used to fill or stroke an object using a pre-defined graphic object which can be replicated ("tiled")
/// at fixed intervals in x and y to cover the areas to be painted. Patterns are defined using a ‘pattern’ element and
/// then referenced by properties ‘fill’ and ‘stroke’ on a given graphics element to indicate that the given element
/// shall be filled or stroked with the referenced pattern.
///
/// Attributes ‘x’, ‘y’, ‘width’, ‘height’ and ‘patternUnits’ define a reference rectangle somewhere on the infinite canvas.
/// The reference rectangle has its top/left at (x, y) and its bottom/right at (x + width, y + height). The tiling
/// theoretically extends a series of such rectangles to infinity in X and Y (positive and negative), with rectangles
/// starting at (x + m*width, y + n* height) for each possible integer value for m and n.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pattern {
    /// Defines the coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’.
    ///
    /// If patternUnits="objectBoundingBox", the user coordinate system for attributes ‘x’, ‘y’, ‘width’ and ‘height’
    /// is established using the bounding box of the element to which the pattern is applied (see Object bounding box units)
    /// and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If patternUnits="userSpaceOnUse", ‘x’, ‘y’, ‘width’ and ‘height’ represent values in the coordinate system
    /// that results from taking the current user coordinate system in place at the time when the ‘pattern’ element
    /// is referenced (i.e., the user coordinate system for the element referencing the ‘pattern’ element via a ‘fill’
    /// or ‘stroke’ property) and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If attribute `units` is not specified, then the effect is as if a value of 'objectBoundingBox' were specified.
    pub units: Variant<PatternUnits>,
    /// Defines the coordinate system for the contents of the ‘pattern’. Note that this attribute has no effect
    /// if attribute ‘viewBox’ is specified.
    ///
    /// If patternContentUnits="userSpaceOnUse", the user coordinate system for the contents of the ‘pattern’
    /// element is the coordinate system that results from taking the current user coordinate system in place
    /// at the time when the ‘pattern’ element is referenced (i.e., the user coordinate system for the element
    /// referencing the ‘pattern’ element via a ‘fill’ or ‘stroke’ property) and then applying the transform
    /// specified by attribute ‘patternTransform’.
    ///
    /// If patternContentUnits="objectBoundingBox", the user coordinate system for the contents of the ‘pattern’
    /// element is established using the bounding box of the element to which the pattern is applied (see Object
    /// bounding box units) and then applying the transform specified by attribute ‘patternTransform’.
    ///
    /// If attribute `content_units` is not specified, then the effect is as if a value of 'userSpaceOnUse'
    /// were specified.
    pub content_units: Variant<PatternUnits>,

    /// Contains the definition of an optional additional transformation from the pattern coordinate system onto the
    /// target coordinate system (i.e., 'userSpaceOnUse' or 'objectBoundingBox'). This allows for things such as
    /// skewing the pattern tiles. This additional transformation matrix is post-multiplied to (i.e., inserted to
    /// the right of) any previously defined transformations, including the implicit transformation necessary to convert
    /// from object bounding box units to user space.
    ///
    /// If attribute `transform` is not specified, then the effect is as if an identity transform were specified.
    pub transform: Variant<Transform>,

    /// ‘x’, ‘y’, ‘width’ and ‘height’ indicate how the pattern tiles are placed and spaced. These attributes represent
    /// coordinates and values in the coordinate space specified by the combination of attributes [`units`](Self::units) and
    /// [`content_units`](Self::content_units).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub x: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub y: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub width: Variant<Measurement>,

    /// See [`x`](Self::x).
    ///
    /// If the attribute is not specified, the effect is as if a value of zero were specified.
    ///
    /// Animatable: yes.
    pub height: Variant<Measurement>,

    /// See [`PreserveAspectRatio`].
    ///
    /// If the attribute is not specified, then the effect is as if a value of xMidYMid meet were specified.
    ///
    /// Animatable: yes.
    pub aspect: Variant<PreserveAspectRatio>,

    /// See [`ViewBox`].
    ///
    /// See [`content_units`](Self::content_units).
    pub viewbox: Option<Variant<ViewBox>>,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            units: Variant::Constant(PatternUnits::ObjectBoundingBox),
            content_units: Variant::Constant(PatternUnits::UserSpaceOnUse),
            transform: Variant::Constant(Transform::identity()),
            x: Default::default(),
            y: Default::default(),
            width: Default::default(),
            height: Default::default(),
            aspect: Default::default(),
            viewbox: None,
        }
    }
}

impl Pattern {
    /// Reset units property.
    pub fn units<V>(mut self, value: V) -> Self
    where
        PatternUnits: From<V>,
    {
        self.units = Variant::Constant(value.into());
        self
    }

    /// Reset units property to register variant.
    pub fn units_variable(mut self, id: usize) -> Self {
        self.units = Variant::Register(id);
        self
    }

    /// Reset content_units property.
    pub fn content_units<V>(mut self, value: V) -> Self
    where
        PatternUnits: From<V>,
    {
        self.content_units = Variant::Constant(value.into());
        self
    }

    /// Reset content_units property to register variant.
    pub fn content_units_variable(mut self, id: usize) -> Self {
        self.content_units = Variant::Register(id);
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
    /// Reset x-axis property.
    pub fn x<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.x = Variant::Constant(value.into());
        self
    }

    /// Reset x-axis property to register variant.
    pub fn x_variable(mut self, id: usize) -> Self {
        self.x = Variant::Register(id);
        self
    }

    /// Reset y-axis property.
    pub fn y<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.y = Variant::Constant(value.into());
        self
    }

    /// Reset y-axis property to register variant.
    pub fn y_variable(mut self, id: usize) -> Self {
        self.y = Variant::Register(id);
        self
    }

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

    /// Reset the aspect property of this viewbox.
    pub fn aspect<V>(mut self, value: V) -> Self
    where
        PreserveAspectRatio: From<V>,
    {
        self.aspect = Variant::Constant(value.into());
        self
    }

    /// Reset the aspect property of this viewbox to register variant.
    pub fn aspect_variable(mut self, id: usize) -> Self {
        self.aspect = Variant::Register(id);
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
