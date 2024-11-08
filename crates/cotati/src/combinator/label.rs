use crate::{View, Renderer};

/// Attach a debug `label` to a draw element.
pub fn label<'a, R, D, E>(name: &'a str, child: D) -> impl Fn(&mut R) -> Result<(), E> + 'a
where
    D: View<R, Error = E> + 'a,
    R: Renderer,
{
    // let name = name.to_owned();

    move |renderer| {
        renderer.push_label(name);
        child.render(renderer)?;
        renderer.pop(1);

        Ok(())
    }
}
