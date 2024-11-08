use crate::{Renderer, Rgba};

use super::Draw;

/// Apply fill paint style to `draw` element.
pub fn fill<R, D, E>(color: Rgba, mut draw: D) -> impl FnMut(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_fill(color);

        let r = draw.render(renderer);

        renderer.pop(1);

        r
    }
}

#[cfg(test)]
mod tests {

    use crate::{circle, mock::NoopRenders, Length, Point};

    use super::*;
    #[test]
    fn test_fill() {
        fill(
            "#f00".parse().unwrap(),
            (
                circle(Point::px(20.0, 20.0), Length::px(10.0)),
                circle(Point::px(20.0, 20.0), Length::px(20.0)),
            ),
        )(&mut NoopRenders)
        .unwrap();
    }
}
