use super::{Angle, Animatable, Animation, Measurement, RecognizedColor, Rgba, ViewBox};

/// ‘fill’ and ‘stroke’ take on a value of type [`Paint`], which is specified as follows:
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Paint {
    /// the explicit color to be used to paint the current object
    Color(Rgba),
    /// A gradient entity reference.
    Gradient(String),
    /// A pattern entity reference.
    Pattern(String),
}

impl Animatable for Paint {}

impl From<RecognizedColor> for Paint {
    fn from(value: RecognizedColor) -> Self {
        Self::Color(value.into())
    }
}

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

impl Animatable for FillRule {}

/// The ‘fill’ instruction paints the interior of the given graphical element.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fill {
    /// paints color.
    ///
    /// `Inherited: yes`
    pub paint: Option<Animation<Paint>>,
    /// fill painting rule, see [`FillRule`] for more information.
    ///
    /// `Inherited: yes`
    pub rule: Option<Animation<FillRule>>,
}

/// Specifies the shape to be used at the end of open subpaths when they are stroked
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}

impl Animatable for StrokeLineCap {}

impl Default for StrokeLineCap {
    fn default() -> Self {
        Self::Butt
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrokeMiterlimit(Measurement);

impl Animatable for StrokeMiterlimit {}

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

impl Animatable for StrokeLineJoin {}

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
    pub paint: Option<Animation<Paint>>,
    /// This property specifies the width of the stroke on the current object
    ///
    /// `Inherited: yes`
    pub width: Option<Animation<Measurement>>,

    /// specifies the shape to be used at the end of open subpaths when they are stroked.
    ///
    /// `Inherited: yes`
    pub linecap: Option<Animation<StrokeLineCap>>,

    /// specifies the shape to be used at the corners of paths or basic shapes when they are stroked.
    ///
    /// `Inherited: yes`
    pub linejoin: Option<Animation<StrokeLineJoin>>,

    /// controls the pattern of dashes and gaps used to stroke paths. `<dasharray>` contains a list of comma and/or
    /// white space separated `<length>s` and `<percentage>s` that specify the lengths of alternating dashes and gaps.
    /// If an odd number of values is provided, then the list of values is repeated to yield an even number of values.
    /// Thus, stroke-dasharray: 5,3,2 is equivalent to stroke-dasharray: 5,3,2,5,3,2.
    ///
    /// `Inherited: yes`
    pub dasharray: Option<Animation<Vec<Animation<Measurement>>>>,
    /// specifies the distance into the dash pattern to start the dash
    ///
    /// `Inherited: yes`
    pub dashoffset: Option<Animation<Measurement>>,
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

impl Animatable for MarkerUnits {}

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
    pub unit: Animation<MarkerUnits>,
    /// The x-axis coordinate of the reference point which is to be aligned exactly at the marker position. The
    /// coordinate is defined in the coordinate system after application of the ‘viewBox’ and ‘preserveAspectRatio’
    /// attributes.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub refx: Animation<Measurement>,

    /// The y-axis coordinate of the reference point which is to be aligned exactly at the marker position. The
    /// coordinate is defined in the coordinate system after application of the ‘viewBox’ and ‘preserveAspectRatio’
    /// attributes.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    pub refy: Animation<Measurement>,

    /// Represents the width of the viewport into which the marker is to be fitted when it is rendered.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    /// If the attribute is not specified, the effect is as if a value of "3" were specified.
    pub width: Animation<Measurement>,

    /// Represents the height of the viewport into which the marker is to be fitted when it is rendered.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    /// If the attribute is not specified, the effect is as if a value of "3" were specified.
    pub height: Animation<Measurement>,

    /// Indicates how the marker is rotated. see [`svg`] document for more information.
    ///
    /// [`svg`]: https://www.w3.org/TR/SVG11/painting.html#MarkerElement
    pub orient: Option<Animation<Angle>>,

    /// stretch to fit a particular container element.
    pub viewbox: Option<Animation<ViewBox>>,
}

impl Default for Marker {
    fn default() -> Self {
        Self {
            unit: Animation::Constant(MarkerUnits::StrokeWidth),
            refx: Animation::Constant(0.0.into()),
            refy: Animation::Constant(0.0.into()),
            width: Animation::Constant(3.0.into()),
            height: Animation::Constant(3.0.into()),
            orient: None,
            viewbox: None,
        }
    }
}
