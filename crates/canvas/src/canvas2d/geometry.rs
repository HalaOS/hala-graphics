/// An immutable 2D floating-point offset.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Offset {
    pub dx: f32,
    pub dy: f32,
}

impl From<(f32, f32)> for Offset {
    fn from(value: (f32, f32)) -> Self {
        Offset {
            dx: value.0,
            dy: value.1,
        }
    }
}

impl core::ops::Add for Offset {
    type Output = Offset;
    fn add(self, rhs: Self) -> Self::Output {
        Offset {
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl Offset {
    /// coordinates of this point are finite.
    pub fn is_finite(&self) -> bool {
        self.dx.is_finite() && self.dy.is_finite()
    }

    /// Creates an offset from its `direction` and `distance`.
    ///
    /// The direction is in radians clockwise from the positive x-axis.
    ///
    pub fn from_direction(direction: f32, distance: f32) -> Self {
        Self {
            dx: libm::cosf(distance * direction),
            dy: libm::sinf(distance * direction),
        }
    }

    /// The magnitude of the offset.
    ///
    /// If you need this value to compare it to another [Offset]'s distance,
    /// consider using [`Offset::distance_sequared`] instead, since it is cheaper to compute.
    pub fn distance(&self) -> f32 {
        libm::sqrtf(self.dx * self.dx + self.dy * self.dy)
    }

    pub fn distance_sequared(&self) -> f32 {
        self.dx * self.dx + self.dy * self.dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl From<(f32, f32)> for Size {
    fn from(value: (f32, f32)) -> Size {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}

impl Size {
    /// coordinates of this size are finite.
    pub fn is_finite(&self) -> bool {
        self.width.is_finite() && self.height.is_finite()
    }
    /// Creates a square [Size] whose [width](Size::width) and [height](Size::height) are the given dimension.
    ///
    /// See also:
    ///
    ///  * [`Size::from_radius`], which is more convenient when the available size
    ///    is the radius of a circle.
    pub fn square(dimension: f32) -> Self {
        Size {
            width: dimension,
            height: dimension,
        }
    }

    /// Creates a [Size] with the given [width](Size::width) and an infinite [height](Size::height).
    pub fn from_width(width: f32) -> Self {
        Size {
            width,
            height: f32::INFINITY,
        }
    }

    /// Creates a [Size] with the given [height](Size::height) and an infinite [width](Size::width).
    pub fn from_height(height: f32) -> Self {
        Size {
            width: f32::INFINITY,
            height,
        }
    }
    /// Creates a square [Size] whose [width](Size::width) and [height](Size::height) are twice the given
    /// dimension.
    ///
    /// This is a square that contains a circle with the given radius.
    ///
    /// See also:
    ///
    ///  * [Size::square], which creates a square with the given dimension.
    pub fn from_radius(radius: f32) -> Self {
        Size {
            width: radius * 2.0,
            height: radius * 2.0,
        }
    }
}

/// An immutable, 2D, axis-aligned, floating-point rectangle whose coordinates are relative to a given origin.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Rect {
    pub offset: Offset,
    pub size: Size,
}

impl<S: Into<Size>> core::ops::Add<S> for Offset {
    type Output = Rect;

    fn add(self, rhs: S) -> Self::Output {
        Rect {
            offset: self,
            size: rhs.into(),
        }
    }
}

impl Rect {
    /// Constructs a rectangle from its center point, width, and height.
    /// The center argument is assumed to be an offset from the origin.
    pub fn from_center<P: Into<Offset>>(center: P, width: f32, height: f32) -> Self {
        let center: Offset = center.into();

        Rect {
            offset: (
                center.dx / 2f32 - width / 2f32,
                center.dy / 2f32 - height / 2f32,
            )
                .into(),
            size: (width, height).into(),
        }
    }

    /// Construct a rectangle that bounds the given circle.
    pub fn from_circle<P: Into<Offset>>(center: P, radius: f32) -> Self {
        Self::from_center(center, radius * 2.0, radius * 2.0)
    }

    /// Construct a rectangle from its left, top, right, and bottom edges.
    pub fn from_ltrb(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            offset: (left, top).into(),
            size: (right - left, bottom - top).into(),
        }
    }

    /// Construct a rectangle from its left and top edges, its width, and its height.
    pub fn from_ltwh(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self {
            offset: (left, top).into(),
            size: (width, height).into(),
        }
    }

    /// Construct the smallest rectangle that encloses the given offsets, treating them as vectors from the origin.
    pub fn from_points<P1: Into<Offset>, P2: Into<Offset>>(a: P1, b: P2) -> Self {
        let a: Offset = a.into();
        let b: Offset = b.into();

        Self::from_ltrb(
            f32::min(a.dx, b.dx),
            f32::min(a.dy, b.dy),
            f32::max(a.dx, b.dx),
            f32::max(a.dy, b.dy),
        )
    }

    /// The offset of the bottom edge of this rectangle from the y axis.
    pub fn bottom(&self) -> f32 {
        self.offset.dy + self.size.height
    }

    /// The offset to the center of the bottom edge of this rectangle.
    pub fn bottom_center(&self) -> Offset {
        (
            self.offset.dx + self.size.width / 2.0,
            self.offset.dy + self.size.height,
        )
            .into()
    }

    /// The offset to the intersection of the bottom and left edges of this rectangle.
    pub fn bottom_left(&self) -> Offset {
        (self.offset.dx, self.offset.dy + self.size.height).into()
    }

    /// The offset to the intersection of the bottom and right edges of this rectangle.
    pub fn bottom_right(&self) -> Offset {
        (
            self.offset.dx + self.size.width,
            self.offset.dy + self.size.height,
        )
            .into()
    }

    /// The offset to the point halfway between the left and right and the top and bottom edges of this rectangle.
    pub fn center(&self) -> Offset {
        (
            self.offset.dx + self.size.width / 2.0,
            self.offset.dy + self.size.height / 2.0,
        )
            .into()
    }

    pub fn center_left(&self) -> Offset {
        (self.offset.dx, self.offset.dy + self.size.height / 2.0).into()
    }

    /// The offset to the center of the right edge of this rectangle.
    pub fn center_right(&self) -> Offset {
        (
            self.offset.dx + self.size.width,
            self.offset.dy + self.size.height / 2.0,
        )
            .into()
    }

    /// The distance between the top and bottom edges of this rectangle.
    pub fn height(&self) -> f32 {
        self.size.height
    }

    /// The offset of the left edge of this rectangle from the x axis.
    pub fn left(&self) -> f32 {
        self.offset.dx
    }

    /// The greater of the magnitudes of the width and the height of this rectangle.
    pub fn longest_side(&self) -> f32 {
        f32::max(self.size.width, self.size.height)
    }

    /// The lesser of the magnitudes of the width and the height of this rectangle.
    pub fn shortest_side(&self) -> f32 {
        f32::min(self.size.width, self.size.height)
    }

    /// The offset of the right edge of this rectangle from the x axis.
    pub fn right(&self) -> f32 {
        self.offset.dx + self.size.width
    }

    /// The offset of the top edge of this rectangle from the y axis.
    pub fn top(&self) -> f32 {
        self.offset.dy
    }

    /// The offset to the center of the top edge of this rectangle.
    pub fn top_center(&self) -> Offset {
        (self.offset.dx + self.size.width / 2.0, self.offset.dy).into()
    }

    /// The offset to the intersection of the top and left edges of this rectangle.
    pub fn top_left(&self) -> Offset {
        self.offset
    }

    /// The offset to the intersection of the top and right edges of this rectangle.
    pub fn top_right(&self) -> Offset {
        (self.offset.dx + self.size.width, self.offset.dy).into()
    }

    /// The distance between the left and right edges of this rectangle.
    pub fn width(&self) -> f32 {
        self.size.width
    }

    /// Whether this rectangle encloses a non-zero area. Negative areas are considered empty.
    pub fn is_empty(&self) -> bool {
        self.left() >= self.right() || self.top() >= self.bottom()
    }

    /// Whether all coordinates of this rectangle are finite.
    pub fn is_finite(&self) -> bool {
        self.offset.is_finite() && self.size.is_finite()
    }

    /// Whether any of the coordinates of this rectangle are equal to positive infinity.
    pub fn is_infinite(&self) -> bool {
        !self.is_finite()
    }

    /// Whether the point specified by the given offset (which is assumed to be relative to the origin)
    /// lies between the left and right and the top and bottom edges of this rectangle.
    pub fn contains<P: Into<Offset>>(&self, offset: P) -> bool {
        let offset = offset.into();

        self.right() > offset.dx
            && self.left() <= offset.dx
            && self.bottom() > offset.dy
            && self.top() <= offset.dy
    }

    /// Returns a new rectangle with edges moved outwards by the given delta.
    pub fn inflate(&self, delta: f32) -> Rect {
        Self::from_ltrb(
            self.left() - delta,
            self.top() - delta,
            self.right() + delta,
            self.bottom() + delta,
        )
    }

    /// Returns a new rectangle with edges moved inwards by the given delta.
    pub fn deflate(&self, delta: f32) -> Rect {
        self.inflate(-delta)
    }

    /// Returns a new rectangle which is the bounding box containing this rectangle and the given rectangle.
    pub fn expand_to_include<R: Into<Rect>>(&self, other: R) -> Rect {
        let other = other.into();

        Self::from_ltrb(
            f32::min(self.left(), other.left()),
            f32::min(self.top(), other.top()),
            f32::max(self.right(), other.right()),
            f32::max(self.bottom(), other.bottom()),
        )
    }

    /// Returns a new rectangle that is the intersection of the given rectangle and this rectangle.
    /// The two rectangles must overlap for this to be meaningful. If the two rectangles do not overlap,
    /// then the resulting Rect will have a negative width or height.
    pub fn intersect<R: Into<Rect>>(&self, other: R) -> Rect {
        let other = other.into();

        Self::from_ltrb(
            f32::max(self.left(), other.left()),
            f32::max(self.top(), other.top()),
            f32::min(self.right(), other.right()),
            f32::min(self.bottom(), other.bottom()),
        )
    }

    /// Whether other has a nonzero area of overlap with this rectangle.
    pub fn overlaps<R: Into<Rect>>(&self, other: R) -> bool {
        let other = other.into();

        if self.right() <= other.left() || other.right() <= self.left() {
            return false;
        }
        if self.bottom() <= other.top() || other.bottom() <= self.top() {
            return false;
        }
        return true;
    }

    /// Returns a new rectangle translated by the given offset.
    pub fn shift<P: Into<Offset>>(&self, offset: P) -> Self {
        let offset = offset.into();

        Self {
            offset: self.offset + offset,
            size: self.size,
        }
    }

    /// Returns a new rectangle with translateX added to the x components and translateY added to the y components.
    /// To translate a rectangle by an Offset rather than by separate x and y components, consider shift.

    pub fn translate(&self, translate_x: f32, translate_y: f32) -> Self {
        Self::shift(&self, (translate_x, translate_y))
    }
}

impl From<(Offset, Offset)> for Rect {
    fn from(value: (Offset, Offset)) -> Self {
        Self::from_points(value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[allow(dead_code)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RRect {
    pub rect: Rect,
    pub tl_radius: Radius,
    pub tr_radius: Radius,
    pub bl_radius: Radius,
    pub br_radius: Radius,
}

impl RRect {
    pub fn new<
        R: Into<Rect>,
        TL: Into<Radius>,
        TR: Into<Radius>,
        BL: Into<Radius>,
        BR: Into<Radius>,
    >(
        rect: R,
        tl: Option<TL>,
        tr: Option<TR>,
        bl: Option<BL>,
        br: Option<BR>,
    ) -> Self {
        RRect {
            rect: rect.into(),
            tl_radius: tl.map(|r| r.into()).unwrap_or(Default::default()),
            tr_radius: tr.map(|r| r.into()).unwrap_or(Default::default()),
            bl_radius: bl.map(|r| r.into()).unwrap_or(Default::default()),
            br_radius: br.map(|r| r.into()).unwrap_or(Default::default()),
        }
    }
}

/// A radius for either circular or elliptical shapes.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serialization",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum Radius {
    /// Constructs a circular radius. x and y will have the same radius value.
    Circular(f32),
    /// Constructs an elliptical radius with the given radii.
    Elliptical { x: f32, y: f32 },
}

impl Default for Radius {
    fn default() -> Self {
        Radius::Circular(0f32)
    }
}

impl From<(f32, f32)> for Radius {
    fn from(value: (f32, f32)) -> Self {
        Radius::Elliptical {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<f32> for Radius {
    fn from(value: f32) -> Self {
        Radius::Circular(value)
    }
}

impl core::ops::Add<f32> for Radius {
    type Output = Radius;

    fn add(self, rhs: f32) -> Self::Output {
        match self {
            Radius::Circular(radius) => Radius::Circular(radius + rhs),
            Radius::Elliptical { x, y } => Radius::Elliptical {
                x: x + rhs,
                y: y + rhs,
            },
        }
    }
}
