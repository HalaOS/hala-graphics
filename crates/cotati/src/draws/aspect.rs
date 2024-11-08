use crate::{PreserveAspectRatio, Renderer};

use super::Draw;

/// Apply `PreserveAspectRatio` to viewbox.
pub fn aspect<R, D, E>(ratio: PreserveAspectRatio, child: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Draw<R, Error = E>,
{
    move |renderer| {
        renderer.push_preserve_aspect_ratio(ratio);

        child.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
