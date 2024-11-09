use crate::{Angle, Canvas, Length, Point, Rgba, Transform};

/// A rendering target must implement this trait.
///
/// # immediately / scoped instructions
///
/// All scoped instructions are created by `push_*` functions and removed by [`pop`](Renderer::pop) function.
///
/// immediately instructions are created by other functions.
pub trait Renderer {
    /// Error type returns by [`submit`](Renderer::submit) function.
    type Error;

    /// Clear up the rendering target.
    fn clear(&mut self);

    /// Pop n instructions from graphics stack.
    fn pop(&mut self, n: usize);

    /// Push `entity` instruction into stack.
    fn push_entity(&mut self, id: &str);

    /// Push a `canvas` instruction.
    fn push_canvas(&mut self, canvas: Canvas);

    /// Push a `path` instruction into graphics stack.
    fn push_path(&mut self);

    /// Push a `transform` instruction into graphics stack.
    fn push_transform(&mut self, transform: Transform);

    /// Push a `fill` instruction into graphics stack.
    fn push_fill(&mut self, color: Rgba);

    /// Push a `stroke` instruction into graphics stack.
    fn push_stroke(&mut self, color: Rgba, width: Length);

    /// Push a debug `label` instruction into graphics stack.
    fn push_label(&mut self, label: &str);

    /// Attach a entity into the rendering tree.
    fn entity_ref(&mut self, id: &str);

    /// Move current point to `to` point.
    fn move_to(&mut self, to: Point);

    /// Draw a line on the target.
    fn line(&mut self, from: Option<Point>, to: Point);

    /// Draw a quadratic bezier curve on the target.
    fn quadratic_bezier(&mut self, from: Option<Point>, ctrl: Point, to: Point);

    /// Draw a cubic bezier curve on the target.
    fn cubic_bezier(&mut self, from: Option<Point>, ctrl1: Point, ctrl2: Point, to: Point);

    /// Draw an elliptic arc curve segment.
    fn arc(
        &mut self,
        center: Option<Point>,
        raddii: (Length, Length),
        start_angle: Angle,
        sweep_angle: Angle,
        x_rotation: Angle,
    );

    /// Submits a series of graphics instructions for execution.
    fn submit(&mut self) -> Result<(), Self::Error>;
}
