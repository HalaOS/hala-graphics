use crate::euclid::Mat4;
use alloc::{boxed::Box, vec::Vec};

use super::geometry::{Offset, RRect, Radius, Rect};

/// Sub-paths consist of segments of various types,
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum SubPath {
    LineTo(Offset),
    RelativeLineTo(Offset),
    MoveTo(Offset),
    RelativeMoveTo(Offset),
    Arc(Box<Arc>),
    ArcTo(Box<ArcTo>),
    ArcToPoint(Box<ArcToPoint>),
    RelativeArcToPoint(Box<ArcToPoint>),
    Oval(Box<Rect>),
    Path(Box<ExtendWithPath>),
    ExtendWithPath(Box<ExtendWithPath>),
    ConicTo(Box<Conic>),
    RelativeConicTo(Box<Conic>),
    CubicTo(Box<Cubic>),
    RelativeCubicTo(Box<Cubic>),
    Close,
    RRect(Box<RRect>),
    QuadraticBezierTo(Box<QuadraticBezier>),
    RelativeQuadraticBezierTo(Box<QuadraticBezier>),
    Shift(Offset),
    Transform(Box<Mat4>),
}

/// A conic bezier subpath parameter
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Conic {
    pub p1: Offset,
    pub p2: Offset,
    /// If the weight is greater than 1, then the curve is a hyperbola; if the weight equals 1,
    /// it's a parabola; and if it is less than 1, it is an ellipse.
    pub w: f32,
}

