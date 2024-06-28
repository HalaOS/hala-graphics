use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::euclid::Mat4;
use crate::image::Image;

use super::{
    geometry::{Offset, RRect, Rect},
    paint::{BlendMode, Color, Paint},
    paragraph::Paragraph,
    path::Path,
};

/// Defines how a new clip region should be merged with the existing clip region.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ClipOp {
    Difference,
    Intersect,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Layer {
    pub ops: Vec<CanvasOp>,
    pub bounds: Option<Rect>,
    pub paint: Paint,
}

impl From<Layer> for CanvasOp {
    fn from(value: Layer) -> Self {
        Self::Layer(Box::new(value))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Scale {
    pub sx: f32,
    pub sy: Option<f32>,
}

impl From<Scale> for CanvasOp {
    fn from(value: Scale) -> Self {
        Self::Scale(value)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Skew {
    pub sx: f32,
    pub sy: f32,
}

impl From<Skew> for CanvasOp {
    fn from(value: Skew) -> Self {
        Self::Skew(value)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawShadow {
    pub path: Path,
    pub color: Color,
    pub elevation: f32,
    pub transparent_occluder: bool,
}

impl From<DrawShadow> for CanvasOp {
    fn from(value: DrawShadow) -> Self {
        Self::DrawShadow(Box::new(value))
    }
}

/// Draw paragraph op.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawParagraph {
    pub paragraph: Paragraph,
    pub offset: Offset,
}

impl From<DrawParagraph> for CanvasOp {
    fn from(value: DrawParagraph) -> Self {
        Self::DrawParagraph(Box::new(value))
    }
}

/// Draw color op.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawColor {
    pub color: Color,
    pub blend_mode: BlendMode,
}

impl From<DrawColor> for CanvasOp {
    fn from(value: DrawColor) -> Self {
        Self::DrawColor(Box::new(value))
    }
}

/// Draw color op.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawPath {
    pub path: Path,
    pub paint: Paint,
}

impl From<DrawPath> for CanvasOp {
    fn from(value: DrawPath) -> Self {
        Self::DrawPath(Box::new(value))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawImage {
    pub image: Image,
    pub offset: Offset,
    pub paint: Paint,
}

impl From<DrawImage> for CanvasOp {
    fn from(value: DrawImage) -> Self {
        Self::DrawImage(Box::new(value))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct DrawImageRect {
    pub image: Image,
    pub src: Rect,
    pub dst: Rect,
    pub paint: Paint,
}

impl From<DrawImageRect> for CanvasOp {
    fn from(value: DrawImageRect) -> Self {
        Self::DrawImageRect(Box::new(value))
    }
}

/// The clip region path object.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ClipPath {
    pub path: Path,
    pub do_anti_alias: Option<bool>,
}

impl From<ClipPath> for CanvasOp {
    fn from(value: ClipPath) -> Self {
        Self::ClipPath(Box::new(value))
    }
}

/// The clip region path object.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ClipRect {
    pub rect: Rect,
    pub clip_op: Option<ClipOp>,
    pub do_anti_alias: Option<bool>,
}

impl From<ClipRect> for CanvasOp {
    fn from(value: ClipRect) -> Self {
        Self::ClipRect(Box::new(value))
    }
}

/// The clip region path object.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ClipRRect {
    pub rrect: RRect,
    pub do_anti_alias: Option<bool>,
}

impl From<ClipRRect> for CanvasOp {
    fn from(value: ClipRRect) -> Self {
        Self::ClipRRect(Box::new(value))
    }
}

/// Defines canvas opcode.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CanvasOp {
    ClipPath(Box<ClipPath>),
    ClipRect(Box<ClipRect>),
    ClipRRect(Box<ClipRRect>),
    DrawColor(Box<DrawColor>),
    DrawPath(Box<DrawPath>),
    DrawImage(Box<DrawImage>),
    DrawImageRect(Box<DrawImageRect>),
    DrawPaint(Box<Paint>),
    DrawParagraph(Box<DrawParagraph>),
    DrawShadow(Box<DrawShadow>),
    Transform(Box<Mat4>),
    Translate(Offset),
    Scale(Scale),
    Skew(Skew),
    Rotate(f32),
    Layer(Box<Layer>),
}

impl From<f32> for CanvasOp {
    fn from(value: f32) -> Self {
        Self::Rotate(value)
    }
}

impl From<Paint> for CanvasOp {
    fn from(value: Paint) -> Self {
        Self::DrawPaint(Box::new(value))
    }
}

impl From<Mat4> for CanvasOp {
    fn from(value: Mat4) -> Self {
        Self::Transform(Box::new(value))
    }
}

impl From<Offset> for CanvasOp {
    fn from(value: Offset) -> Self {
        Self::Translate(value)
    }
}

/// An interface for recording graphical operations.
#[allow(unused)]
#[derive(Debug, Clone)]
struct Canvas {
    ops: Vec<CanvasOp>,
    ops_stacks: Vec<Vec<CanvasOp>>,
    layer_stacks: Vec<Layer>,
}

#[allow(unused)]
impl Canvas {
    /// Reduces the clip region to the intersection of the current clip and the given [`Path`](Path).
    #[inline]
    pub fn clip_path(&mut self, path: Path, do_anti_alias: Option<bool>) {
        self.ops.push(
            ClipPath {
                path,
                do_anti_alias,
            }
            .into(),
        )
    }

    /// Reduces the clip region to the intersection of the current clip and the given rectangle.
    #[inline]
    pub fn clip_rect<R: Into<Rect>>(
        &mut self,
        rect: R,
        clip_op: Option<ClipOp>,
        do_anti_alias: Option<bool>,
    ) {
        self.ops.push(
            ClipRect {
                rect: rect.into(),
                clip_op,
                do_anti_alias,
            }
            .into(),
        )
    }

    /// Reduces the clip region to the intersection of the current clip and the given rounded rectangle.
    #[inline]
    pub fn clip_rrect<RR: Into<RRect>>(&mut self, rrect: RR, do_anti_alias: Option<bool>) {
        self.ops.push(
            ClipRRect {
                rrect: rrect.into(),
                do_anti_alias,
            }
            .into(),
        )
    }

    // /// Draw an arc scaled to fit inside the given rectangle.
    // pub fn draw_arc<R: Into<Rect>>(
    //     &mut self,
    //     rect: R,
    //     start_angle: f32,
    //     sweep_angle: f32,
    //     use_center: bool,
    //     paint: Paint,
    // ) {
    // }

    // /// Draws a circle centered at the point given by the first argument and that has the radius given by the second argument,
    // /// with the Paint given in the third argument. Whether the circle is filled or stroked (or both) is controlled by Paint.style.
    pub fn draw_circle<P: Into<Offset>>(&mut self, center: P, radius: f32, paint: Paint) {}

    /// Paints the given Color onto the canvas, applying the given BlendMode, with the given color being the source and the background being the destination.
    #[inline]
    pub fn draw_color<C: Into<Color>>(&mut self, color: C, blend_mode: BlendMode) {
        self.ops.push(
            DrawColor {
                color: color.into(),
                blend_mode,
            }
            .into(),
        )
    }

    /// Draws the given Image into the canvas with its top-left corner at the given Offset. The image is composited into the canvas using the given Paint.
    pub fn draw_image<P: Into<Offset>>(&mut self, image: Image, offset: P, paint: Paint) {
        self.ops.push(
            DrawImage {
                image,
                offset: offset.into(),
                paint,
            }
            .into(),
        )
    }

    /// Draws the subset of the given image described by the src argument into the canvas in the axis-aligned rectangle given by the dst argument.
    pub fn draw_image_rect<SR: Into<Rect>, DR: Into<Rect>>(
        &mut self,
        image: Image,
        src: SR,
        dst: DR,
        paint: Paint,
    ) {
        self.ops.push(
            DrawImageRect {
                image,
                src: src.into(),
                dst: dst.into(),
                paint,
            }
            .into(),
        )
    }

    /// Draws the given Path with the given Paint.
    pub fn draw_path(&mut self, path: Path, paint: Paint) {
        self.ops.push(DrawPath { path, paint }.into())
    }

    /// Fills the canvas with the given Paint.
    pub fn draw_paint(&mut self, paint: Paint) {
        self.ops.push(paint.into())
    }

    /// Draws the text in the given Paragraph into this canvas at the given [`Offset`].
    pub fn draw_paragraph<P: Into<Offset>>(&mut self, paragraph: Paragraph, offset: P) {
        self.ops.push(
            DrawParagraph {
                paragraph,
                offset: offset.into(),
            }
            .into(),
        )
    }

    /// Draws a shadow for a Path representing the given material elevation.
    pub fn draw_shadow<C: Into<Color>>(
        &mut self,
        path: Path,
        color: C,
        elevation: f32,
        transparent_occluder: bool,
    ) {
        self.ops.push(
            DrawShadow {
                path,
                color: color.into(),
                elevation,
                transparent_occluder,
            }
            .into(),
        )
    }

    // /// Returns the current transform including the combined result of all transform methods
    // /// executed since the creation of this Canvas object, and respecting the save/restore history.
    // pub fn get_transform(&self) -> Mat4 {
    //     todo!()
    // }

    /// Multiply the current transform by the specified 4⨉4 transformation matrix
    /// specified as a list of values in column-major order.
    pub fn transform(&mut self, matrix4: Mat4) {
        self.ops.push(matrix4.into())
    }

    /// Add a translation to the current transform, shifting the coordinate space horizontally by
    /// the first argument and vertically by the second argument.
    pub fn translate<P: Into<Offset>>(&mut self, delta: P) {
        self.ops.push(delta.into().into())
    }

    /// Add an axis-aligned scale to the current transform, scaling by the first argument in the
    /// horizontal direction and the second in the vertical direction.
    pub fn scale(&mut self, sx: f32, sy: Option<f32>) {
        self.ops.push(Scale { sx, sy }.into())
    }

    /// Add an axis-aligned skew to the current transform, with the first argument being the horizontal
    /// skew in rise over run units clockwise around the origin, and the second argument being the vertical skew in rise over run units clockwise around the origin.
    pub fn skew(&mut self, sx: f32, sy: f32) {
        self.ops.push(Skew { sx, sy }.into())
    }

    /// Add a rotation to the current transform. The argument is in radians clockwise.
    pub fn rotate(&mut self, radians: f32) {
        self.ops.push(radians.into())
    }

    /// Saves a copy of the current transform and clip on the save stack, and then creates a new group which subsequent calls will become a part of.
    /// When the save stack is later popped, the group will be flattened into a layer and have the given paint's Paint.colorFilter and Paint.blendMode applied.
    pub fn save_layer<R: Into<Rect>>(&mut self, bounds: Option<R>, paint: Paint) {
        self.layer_stacks.push(Layer {
            ops: Vec::new(),
            bounds: bounds.map(|r| r.into()),
            paint,
        });

        self.ops_stacks.push(self.ops.drain(0..).collect());
    }

    /// Pops the current save stack, if there is anything to pop. Otherwise, does nothing.
    pub fn restore(&mut self) {
        if let Some(mut layer) = self.layer_stacks.pop() {
            layer.ops = self.ops.drain(0..).collect();
            self.ops = self.ops_stacks.pop().unwrap();
            self.ops.push(layer.into());
        }
    }

    /// Returns the number of items on the save stack, including the initial state.
    pub fn get_layer_count(&self) -> usize {
        self.layer_stacks.len() + 1
    }

    /// Restores the save stack to a previous level as might be obtained from [`get_layer_count`](Canvas::get_layer_count).
    /// If count is less than 1, the stack is restored to its initial state.
    /// If count is greater than the current getSaveCount then nothing happens.
    pub fn restore_to(&mut self, count: usize) {
        let layer_count = self.get_layer_count();

        if layer_count < count {
            return;
        }

        for _ in 0..(layer_count - count - 1) {
            self.restore()
        }
    }
}
