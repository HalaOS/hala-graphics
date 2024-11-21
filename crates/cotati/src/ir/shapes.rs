use super::{Measurement, Point, Variant};

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
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect {
    /// The x-axis coordinate of the side of the rectangle which has the smaller x-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x: Variant<Measurement>,

    /// The y-axis coordinate of the side of the rectangle which has the smaller y-axis coordinate value in the current user coordinate system.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y: Variant<Measurement>,

    /// The width of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub width: Variant<Measurement>,

    /// The height of the rectangle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub height: Variant<Measurement>,

    /// For rounded rectangles, the x-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub rx: Variant<Measurement>,

    /// For rounded rectangles, the y-axis radius of the ellipse used to round off the corners of the rectangle.
    /// A negative value is an error (see Error processing).
    ///
    /// Animatable: yes.
    pub ry: Variant<Measurement>,
}

impl Rect {
    /// Create a new Rect with constant properties.
    pub fn new<X, Y, W, H, RX, RY>(x: X, y: Y, w: W, h: H, rx: RX, ry: RY) -> Self
    where
        Measurement: From<X> + From<Y> + From<W> + From<H> + From<RX> + From<RY>,
    {
        Self {
            x: Variant::Constant(x.into()),
            y: Variant::Constant(y.into()),
            width: Variant::Constant(w.into()),
            height: Variant::Constant(h.into()),
            rx: Variant::Constant(rx.into()),
            ry: Variant::Constant(ry.into()),
        }
    }

    /// Create a new Rect with variable properties.
    pub fn variable(x: usize, y: usize, w: usize, h: usize, rx: usize, ry: usize) -> Self {
        Self {
            x: Variant::Register(x),
            y: Variant::Register(y),
            width: Variant::Register(w),
            height: Variant::Register(h),
            rx: Variant::Register(rx),
            ry: Variant::Register(ry),
        }
    }

    /// Create a square with constant properties.
    pub fn square<X, Y, W, RX, RY>(x: X, y: Y, w: W, rx: RX, ry: RY) -> Self
    where
        Measurement: From<X> + From<Y> + From<W> + From<RX> + From<RY>,
    {
        let width: Measurement = w.into();

        Self {
            x: Variant::Constant(x.into()),
            y: Variant::Constant(y.into()),
            width: Variant::Constant(width.clone()),
            height: Variant::Constant(width),
            rx: Variant::Constant(rx.into()),
            ry: Variant::Constant(ry.into()),
        }
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

    /// Reset rx property.
    pub fn rx<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.rx = Variant::Constant(value.into());
        self
    }

    /// Reset rx property to register variant.
    pub fn rx_variable(mut self, id: usize) -> Self {
        self.rx = Variant::Register(id);
        self
    }

    /// Reset ry property.
    pub fn ry<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.ry = Variant::Constant(value.into());
        self
    }

    /// Reset ry property to register variant.
    pub fn ry_variable(mut self, id: usize) -> Self {
        self.ry = Variant::Register(id);
        self
    }
}

/// The ‘circle’ element defines a circle based on a center point and a radius.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle {
    /// The x-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Variant<Measurement>,

    /// The y-axis coordinate of the center of the circle.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Variant<Measurement>,

    /// The radius of the circle.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub r: Variant<Measurement>,
}

impl Circle {
    /// Create new Circle instance with constant properties.
    pub fn new<CX, CY, R>(cx: CX, cy: CY, r: R) -> Self
    where
        Measurement: From<CX> + From<CY> + From<R>,
    {
        Self {
            cx: Variant::Constant(cx.into()),
            cy: Variant::Constant(cy.into()),
            r: Variant::Constant(r.into()),
        }
    }

    /// Create new Circle instance with constant properties.
    pub fn variable(cx: usize, cy: usize, r: usize) -> Self {
        Self {
            cx: Variant::Register(cx),
            cy: Variant::Register(cy),
            r: Variant::Register(r),
        }
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

    /// Reset cy property.
    pub fn cy<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.cy = Variant::Constant(value.into());
        self
    }

    /// Reset cy property to register variant.
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
}

