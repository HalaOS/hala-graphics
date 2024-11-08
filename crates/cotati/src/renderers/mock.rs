use crate::{Angle, Length, Point, PreserveAspectRatio, Rgba, Transform};

use super::Renderer;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MockInstruction {
    Clear,
    Pop(usize),
    Entity(String),
    EntityReference(String),
    Transform(Transform),
    Fill(Rgba),
    Stroke {
        color: Rgba,
        width: Length,
    },

    Line {
        from: Point,
        to: Point,
    },

    QuadraticBezier {
        from: Point,
        ctrl: Point,
        to: Point,
    },

    CubicBezier {
        from: Point,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
    },

    Arc {
        center: Point,
        raddii: (Length, Length),
        start_angle: Angle,
        sweep_angle: Angle,
        x_rotation: Angle,
    },

    Canvas {
        width: Length,
        height: Length,
    },

    ViewBox {
        width: Length,
        height: Length,
    },
    Aspect(PreserveAspectRatio),

    Label(String),

    Submit,
}

#[derive(Default)]
pub struct MockRenderer(Vec<MockInstruction>);

impl MockRenderer {
    pub fn instructions(&self) -> &[MockInstruction] {
        &self.0
    }
}

impl Renderer for MockRenderer {
    type Error = ();

    fn clear(&mut self) {
        self.0.push(MockInstruction::Clear);
    }

    fn pop(&mut self, n: usize) {
        self.0.push(MockInstruction::Pop(n));
    }

    fn push_entity(&mut self, id: &str) {
        self.0.push(MockInstruction::Entity(id.to_string()));
    }

    fn entity_ref(&mut self, id: &str) {
        self.0
            .push(MockInstruction::EntityReference(id.to_string()));
    }

    fn push_transform(&mut self, transform: crate::Transform) {
        self.0.push(MockInstruction::Transform(transform));
    }

    fn push_fill(&mut self, color: crate::Rgba) {
        self.0.push(MockInstruction::Fill(color));
    }

    fn push_stroke(&mut self, color: crate::Rgba, width: crate::Length) {
        self.0.push(MockInstruction::Stroke { color, width });
    }

    fn line(&mut self, from: crate::Point, to: crate::Point) {
        self.0.push(MockInstruction::Line { from, to });
    }

    fn quadratic_bezier(&mut self, from: crate::Point, ctrl: crate::Point, to: crate::Point) {
        self.0
            .push(MockInstruction::QuadraticBezier { from, ctrl, to });
    }

    fn cubic_bezier(
        &mut self,
        from: crate::Point,
        ctrl1: crate::Point,
        ctrl2: crate::Point,
        to: crate::Point,
    ) {
        self.0.push(MockInstruction::CubicBezier {
            from,
            ctrl1,
            ctrl2,
            to,
        });
    }

    fn arc(
        &mut self,
        center: crate::Point,
        raddii: (crate::Length, crate::Length),
        start_angle: crate::Angle,
        sweep_angle: crate::Angle,
        x_rotation: crate::Angle,
    ) {
        self.0.push(MockInstruction::Arc {
            center,
            raddii,
            start_angle,
            sweep_angle,
            x_rotation,
        });
    }

    fn push_canvas(&mut self, width: crate::Length, height: crate::Length) {
        self.0.push(MockInstruction::Canvas { width, height });
    }

    fn push_viewbox(&mut self, width: crate::Length, height: crate::Length) {
        self.0.push(MockInstruction::ViewBox { width, height });
    }

    fn push_preserve_aspect_ratio(&mut self, ratio: crate::PreserveAspectRatio) {
        self.0.push(MockInstruction::Aspect(ratio));
    }

    fn push_label(&mut self, label: &str) {
        self.0.push(MockInstruction::Label(label.to_owned()));
    }

    fn submit(&mut self) -> Result<(), Self::Error> {
        self.0.push(MockInstruction::Submit);

        Ok(())
    }
}
