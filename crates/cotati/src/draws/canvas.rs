use crate::{Length, Renderer};

use super::Draw;

/// Create a new `canvas`.
pub fn canvas<R, D, E>(width: Length, height: Length, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_canvas(width, height);

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
