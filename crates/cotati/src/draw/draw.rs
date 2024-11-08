use crate::Renderer;

/// A draw element must implement this trait.
pub trait Draw<R>
where
    R: Renderer,
{
    /// Error returns by draw function.
    type Error;

    /// Render draw element to target.
    fn render(&mut self, renderer: &mut R) -> Result<(), Self::Error>;
}

/// All `FnMut(&mut R) -> Result<(), E>` function are valid [`Draw`] elements.
impl<F, R, E> Draw<R> for F
where
    R: Renderer,
    F: FnMut(&mut R) -> Result<(), E>,
{
    type Error = E;

    fn render(&mut self, renderer: &mut R) -> Result<(), Self::Error> {
        self(renderer)
    }
}
