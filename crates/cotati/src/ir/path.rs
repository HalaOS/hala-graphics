use super::{Angle, Animatable, FrameVariable, Measurement, Point};

/// A direction that representation a path drawing commander.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathEvent {
    // (absolute) Start a new sub-path at the given (x,y) coordinate.
    MoveTo(Point),
    /// Close the current subpath by drawing a straight line from the current point to current subpath's initial point.
    ClosePath,
    /// Draw a line from the current point to the given (x,y) coordinate which becomes the new current point.
    LineTo(Point),
    /// Draw a polyline. At the end of the command, the new current point is set to the final set of coordinates provided.
    Polyline(Vec<Point>),

    /// Draws a cubic Bézier curve from the current point to `to` point,
    /// using `ctrl1` as the control point at the beginning of the curve and `ctrl2` as the control point at the end of the curve.
    CubicBezier {
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
    },

    /// Draws a quadratic Bézier curve from the current point to `to` point using `ctrl` as the control point.
    QuadraticBezier {
        ctrl: Point,
        to: Point,
    },

    /// Draws an elliptical arc from the current point to `to` point.
    ///
    /// The center (cx, cy) of the ellipse is calculated automatically to satisfy the constraints
    /// imposed by the other parameters.
    Arc {
        /// The size and orientation of the ellipse are defined by two radii (rx, ry) and an x_rotation,
        /// which indicates how the ellipse as a whole is rotated relative to the current coordinate system.
        rx: Measurement,
        /// See [`rx`](PathEvent::Arc::rx)
        ry: Measurement,
        /// See [`rx`](PathEvent::Arc::rx)
        x_rotation: Angle,
        /// `large_arc` and [`sweep`](PathEvent::Arc::sweep) contribute to the automatic calculations
        /// and help determine how the arc is drawn.
        large_arc: bool,
        /// See [`large_arc`](PathEvent::Arc::large_arc)
        sweep: bool,
        /// Draws an elliptical arc from the current point to `to` point.
        to: Point,
    },
}
impl FrameVariable for PathEvent {}

/// Paths represent the outline of a shape which can be filled, stroked, used as a clipping path,
/// or any combination of the three.
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Path {
    /// The definition of the outline of a shape.
    pub data: Animatable<Vec<Animatable<PathEvent>>>,

    /// The author's computation of the total length of the path, in user units.
    /// This value is used to calibrate the user agent's own distance-along-a-path
    /// calculations with that of the author. The user agent will scale all
    /// distance-along-a-path computations by the ratio of ‘pathLength’ to the user
    /// agent's own computed value for total path length. ‘pathLength’ potentially
    /// affects calculations for text on a path, motion animation and various stroke
    /// operations.
    ///
    /// A negative value is an error (see Error processing).
    pub length: Animatable<Measurement>,
}
