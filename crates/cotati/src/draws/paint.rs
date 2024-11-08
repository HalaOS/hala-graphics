use crate::{Length, Renderer, Rgba};

use super::Draw;

/// Apply fill paint style to `draw` element.
pub fn fill<R, D, E>(color: Rgba, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_fill(color);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}

/// Apply fill paint style to `draw` element.
pub fn stroke<R, D, E>(color: Rgba, width: Length, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_stroke(color, width);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::{circle, NoopRenderer};

    use super::*;
    #[test]
    fn test_fill() {
        fill(
            "#f00".parse().unwrap(),
            (
                circle((20.0, 20.0).into(), 10.0.into()),
                circle((20.0, 20.0).into(), 10.0.into()),
            ),
        )(&mut NoopRenderer)
        .unwrap();
    }
}
