use super::{Angle, Measurement, Point, Variable, Variant};

/// A direction that representation a path drawing commander.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    // (absolute) Start a new sub-path at the given (x,y) coordinate.
    MoveTo(Variant<Point>),
    /// Close the current subpath by drawing a straight line from the current point to current subpath's initial point.
    ClosePath,
    /// Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineTo(Variant<Point>),
    /// Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    Polyline(Variant<Vec<Variant<Point>>>),

    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezier {
        ctrl1: Variant<Point>,
        ctrl2: Variant<Point>,
        to: Variant<Point>,
    },

    /// Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezier {
        ctrl: Variant<Point>,
        to: Variant<Point>,
    },

    /// Draws an elliptical arc from the current point to `to` point.
    ///
    /// The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters.
    Arc {
        /// The size and orientation of the ellipse are defined by two radii (rx, ry) and an x_rotation,
        /// which indicates how the ellipse as a whole is rotated relative to the current coordinate system.
        rx: Variant<Measurement>,
        /// See [`rx`](PathEvent::Arc::rx)
        ry: Variant<Measurement>,
        /// See [`rx`](PathEvent::Arc::rx)
        x_rotation: Variant<Angle>,
        /// `large_arc` and [`sweep`](PathEvent::Arc::sweep) contribute to the automatic calculations
        /// and help determine how the arc is drawn.
        large_arc: bool,
        /// See [`large_arc`](PathEvent::Arc::large_arc)
        sweep: bool,
        /// Draws an elliptical arc from the current point to `to` point.
        to: Variant<Point>,
    },
}
impl Variable for PathEvent {}

/// Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,
/// or any combination of the three.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    /// The definition of the outline of a shape.
    pub data: Variant<Vec<Variant<PathEvent>>>,

    /// The author's computation of the total length of the path, in user units.
    /// This value is used to calibrate the user agent's own distance-along-a-path
    /// calculations with that of the author. The user agent will scale all
    /// distance-along-a-path computations by the ratio of ‘pathLength’ to the user
    /// agent's own computed value for total path length. ‘pathLength’ potentially
    /// affects calculations for text on a path, motion animation and various stroke
    /// operations.
    ///
    /// A negative value is an error (see Error processing).
    pub length: Variant<Measurement>,
}

impl Path {
    /// Reset data property.
    pub fn data<I>(mut self, data: I) -> Self
    where
        I: IntoIterator,
        PathEvent: From<I::Item>,
    {
        self.data = Variant::Constant(
            data.into_iter()
                .map(|v| Variant::Constant(v.into()))
                .collect(),
        );
        self
    }

    /// Reset data property to register variant.
    pub fn data_variable(mut self, id: usize) -> Self {
        self.data = Variant::Register(id);
        self
    }

    /// Reset length property.
    pub fn length<V>(mut self, value: V) -> Self
    where
        Measurement: From<V>,
    {
        self.length = Variant::Constant(value.into());
        self
    }

    /// Reset length property to register variant.
    pub fn length_variable(mut self, id: usize) -> Self {
        self.length = Variant::Register(id);
        self
    }
}
