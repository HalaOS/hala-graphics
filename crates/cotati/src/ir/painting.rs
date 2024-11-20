use super::{Angle, Measurement, RecognizedColor, Rgba, Variable, Variant};

//// The ‘fill-rule’ property indicates the algorithm which is to be used to determine what parts of the canvas are
//// included inside the shape. For a simple, non-intersecting path, it is intuitively clear what region lies "inside";
//// however, for a more complex path, such as a path that intersects itself or where one subpath encloses another,
//// the interpretation of "inside" is not so obvious.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FillRule {
    /// This rule determines the "insideness" of a point on the canvas by drawing a ray from that point to infinity in
    /// any direction and then examining the places where a segment of the shape crosses the ray. Starting with a count of zero,
    /// add one each time a path segment crosses the ray from left to right and subtract one each time a path segment crosses
    /// the ray from right to left. After counting the crossings, if the result is zero then the point is outside the path.
    /// Otherwise, it is inside.
    Nonzero,
    /// This rule determines the "insideness" of a point on the canvas by drawing a ray from that point to infinity in any direction
    /// and counting the number of path segments from the given shape that the ray crosses. If this number is odd, the point is inside;
    /// if even, the point is outside.
    EvenOdd,
}

impl Default for FillRule {
    fn default() -> Self {
        Self::Nonzero
    }
}

impl Variable for FillRule {}

/// The ‘fill’ instruction paints the interior of the given graphical element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    /// paints color.
    ///
    /// `Inherited: yes`
    pub color: Option<Variant<Rgba>>,
    /// fill painting rule, see [`FillRule`] for more information.
    ///
    /// `Inherited: yes`
    pub rule: Option<Variant<FillRule>>,
}

impl Fill {
    /// Initialize a new `Fill` painting instruction.
    ///
    /// * set color = [`black`](RecognizedColor::black)
    /// * set rule = [`Nonzero`](FillRule::Nonzero)
    pub fn new() -> Self {
        Self {
            color: Some(Variant::Constant(RecognizedColor::black.into())),
            rule: Some(Variant::Constant(FillRule::Nonzero)),
        }
    }
    /// Reset the color property.
    pub fn color<V>(mut self, value: V) -> Self
    where
        Rgba: From<V>,
    {
        self.color = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset the color property to register variant.
    pub fn color_register(mut self, id: usize) -> Self {
        self.color = Some(Variant::Register(id));
        self
    }

    /// Reset the color property.
    pub fn rule<V>(mut self, value: V) -> Self
    where
        FillRule: From<V>,
    {
        self.rule = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset the rule property to register variant.
    pub fn rule_register(mut self, id: usize) -> Self {
        self.color = Some(Variant::Register(id));
        self
    }
}

/// Specifies the shape to be used at the end of open subpaths when they are stroked
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}

impl Variable for StrokeLineCap {}

impl Default for StrokeLineCap {
    fn default() -> Self {
        Self::Butt
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrokeMiterlimit(Measurement);

impl Variable for StrokeMiterlimit {}

impl Default for StrokeMiterlimit {
    fn default() -> Self {
        Self(4.0.into())
    }
}

/// Specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineJoin {
    Miter(StrokeMiterlimit),
    Round,
    Bevel,
}

impl Variable for StrokeLineJoin {}

impl Default for StrokeLineJoin {
    fn default() -> Self {
        Self::Miter(Default::default())
    }
}

/// This property affect how an element is stroked.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stroke {
    /// paints color paints along the outline of the given graphical element.
    ///
    /// `Inherited: yes`
    pub color: Option<Variant<Rgba>>,
    /// This property specifies the width of the stroke on the current object
    ///
    /// `Inherited: yes`
    pub width: Option<Variant<Measurement>>,

    /// specifies the shape to be used at the end of open subpaths when they are stroked.
    ///
    /// `Inherited: yes`
    pub linecap: Option<Variant<StrokeLineCap>>,

