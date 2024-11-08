use crate::{Angle, Length, Point, PreserveAspectRatio, Rgba, Transform};

/// A rendering target must implement this trait.
pub trait Renderer {
    /// Error type returns by [`submit`](Renderer::submit) function.
    type Error;

    /// Clear all graphics instructions in the stack.
    fn clear(&mut self);

    /// Pop n instructions from graphics stack.
    fn pop(&mut self, n: usize);

    /// Push a `canvas` instruction.
    fn push_canvas(&mut self, width: Length, height: Length);
    /// Push a `viewbox` instruction.
    fn push_viewbox(&mut self, width: Length, height: Length);

    /// Push a `PreserveAspectRatio` instruction.
    fn push_preserve_aspect_ratio(&mut self, ratio: PreserveAspectRatio);

    /// Push a `transform` instruction into graphics stack.
    fn push_transform(&mut self, transform: Transform);

    /// Push a `fill` instruction into graphics stack.
    fn push_fill(&mut self, color: Rgba);

    /// Push a `stroke` instruction into graphics stack.
    fn push_stroke(&mut self, color: Rgba, width: Length);

    /// Draw a line on the target.
    fn line(&mut self, from: Point, to: Point);

    /// Draw a quadratic bezier curve on the target.
    fn quadratic_bezier(&mut self, from: Point, ctrl: Point, to: Point);

    /// Draw a cubic bezier curve on the target.
    fn cubic_bezier(&mut self, from: Point, ctrl1: Point, ctrl2: Point, to: Point);

    /// Draw an elliptic arc curve segment.
    fn arc(
        &mut self,
        center: Point,
        raddii: (Length, Length),
        start_angle: Angle,
        sweep_angle: Angle,
        x_rotation: Angle,
    );

    /// Submits a series of graphics instructions for execution.
    fn submit(&mut self) -> Result<(), Self::Error>;
}
