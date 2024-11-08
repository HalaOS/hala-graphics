use crate::{Draw, Length, Renderer, Rgba};

/// Apply fill paint style to `draw` element.
pub fn fill<R, D, C, E>(color: C, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    Rgba: From<C>,
    R: Renderer,
    D: Draw<R, Error = E>,
{
    let color = color.into();

    move |renderer| {
        renderer.push_fill(color);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}

/// Apply fill paint style to `draw` element.
pub fn stroke<R, D, C, L, E>(color: C, width: L, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    Length: From<L>,
    Rgba: From<C>,
    R: Renderer,
    D: Draw<R, Error = E>,
{
    let color = color.into();
    let width = width.into();

    move |renderer| {
        renderer.push_stroke(color, width);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
