use crate::{Draw, Length, Renderer};

/// Apply `viewbox` to canvas.
pub fn viewbox<R, D, W, H, E>(width: W, height: H, child: D) -> impl Fn(&mut R) -> Result<(), E>
where
    Length: From<W> + From<H>,
    R: Renderer,
    D: Draw<R, Error = E>,
{
    let width = width.into();
    let height = height.into();

    move |renderer| {
        renderer.push_viewbox(width, height);

        child.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
