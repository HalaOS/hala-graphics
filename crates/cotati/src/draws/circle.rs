use crate::{Angle, Length, Point, Renderer};

/// Create a circle
pub fn circle<R>(center: Point, radius: Length) -> impl Fn(&mut R) -> Result<(), ()>
where
    R: Renderer,
{
    move |render| {
        render.arc(
            center,
            (radius, radius),
            Angle::zero(),
            Angle::two_pi(),
            Angle::zero(),
        );

        Ok(())
    }
}
