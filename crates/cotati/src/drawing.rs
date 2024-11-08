use crate::Renderer;

/// A vector drawing element/commander must implement this trait.
pub trait Drawing<R>
where
    R: Renderer,
{
    /// Error returns by [`render`](View) function.
    type Error;

    /// Render element to target.
    fn render(&self, renderer: &mut R) -> Result<(), Self::Error>;
}

/// All `FnMut(& R) -> Result<(), E>` function are valid [`Draw`] elements.
impl<F, R, E> Drawing<R> for F
where
    R: Renderer,
    F: Fn(&mut R) -> Result<(), E>,
{
    type Error = E;

    fn render(&self, renderer: &mut R) -> Result<(), Self::Error> {
        self(renderer)
    }
}
