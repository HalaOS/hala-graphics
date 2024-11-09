use crate::{Angle, Length, Point, Renderer};

/// Create a circle
pub fn circle<R, P, L>(center: P, radius: L) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<P>,
    Length: From<L>,
    R: Renderer,
{
    let center = center.into();
    let radius = radius.into();

    move |render| {
        render.arc(
            Some(center),
            (radius, radius),
            Angle::zero(),
            Angle::two_pi(),
            Angle::zero(),
        );

        Ok(())
    }
}