    /// specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
    ///
    /// `Inherited: yes`
    pub linejoin: Option<Variant<StrokeLineJoin>>,

    /// controls the pattern of dashes and gaps used to stroke paths. `<dasharray>` contains a list of comma and/or
    /// white space separated `<length>s` and `<percentage>s` that specify the lengths of alternating dashes and gaps.
    /// If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
    /// Thus, stroke-dasharray: 5,3,2 is equivalent to stroke-dasharray: 5,3,2,5,3,2.
    ///
    /// `Inherited: yes`
    pub dasharray: Option<Variant<Vec<Measurement>>>,
    /// specifies the distance into the dash pattern to start the dash
    ///
    /// `Inherited: yes`
    pub dashoffset: Option<Variant<Measurement>>,
}

impl Stroke {
    /// Reset color property.
    pub fn color<V>(mut self, value: V) -> Self
    where
        Rgba: From<V>,
    {
        self.color = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset color property to register variant.
    pub fn color_register(mut self, id: usize) -> Self {
        self.color = Some(Variant::Register(id));
        self
    }

    /// Reset stroke width property.
    pub fn width<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.width = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset stroke width property to register variant.
    pub fn width_register(mut self, id: usize) -> Self {
        self.width = Some(Variant::Register(id));
        self
    }

    /// Reset linecap property.
    pub fn linecap<V>(mut self, value: V) -> Self
    where
        StrokeLineCap: From<V>,
    {
        self.linecap = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset linecap property to register variant.
    pub fn linecap_register(mut self, id: usize) -> Self {
        self.linecap = Some(Variant::Register(id));
        self
    }

    /// Reset linejoin property.
    pub fn linejoin<V>(mut self, value: V) -> Self
    where
        StrokeLineJoin: From<V>,
    {
        self.linejoin = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset linejoin property to register variant.
    pub fn linejoin_register(mut self, id: usize) -> Self {
        self.linejoin = Some(Variant::Register(id));
        self
    }

    /// Reset dasharray property.
    pub fn dasharray<I, V>(mut self, value: I) -> Self
    where
        I: IntoIterator<Item = V>,
        Measurement: From<V>,
    {
        self.dasharray = Some(Variant::Constant(
            value.into_iter().map(|v| v.into()).collect(),
        ));
        self
    }

    /// Reset dasharray property to register variant.
    pub fn dasharray_register(mut self, id: usize) -> Self {
        self.dasharray = Some(Variant::Register(id));
        self
    }

    /// Reset dashoffset property.
    pub fn dashoffset<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.dashoffset = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset dashoffset property to register variant.
    pub fn dashoffset_register(mut self, id: usize) -> Self {
        self.dashoffset = Some(Variant::Register(id));
        self
    }
}

/// Defines the coordinate system for attributes ‘markerWidth’, ‘markerHeight’ and the contents of the ‘marker’.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MarkerUnits {
    /// If markerUnits="strokeWidth", ‘markerWidth’, ‘markerHeight’ and the contents of the ‘marker’ represent values
    /// in a coordinate system which has a single unit equal the size in user units of the current stroke width (see
    /// the ‘stroke-width’ property) in place for the graphic object referencing the marker.
    StrokeWidth,
    /// If markerUnits="userSpaceOnUse", ‘markerWidth’, ‘markerHeight’ and the contents of the ‘marker’ represent values
    /// in the current user coordinate system in place for the graphic object referencing the marker (i.e., the user
    /// coordinate system for the element referencing the ‘marker’ element via a ‘marker’, ‘marker-start’, ‘marker-mid’
    /// or ‘marker-end’ property).
    UserSpaceOnUse,
}

impl Variable for MarkerUnits {}

impl Default for MarkerUnits {
    fn default() -> Self {
        Self::StrokeWidth
    }
}

/// A marker is a symbol which is attached to one or more vertices of ‘path’, ‘line’, ‘polyline’ and ‘polygon’ elements.
/// Typically, markers are used to make arrowheads or polymarkers. Arrowheads can be defined by attaching a marker to the
/// start or end vertices of ‘path’, ‘line’ or ‘polyline’ elements. Polymarkers can be defined by attaching a marker to all
/// vertices of a ‘path’, ‘line’, ‘polyline’ or ‘polygon’ element.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Marker {
    /// Defines the coordinate system for attributes ‘markerWidth’, ‘markerHeight’ and the contents of the ‘marker’.
    ///
    /// If attribute ‘markerUnits’ is not specified, then the effect is as if a value of 'strokeWidth' were specified.
    pub unit: Variant<MarkerUnits>,
    /// The x-axis coordinate of the reference point which is to be aligned exactly at the marker position. The
    /// coordinate is defined in the coordinate system after application of the ‘viewBox’ and ‘preserveAspectRatio’
    /// attributes.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub refx: Variant<Measurement>,

    /// The y-axis coordinate of the reference point which is to be aligned exactly at the marker position. The
    /// coordinate is defined in the coordinate system after application of the ‘viewBox’ and ‘preserveAspectRatio’
    /// attributes.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub refy: Variant<Measurement>,

    /// Represents the width of the viewport into which the marker is to be fitted when it is rendered.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    /// If the attribute is not specified, the effect is as if a value of "3" were specified.
    pub width: Variant<Measurement>,

    /// Represents the height of the viewport into which the marker is to be fitted when it is rendered.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    /// If the attribute is not specified, the effect is as if a value of "3" were specified.
    pub height: Variant<Measurement>,

    /// Indicates how the marker is rotated. see [`svg`] document for more information.
    ///
    /// [`svg`]: https://www.w3.org/TR/SVG11/painting.html#MarkerElement
    pub orient: Option<Variant<Angle>>,
}

impl Default for Marker {
    fn default() -> Self {
        Self {
            unit: Variant::Constant(MarkerUnits::StrokeWidth),
            refx: Variant::Constant(0.0.into()),
            refy: Variant::Constant(0.0.into()),
            width: Variant::Constant(3.0.into()),
            height: Variant::Constant(3.0.into()),
            orient: None,
        }
    }
}

impl Marker {
    /// Reset unit property.
    pub fn unit<V>(mut self, value: V) -> Self
    where
        MarkerUnits: From<V>,
    {
        self.unit = Variant::Constant(value.into());
        self
    }

