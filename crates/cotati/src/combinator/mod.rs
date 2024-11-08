mod tuple;

mod circle;
pub use circle::*;
mod paint;
pub use paint::*;
mod aspect;
pub use aspect::*;
mod canvas;
pub use canvas::*;
mod viewbox;
pub use viewbox::*;
mod entity;
pub use entity::*;
mod label;
pub use label::*;

#[cfg(test)]
mod tests {

    use crate::{
        Angle, Length, MockInstruction, MockRenderer, Point, PreserveAspectRatio, Renderer, Rgba,
        Unit,
    };

    use super::*;
    #[test]
    fn test_combinator() {
        let mut renderer = MockRenderer::default();

        canvas(
            10.0,
            10.0,
            viewbox(
                10.0,
                10.0,
                aspect(
                    Default::default(),
                    fill(
                        (255, 0, 255),
                        stroke(
                            (255, 0, 255),
                            1.0,
                            label(
                                "content",
                                (
                                    circle((20.0, 20.0), 10.0),
                                    circle((20.0, 20.0), Length::pc(10.0)),
                                    |renderer: &mut MockRenderer| {
                                        renderer.line((0.0, 0.0).into(), (5.0, 5.0).into());
                                        Ok(())
                                    },
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )(&mut renderer)
        .unwrap();

        assert_eq!(
            renderer.instructions(),
            &[
                MockInstruction::Canvas {
                    width: Length(10.0, Unit::Px),
                    height: Length(10.0, Unit::Px)
                },
                MockInstruction::ViewBox {
                    width: Length(10.0, Unit::Px),
                    height: Length(10.0, Unit::Px)
                },
                MockInstruction::Aspect(PreserveAspectRatio::xMidYMid),
                MockInstruction::Fill(Rgba(1.0, 0.0, 1.0, 1.0)),
                MockInstruction::Stroke {
                    color: Rgba(1.0, 0.0, 1.0, 1.0),
                    width: Length(1.0, Unit::Px)
                },
                MockInstruction::Label("content".to_owned()),
                MockInstruction::Arc {
                    center: Point {
                        x: 20.0,
                        y: 20.0,
                        unit: Unit::Px
                    },
                    raddii: (Length(10.0, Unit::Px), Length(10.0, Unit::Px)),
                    start_angle: Angle::deg(0.0),
                    sweep_angle: Angle::deg(360.0),
                    x_rotation: Angle::deg(0.0)
                },
                MockInstruction::Arc {
                    center: Point {
                        x: 20.0,
                        y: 20.0,
                        unit: Unit::Px
                    },
                    raddii: (Length(10.0, Unit::Pc), Length(10.0, Unit::Pc)),
                    start_angle: Angle::deg(0.0),
                    sweep_angle: Angle::deg(360.0),
                    x_rotation: Angle::deg(0.0)
                },
                MockInstruction::Line {
                    from: Point {
                        x: 0.0,
                        y: 0.0,
                        unit: Unit::Px
                    },
                    to: Point {
                        x: 5.0,
                        y: 5.0,
                        unit: Unit::Px
                    }
                },
                MockInstruction::Pop(1),
                MockInstruction::Pop(1),
                MockInstruction::Pop(1),
                MockInstruction::Pop(1),
                MockInstruction::Pop(1),
                MockInstruction::Pop(1)
            ]
        );
    }
}
