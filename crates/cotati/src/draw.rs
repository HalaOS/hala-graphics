use crate::Renderer;

/// A draw element must implement this trait.
pub trait Draw<R>
where
    R: Renderer,
{
    /// Error returns by draw function.
    type Error;

    /// Render draw element to target.
    fn render(&mut self, render: &mut R) -> Result<(), Self::Error>;
}

/// All `FnMut(&mut R) -> Result<(), E>` function are valid [`Draw`] elements.
impl<F, R, E> Draw<R> for F
where
    R: Renderer,
    F: FnMut(&mut R) -> Result<(), E>,
{
    type Error = E;

    fn render(&mut self, render: &mut R) -> Result<(), Self::Error> {
        self(render)
    }
}

struct PairDraw<D1, D2> {
    left: D1,
    right: D2,
}

impl<D1, D2, R, E1, E2> Draw<R> for PairDraw<D1, D2>
where
    R: Renderer,
    D1: Draw<R, Error = E1>,
    D2: Draw<R, Error = E2>,
    E2: From<E1>,
{
    type Error = E2;

    fn render(&mut self, render: &mut R) -> Result<(), Self::Error> {
        self.left.render(render)?;
        self.right.render(render)
    }
}

macro_rules! tuple_draw {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+ , R, E> Draw<R> for ($header, $($tail),+)
        where
             R: Renderer,
            $header: Draw<R, Error=E>,
            $($tail: Draw<R, Error=E>),+,
        {
            type Error = E;

            #[allow(non_snake_case)]
            fn render(&mut self, render: &mut R) -> Result<(), Self::Error> {
                    let ($header, $($tail),+) = self;

                    $header.render(render)?;

                    $($tail.render(render)?;)+

                    Ok(())
            }
        }

        tuple_draw!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_draw!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20
);

#[cfg(test)]
mod tests {
    use crate::Renderer;

    use super::Draw;

    use crate::mock::NoopRenders;

    fn mock_1<R>(_: &mut R) -> Result<(), ()>
    where
        R: Renderer,
    {
        Ok(())
    }

    fn mock_2<R>(_: &mut R) -> Result<(), ()>
    where
        R: Renderer,
    {
        Ok(())
    }

    #[test]
    fn test_tuple() {
        (mock_1, mock_2).render(&mut NoopRenders).unwrap();
    }
}