    /// Reset unit property to register variant.
    pub fn unit_register(mut self, id: usize) -> Self {
        self.unit = Variant::Register(id);
        self
    }

    /// Reset refx property.
    pub fn refx<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.refx = Variant::Constant(value.into());
        self
    }

    /// Reset refx property to register variant.
    pub fn refx_register(mut self, id: usize) -> Self {
        self.refx = Variant::Register(id);
        self
    }

    /// Reset refy property.
    pub fn refy<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.refy = Variant::Constant(value.into());
        self
    }

    /// Reset refy property to register variant.
    pub fn refy_register(mut self, id: usize) -> Self {
        self.refy = Variant::Register(id);
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
    pub fn width_register(mut self, id: usize) -> Self {
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

    /// Reset refy property to register variant.
    pub fn height_register(mut self, id: usize) -> Self {
        self.height = Variant::Register(id);
        self
    }

    /// Reset orient property.
    pub fn orient<V>(mut self, value: V) -> Self
    where
        Angle: From<V>,
    {
        self.orient = Some(Variant::Constant(value.into()));
        self
    }

    /// Reset orient property to register variant.
    pub fn orient_register(mut self, id: usize) -> Self {
        self.orient = Some(Variant::Register(id));
        self
    }

    /// Reset orient property to `auto`.
    pub fn orient_auto(mut self) -> Self {
        self.orient = None;
        self
    }
}
