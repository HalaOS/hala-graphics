use crate::{Length, Renderer};

use super::Draw;

/// Apply `viewbox` to canvas.
pub fn viewbox<R, D, E>(width: Length, height: Length, child: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_viewbox(width, height);

        child.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