/// The ‘ellipse’ element defines an ellipse which is axis-aligned with the current user coordinate
/// system based on a center point and two radii.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ellipse {
    /// The x-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cx: Variant<Measurement>,

    /// The y-axis coordinate of the center of the ellipse.
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub cy: Variant<Measurement>,

    /// The x-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub rx: Variant<Measurement>,

    /// The y-axis radius of the ellipse.
    /// A negative value is an error (see Error processing). A value of zero disables rendering of the element.
    ///
    /// Animatable: yes.
    pub ry: Variant<Measurement>,
}

impl Ellipse {
    /// Create new Circle instance with constant properties.
    pub fn new<CX, CY, RX, RY>(cx: CX, cy: CY, rx: RX, ry: RY) -> Self
    where
        Measurement: From<CX> + From<CY> + From<RX> + From<RY>,
    {
        Self {
            cx: Variant::Constant(cx.into()),
            cy: Variant::Constant(cy.into()),
            rx: Variant::Constant(rx.into()),
            ry: Variant::Constant(ry.into()),
        }
    }

    /// Create a new Ellipse instance with variable values.
    pub fn variable(cx: usize, cy: usize, rx: usize, ry: usize) -> Self {
        Self {
            cx: Variant::Register(cx),
            cy: Variant::Register(cy),
            rx: Variant::Register(rx),
            ry: Variant::Register(ry),
        }
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

    /// Reset cy property.
    pub fn cy<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.cy = Variant::Constant(value.into());
        self
    }

    /// Reset cy property to register variant.
    pub fn cy_variable(mut self, id: usize) -> Self {
        self.cy = Variant::Register(id);
        self
    }

    /// Reset rx property.
    pub fn rx<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.rx = Variant::Constant(value.into());
        self
    }

    /// Reset rx property to register variant.
    pub fn rx_variable(mut self, id: usize) -> Self {
        self.rx = Variant::Register(id);
        self
    }

    /// Reset ry property.
    pub fn ry<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.ry = Variant::Constant(value.into());
        self
    }

    /// Reset ry property to register variant.
    pub fn ry_variable(mut self, id: usize) -> Self {
        self.ry = Variant::Register(id);
        self
    }
}

/// The ‘line’ element defines a line segment that starts at one point and ends at another.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    /// The x-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x1: Variant<Measurement>,

    /// The y-axis coordinate of the start of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y1: Variant<Measurement>,

    /// The x-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub x2: Variant<Measurement>,

    /// The y-axis coordinate of the end of the line.
    ///
    /// If the attribute is not specified, the effect is as if a value of "0" were specified.
    ///
    /// Animatable: yes.
    pub y2: Variant<Measurement>,
}

impl Line {
    /// Create a new Line instance with constant value.
    pub fn new<X1, Y1, X2, Y2>(x1: X1, y1: Y1, x2: X2, y2: Y2) -> Self
    where
        Measurement: From<X1> + From<Y1> + From<X2> + From<Y2>,
    {
        Self {
            x1: Variant::Constant(x1.into()),
            y1: Variant::Constant(y1.into()),
            x2: Variant::Constant(x2.into()),
            y2: Variant::Constant(y2.into()),
        }
    }

    /// Create a new Line instance with variable values.
    pub fn variable(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            x1: Variant::Register(x1),
            y1: Variant::Register(y1),
            x2: Variant::Register(x2),
            y2: Variant::Register(y2),
        }
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
}

/// The ‘polyline’ element defines a set of connected straight line segments. Typically, ‘polyline’ elements define open shapes.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polyline(
    /// The points that make up the polyline. All coordinate values are in the user coordinate system.
    ///
    /// Animatable: yes.
    Variant<Vec<Variant<Point>>>,
);

impl Polyline {
    /// Create a new Line instance with constant value.
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator,
        Variant<Point>: From<I::Item>,
    {
        Self(Variant::Constant(
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
    Variant<Vec<Variant<Point>>>,
);

impl Polygon {
    /// Create a new polygon instance with constant value.
    pub fn new<I>(points: I) -> Self
    where
        I: IntoIterator,
        Variant<Point>: From<I::Item>,
    {
        Self(Variant::Constant(
            points.into_iter().map(|v| v.into()).collect(),
        ))
    }
}
