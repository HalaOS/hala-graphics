use crate::{Drawing, Length, Renderer};

/// Create a new `canvas`.
pub fn canvas<R, D, W, H, E>(width: W, height: H, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    Length: From<W> + From<H>,
    R: Renderer,
    D: Drawing<R, Error = E>,
{
    let width = width.into();
    let height = height.into();

    move |renderer| {
        renderer.push_canvas(width, height);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
