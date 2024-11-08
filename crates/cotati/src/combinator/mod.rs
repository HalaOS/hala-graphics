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

    use crate::{Length, NoopRenderer, PreserveAspectRatio};

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
                    PreserveAspectRatio::default(),
                    fill(
                        (255, 0, 255),
                        stroke(
                            (255, 0, 255),
                            1.0,
                            (
                                circle((20.0, 20.0), 10.0),
                                circle((20.0, 20.0), Length::pc(10.0)),
                            ),
                        ),
                    ),
                ),
            ),
        )(&mut NoopRenderer)
        .unwrap();
    }
}
