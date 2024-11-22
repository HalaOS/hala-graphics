use super::{Animatable, Measurement, Point};

/// The ‘rect’ element defines a rectangle which is axis-aligned with the current user coordinate system.
/// Rounded rectangles can be achieved by setting appropriate values for attributes ‘rx’ and ‘ry’.
///
///
/// The values used for the x- and y-axis rounded corner radii are determined implicitly if the ‘rx’ or ‘ry’ attributes (or both) are not specified, or are specified but with invalid values. The values are also subject to clamping so that the lengths of the straight segments of the rectangle are never negative. The effective values for ‘rx’ and ‘ry’ are determined by following these steps in order:
///
/// 1. Let rx and ry be length values.
/// 1. If neither ‘rx’ nor ‘ry’ are properly specified, then set both rx and ry to 0. (This will result in square corners.)
/// 1. Otherwise, if a properly specified value is provided for ‘rx’, but not for ‘ry’, then set both rx and ry to the value of ‘rx’.
/// 1. Otherwise, if a properly specified value is provided for ‘ry’, but not for ‘rx’, then set both rx and ry to the value of ‘ry’.
/// 1. Otherwise, both ‘rx’ and ‘ry’ were specified properly. Set rx to the value of ‘rx’ and ry to the value of ‘ry’.
/// 1. If rx is greater than half of ‘width’, then set rx to half of ‘width’.
/// 1. If ry is greater than half of ‘height’, then set ry to half of ‘height’.
/// 1. The effective values of ‘rx’ and ‘ry’ are rx and ry, respectively.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x-axis coordinate of the side of the rectangle which has the smaller x-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x: Animatable<Measurement>,

    /// The y-axis coordinate of the side of the rectangle which has the smaller y-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y: Animatable<Measurement>,

    /// The width of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub width: Animatable<Measurement>,

    /// The height of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub height: Animatable<Measurement>,

    /// For rounded rectangles, the x-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub rx: Animatable<Measurement>,

    /// For rounded rectangles, the y-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub ry: Animatable<Measurement>,
}

/// The ‘circle’ element defines a circle based on a center point and a radius.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    /// The x-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Animatable<Measurement>,

    /// The y-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Animatable<Measurement>,

    /// The radius of the circle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub r: Animatable<Measurement>,
}

/// The ‘ellipse’ element defines an ellipse which is axis-aligned with the current user coordinate
/// system based on a center point and two radii.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    /// The x-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Animatable<Measurement>,

    /// The y-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Animatable<Measurement>,

    /// The x-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub rx: Animatable<Measurement>,

    /// The y-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub ry: Animatable<Measurement>,
}

/// The ‘line’ element defines a line segment that starts at one point and ends at another.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    /// The x-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x1: Animatable<Measurement>,

    /// The y-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y1: Animatable<Measurement>,

    /// The x-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x2: Animatable<Measurement>,

    /// The y-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y2: Animatable<Measurement>,
}

/// The ‘polyline’ element defines a set of connected straight line segments. Typically, ‘polyline’ elements define open shapes.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(
    /// The points that make up the polyline. All coordinate values are in the user coordinate system.
    ///
    /// Animatable: yes.
    Animatable<Vec<Animatable<Point>>>,
);

impl Polyline {
    /// Create a new Line instance with constant value.
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator,
        Animatable<Point>: From<I::Item>,
    {
        Self(Animatable::Constant(
            points.into_iter().map(|v| v.into()).collect(),
        ))
    }
}

/// The ‘polygon’ element defines a closed shape consisting of a set of connected straight line segments.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon(
    /// The points that make up the polygon. All coordinate values are in the user coordinate system.
    ///
    /// Animatable: yes.
    Animatable<Vec<Animatable<Point>>>,
);

impl Polygon {
    /// Create a new polygon instance with constant value.
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator,
        Animatable<Point>: From<I::Item>,
    {
        Self(Animatable::Constant(
            points.into_iter().map(|v| v.into()).collect(),
        ))
    }
}
