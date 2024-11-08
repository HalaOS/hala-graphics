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

#[cfg(test)]
mod tests {

    use crate::{Length, NoopRenderer, Renderer};

    use super::*;
    #[test]
    fn test_combinator() {
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
                            (
                                circle((20.0, 20.0), 10.0),
                                circle((20.0, 20.0), Length::pc(10.0)),
                                |renderer: &mut NoopRenderer| {
                                    renderer.line((0.0, 0.0).into(), (5.0, 5.0).into());
                                    Ok(())
                                },
                            ),
                        ),
                    ),
                ),
            ),
        )(&mut NoopRenderer)
        .unwrap();
    }
}
