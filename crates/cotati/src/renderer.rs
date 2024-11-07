use crate::{Angle, Length, Point, Rgba, Transform};

/// A rendering target must implement this trait.
pub trait Renderer {
    /// Pop n instructions from graphics stack.
    fn pop(&mut self, n: usize);

    /// Push a `transform` instruction into graphics stack.
    fn push_transform(&mut self, transform: Transform);

    /// Push a `fill` instruction into graphics stack.
    fn push_fill(&mut self, color: Rgba);

    /// Push a `stroke` instruction into graphics stack.
    fn push_stroke(&mut self, color: Rgba, width: Rgba);

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
}

#[cfg(test)]
#[allow(unused)]
pub(crate) mod mock {
    use super::Renderer;

    pub(crate) struct NoopRenders;

    impl Renderer for NoopRenders {
        fn pop(&mut self, n: usize) {}

        fn push_transform(&mut self, transform: crate::Transform) {}

        fn push_fill(&mut self, color: crate::Rgba) {}

        fn push_stroke(&mut self, color: crate::Rgba, width: crate::Rgba) {}

        fn line(&mut self, from: crate::Point, to: crate::Point) {}

        fn quadratic_bezier(&mut self, from: crate::Point, ctrl: crate::Point, to: crate::Point) {}

        fn cubic_bezier(
            &mut self,
            from: crate::Point,
            ctrl1: crate::Point,
            ctrl2: crate::Point,
            to: crate::Point,
        ) {
        }

        fn arc(
            &mut self,
            center: crate::Point,
            raddii: (crate::Length, crate::Length),
            start_angle: crate::Angle,
            sweep_angle: crate::Angle,
            x_rotation: crate::Angle,
        ) {
        }
    }
}
