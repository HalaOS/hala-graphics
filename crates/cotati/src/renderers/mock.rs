use super::Renderer;

pub(crate) struct NoopRenderer;

#[allow(unused)]
impl Renderer for NoopRenderer {
    type Error = ();

    fn clear(&mut self) {}

    fn pop(&mut self, n: usize) {}

    fn push_transform(&mut self, transform: crate::Transform) {}

    fn push_fill(&mut self, color: crate::Rgba) {}

    fn push_stroke(&mut self, color: crate::Rgba, width: crate::Length) {}

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

    fn push_canvas(&mut self, width: crate::Length, height: crate::Length) {}

    fn push_viewbox(&mut self, width: crate::Length, height: crate::Length) {}

    fn push_preserve_aspect_ratio(&mut self, ratio: crate::PreserveAspectRatio) {}

    fn submit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
