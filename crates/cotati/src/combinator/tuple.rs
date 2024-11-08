use crate::{Renderer, View};

macro_rules! tuple_views {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+ , R, E> View<R> for ($header, $($tail),+)
        where
             R: Renderer,
            $header: View<R, Error=E>,
            $($tail: View<R, Error=E>),+,
        {
            type Error = E;

            #[allow(non_snake_case)]
            fn render(&self, render: &mut R) -> Result<(), Self::Error> {
                    let ($header, $($tail),+) = self;

                    $header.render(render)?;

                    $($tail.render(render)?;)+

                    Ok(())
            }
        }

        tuple_views!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_views!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20
);

#[cfg(test)]
mod tests {
    use crate::Renderer;

    use super::View;

    use crate::MockRenderer;

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
        (mock_1, mock_2)
            .render(&mut MockRenderer::default())
            .unwrap();
    }
}