/// A cubic bezier subpath parameter
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Cubic {
    pub p1: Offset,
    pub p2: Offset,
    pub p3: Offset,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Arc {
    pub oval: Rect,
    pub start_angle: f32,
    pub sweep_angle: f32,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ArcTo {
    pub oval: Rect,
    pub start_angle: f32,
    pub sweep_angle: f32,
    pub force_move_to: bool,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ArcToPoint {
    pub arc_end: Offset,
    pub radius: Option<Radius>,
    pub rotation: Option<f32>,
    pub large_arc: Option<bool>,
    pub clockwise: Option<bool>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ExtendWithPath {
    pub path: Path,
    pub offset: Offset,
    pub matrix4: Option<Mat4>,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct QuadraticBezier {
    pub p1: Offset,
    pub p2: Offset,
}

/// A complex, one-dimensional subset of a plane.
/// A path consists of a number of sub-paths, and a current point.
/// Sub-paths consist of segments of various types, such as lines, arcs, or beziers. Sub-paths can be open or closed, and can self-intersect.
/// Closed sub-paths enclose a (possibly discontiguous) region of the plane based on the current fillType.
/// The current point is initially at the origin. After each operation adding a segment to a sub-path, the current point is updated to the end of that segment.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Path {
    subpathes: Vec<SubPath>,
}

#[allow(unused)]
impl Path {
    /// Adds a new sub-path with one arc segment that consists of the arc that follows the edge of the oval bounded by the given rectangle,
    /// from startAngle radians around the oval up to startAngle + sweepAngle radians around the oval, with zero radians being the point on
    /// the right hand side of the oval that crosses the horizontal line that intersects the center of the rectangle and with positive angles
    /// going clockwise around the oval.
    pub fn add_arc<R: Into<Rect>>(&mut self, oval: R, start_angle: f32, sweep_angle: f32) {
        let arc = Arc {
            oval: oval.into(),
            start_angle,
            sweep_angle,
        };

        self.subpathes.push(SubPath::Arc(Box::new(arc)));
    }

    /// Adds a new sub-path that consists of a curve that forms the ellipse that fills the given rectangle.
    pub fn add_oval<R: Into<Rect>>(&mut self, oval: R) {
        self.subpathes.push(SubPath::Oval(Box::new(oval.into())));
    }

    /// Adds the sub-paths of path, offset by offset, to this path.
    pub fn add_path<P: Into<Offset>>(&mut self, path: Path, offset: P, matrix4: Option<Mat4>) {
        let extend_with_path = ExtendWithPath {
            path,
            offset: offset.into(),
            matrix4,
        };

        self.subpathes
            .push(SubPath::Path(Box::new(extend_with_path)));
    }

    /// Adds a new sub-path with a sequence of line segments that connect the given points.
    pub fn add_polygon<V: Into<Vec<P>>, P: Into<Offset>>(&mut self, points: V, close: bool) {
        let points: Vec<P> = points.into();

        let mut start_point = None;

        for (index, p) in points.into_iter().enumerate() {
            let p: Offset = p.into();

            if index == 0 && close {
                start_point = Some(p);
            }

            self.line_to(p);
        }

        if let Some(start_point) = start_point {
            self.line_to(start_point);
        }
    }

    /// Adds a new sub-path that consists of four lines that outline the given rectangle.
    pub fn add_rect<R: Into<Rect>>(&mut self, rect: R) {
        let rect: Rect = rect.into();

        self.line_to(rect.top_left());
        self.line_to(rect.top_right());
        self.line_to(rect.bottom_right());
        self.line_to(rect.bottom_left());
        self.line_to(rect.top_left());
    }
    /// Adds a new sub-path that consists of the straight lines and curves needed to form the rounded rectangle described by the argument.
    pub fn add_rrect<RR: Into<RRect>>(&mut self, rrect: RR) {
        // let rrect: RRect = rrect.into();
        self.subpathes.push(SubPath::RRect(Box::new(rrect.into())));
    }

    /// If the forceMoveTo argument is false, adds a straight line segment and an arc segment.              
    pub fn arc_to<R: Into<Rect>>(
        &mut self,
        oval: R,
        start_angle: f32,
        sweep_angle: f32,
        force_move_to: bool,
    ) {
        let arc_to = ArcTo {
            oval: oval.into(),
            start_angle,
            sweep_angle,
            force_move_to,
        };

        self.subpathes.push(SubPath::ArcTo(Box::new(arc_to)));
    }

    /// Adds the sub-paths of path, offset by offset, to this path.
    /// The current sub-path is extended with the first sub-path of path, connecting them with a lineTo if necessary.
    pub fn extend_with_path<P: Into<Offset>>(
        &mut self,
        path: Path,
        offset: P,
        matrix4: Option<Mat4>,
    ) {
        let extend_with_path = ExtendWithPath {
            path,
            offset: offset.into(),
            matrix4,
        };

        self.subpathes
            .push(SubPath::ExtendWithPath(Box::new(extend_with_path)));
    }

    /// Closes the last sub-path, as if a straight line had been drawn from the current point to the first point of the sub-path.
    pub fn close(&mut self) {
        self.subpathes.push(SubPath::Close);
    }

    /// Appends up to four conic curves weighted to describe an oval of radius and rotated by rotation (measured in degrees and clockwise).
    pub fn arc_to_point<P: Into<Offset>>(
        &mut self,
        arc_end: P,
        radius: Option<Radius>,
        rotation: Option<f32>,
        large_arc: Option<bool>,
        clockwise: Option<bool>,
    ) {
        let arc_to_point = ArcToPoint {
            arc_end: arc_end.into(),
            radius,
            rotation,
            large_arc,
            clockwise,
        };

        self.subpathes
            .push(SubPath::ArcToPoint(Box::new(arc_to_point)));
    }

    /// Adds a bezier segment that curves from the current point to the given point (x2,y2),
    /// using the control points (x1,y1) and the weight w.
    /// If the weight is greater than 1, then the curve is a hyperbola; if the weight equals 1,
    /// it's a parabola; and if it is less than 1, it is an ellipse.
    pub fn conic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, w: f32) {
        let conic = Conic {
            p1: (x1, x2).into(),
            p2: (x2, y2).into(),
            w,
        };

        self.subpathes.push(SubPath::ConicTo(Box::new(conic)));
    }

    /// Adds a cubic bezier segment that curves from the current point to the given point (x3,y3),
    /// using the control points (x1,y1) and (x2,y2).
    pub fn cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let cubic = Cubic {
            p1: (x1, y1).into(),
            p2: (x2, y2).into(),
            p3: (x3, y3).into(),
        };

        self.subpathes.push(SubPath::CubicTo(Box::new(cubic)));
    }

    /// Adds a straight line segment from the current point to the given point.
    pub fn line_to<P: Into<Offset>>(&mut self, point: P) {
        self.subpathes.push(SubPath::LineTo(point.into()));
    }

    /// Starts a new sub-path at the given coordinate.
    pub fn move_to<P: Into<Offset>>(&mut self, point: P) {
        self.subpathes.push(SubPath::MoveTo(point.into()));
    }

    /// Adds a quadratic bezier segment that curves from the current point to the given point (x2,y2), using the control point (x1,y1).
    pub fn quadratic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let bezier = QuadraticBezier {
            p1: (x1, y1).into(),
            p2: (x2, y2).into(),
        };

        self.subpathes
            .push(SubPath::QuadraticBezierTo(Box::new(bezier)));
    }

    /// Appends up to four conic curves weighted to describe an oval of radius and rotated by rotation (measured in degrees and clockwise).

    pub fn relative_arc_to_point<P: Into<Offset>>(
        &mut self,
        arc_end_delta: P,
        radius: Option<Radius>,
        rotation: Option<f32>,
        large_arc: Option<bool>,
        clockwise: Option<bool>,
    ) {
    }

    /// Adds a bezier segment that curves from the current point to the point at the offset (x2,y2) from the current point,
    /// using the control point at the offset (x1,y1) from the current point and the weight w. If the weight is greater than 1,
    /// then the curve is a hyperbola; if the weight equals 1, it's a parabola; and if it is less than 1, it is an ellipse.
    pub fn relative_conic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, w: f32) {
        let conic = Conic {
            p1: (x1, x2).into(),
            p2: (x2, y2).into(),
            w,
        };

        self.subpathes
            .push(SubPath::RelativeConicTo(Box::new(conic)));
    }

    /// Adds a cubic bezier segment that curves from the current point to the point at the offset (x3,y3) from the current point,
    /// using the control points at the offsets (x1,y1) and (x2,y2) from the current point.
    pub fn relative_cubic_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        let cubic = Cubic {
            p1: (x1, y1).into(),
            p2: (x2, y2).into(),
            p3: (x3, y3).into(),
        };

        self.subpathes
            .push(SubPath::RelativeCubicTo(Box::new(cubic)));
    }

    /// Adds a straight line segment from the current point to the point at the given offset from the current point.
    pub fn relative_line_to<P: Into<Offset>>(&mut self, point: P) {
        self.subpathes.push(SubPath::RelativeLineTo(point.into()));
    }

    /// Starts a new sub-path at the given offset from the current point.
    pub fn relative_move_to<P: Into<Offset>>(&mut self, point: P) {
        self.subpathes.push(SubPath::RelativeMoveTo(point.into()));
    }

    /// Adds a quadratic bezier segment that curves from the current point to the point at the offset (x2,y2) from the current point,
    /// using the control point at the offset (x1,y1) from the current point.
    pub fn relative_quadratic_bezier_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let bezier = QuadraticBezier {
            p1: (x1, y1).into(),
            p2: (x2, y2).into(),
        };

        self.subpathes
            .push(SubPath::RelativeQuadraticBezierTo(Box::new(bezier)));
    }

    /// Clears the Path object of all sub-paths, returning it to the same state it had when it was created. The current point is reset to the origin.
    pub fn reset(&mut self) {
        self.subpathes.clear();
    }

    /// Returns a copy of the path with all the segments of every sub-path translated by the given offset.
    pub fn shift<P: Into<Offset>>(&mut self, offset: P) -> Path {
        let mut cloned = self.clone();

        cloned.subpathes.push(SubPath::Shift(offset.into()));

        cloned
    }

    /// Returns a copy of the path with all the segments of every sub-path transformed by the given matrix.
    pub fn transform(&mut self, mat4: Mat4) -> Path {
        let mut cloned = self.clone();

        cloned.subpathes.push(SubPath::Transform(Box::new(mat4)));

        cloned
    }
}
