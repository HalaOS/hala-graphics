use crate::{Angle, Canvas, Length, Point, Rgba, Transform};

use super::Renderer;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum MockDirection {
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
        from: Option<crate::Point>,
        to: Point,
    },

    QuadraticBezier {
        from: Option<crate::Point>,
        ctrl: Point,
        to: Point,
    },

    CubicBezier {
        from: Option<crate::Point>,
        ctrl1: Point,
        ctrl2: Point,
        to: Point,
    },

    Arc {
        center: Option<crate::Point>,
        raddii: (Length, Length),
        start_angle: Angle,
        sweep_angle: Angle,
        x_rotation: Angle,
    },

    Canvas(Canvas),

    Label(String),

    Submit,

    Path,

    MoveTo(Point),
}

#[derive(Default)]
pub struct MockRenderer(Vec<MockDirection>);

impl MockRenderer {
    pub fn instructions(&self) -> &[MockDirection] {
        &self.0
    }
}

impl Renderer for MockRenderer {
    type Error = ();

    fn clear(&mut self) {
        self.0.push(MockDirection::Clear);
    }

    fn pop(&mut self, n: usize) {
        self.0.push(MockDirection::Pop(n));
    }

    fn push_entity(&mut self, id: &str) {
        self.0.push(MockDirection::Entity(id.to_string()));
    }

    fn entity_ref(&mut self, id: &str) {
        self.0.push(MockDirection::EntityReference(id.to_string()));
    }

    fn push_transform(&mut self, transform: crate::Transform) {
        self.0.push(MockDirection::Transform(transform));
    }

    fn push_fill(&mut self, color: crate::Rgba) {
        self.0.push(MockDirection::Fill(color));
    }

    fn push_stroke(&mut self, color: crate::Rgba, width: crate::Length) {
        self.0.push(MockDirection::Stroke { color, width });
    }

    fn line(&mut self, from: Option<crate::Point>, to: crate::Point) {
        self.0.push(MockDirection::Line { from, to });
    }

    fn quadratic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl: crate::Point,
        to: crate::Point,
    ) {
        self.0
            .push(MockDirection::QuadraticBezier { from, ctrl, to });
    }

    fn cubic_bezier(
        &mut self,
        from: Option<crate::Point>,
        ctrl1: crate::Point,
        ctrl2: crate::Point,
        to: crate::Point,
    ) {
        self.0.push(MockDirection::CubicBezier {
            from,
            ctrl1,
            ctrl2,
            to,
        });
    }

    fn arc(
        &mut self,
        center: Option<crate::Point>,
        raddii: (crate::Length, crate::Length),
        start_angle: crate::Angle,
        sweep_angle: crate::Angle,
        x_rotation: crate::Angle,
    ) {
        self.0.push(MockDirection::Arc {
            center,
            raddii,
            start_angle,
            sweep_angle,
            x_rotation,
        });
    }

    fn push_canvas(&mut self, canvas: Canvas) {
        self.0.push(MockDirection::Canvas(canvas));
    }

    fn push_label(&mut self, label: &str) {
        self.0.push(MockDirection::Label(label.to_owned()));
    }

    fn submit(&mut self) -> Result<(), Self::Error> {
        self.0.push(MockDirection::Submit);

        Ok(())
    }

    fn push_path(&mut self) {
        self.0.push(MockDirection::Path);
    }

    fn move_to(&mut self, to: Point) {
        self.0.push(MockDirection::MoveTo(to));
    }
}
