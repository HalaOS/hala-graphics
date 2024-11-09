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

/// In a path move current point to `to` point.
pub fn move_to<T, R>(to: T) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<T>,
    R: Renderer,
{
    let to = to.into();

    move |renderer| {
        renderer.line(None, to);
        Ok(())
    }
}

/// Create a line
pub fn line<F, T, R>(from: F, to: T) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<F> + From<T>,
    R: Renderer,
{
    let from = from.into();
    let to = to.into();

    move |renderer| {
        renderer.line(Some(from), to);
        Ok(())
    }
}

/// Create a line segment of one path.
pub fn line_to<F, T, R>(from: F, to: T) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<F> + From<T>,
    R: Renderer,
{
    let from = from.into();
    let to = to.into();

    move |renderer| {
        renderer.line(Some(from), to);
        Ok(())
    }
}

/// Create a quadratic bezier curve.
pub fn quadratic_bezier<F, C, T, R>(from: F, ctrl: C, to: T) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<F> + From<C> + From<T>,
    R: Renderer,
{
    let from = from.into();
    let ctrl = ctrl.into();
    let to = to.into();

    move |renderer| {
        renderer.quadratic_bezier(Some(from), ctrl, to);
        Ok(())
    }
}

/// Create a quadratic bezier curve segment of one path.
pub fn quadratic_bezier_to<F, C, T, R>(ctrl: C, to: T) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<F> + From<C> + From<T>,
    R: Renderer,
{
    let ctrl = ctrl.into();
    let to = to.into();

    move |renderer| {
        renderer.quadratic_bezier(None, ctrl, to);
        Ok(())
    }
}

/// Create a cubic bezier curve.
pub fn cubic_bezier<F, C1, C2, T, R>(
    from: F,
    ctrl1: C1,
    ctrl2: C2,
    to: T,
) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<F> + From<C1> + From<C2> + From<T>,
    R: Renderer,
{
    let from = from.into();
    let ctrl1 = ctrl1.into();
    let ctrl2 = ctrl2.into();
    let to = to.into();

    move |renderer| {
        renderer.cubic_bezier(Some(from), ctrl1, ctrl2, to);
        Ok(())
    }
}

/// Create a quadratic bezier curve segment of one path.
pub fn cubic_bezier_to<C1, C2, T, R>(
    ctrl1: C1,
    ctrl2: C2,
    to: T,
) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<C1> + From<C2> + From<T>,
    R: Renderer,
{
    let ctrl1 = ctrl1.into();
    let ctrl2 = ctrl2.into();
    let to = to.into();

    move |renderer| {
        renderer.cubic_bezier(None, ctrl1, ctrl2, to);
        Ok(())
    }
}

/// Create a elliptic arc curve.
pub fn arc<C, RX, RY, START, SWEEP, X, R>(
    center: C,
    rx: RX,
    ry: RY,
    start_angle: START,
    sweep_angle: SWEEP,
    x_rotation: X,
) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<C>,
    Length: From<RX> + From<RY>,
    Angle: From<START> + From<SWEEP> + From<X>,
    R: Renderer,
{
    let center = center.into();
    let raddii = (rx.into(), ry.into());
    let start_angle = start_angle.into();
    let sweep_angle = sweep_angle.into();
    let x_rotation = x_rotation.into();

    move |renderer| {
        renderer.arc(Some(center), raddii, start_angle, sweep_angle, x_rotation);
        Ok(())
    }
}

/// Create a elliptic arc curve segment of one path.
pub fn arc_to<C, RX, RY, START, SWEEP, X, R>(
    rx: RX,
    ry: RY,
    start_angle: START,
    sweep_angle: SWEEP,
    x_rotation: X,
) -> impl Fn(&mut R) -> Result<(), ()>
where
    Point: From<C>,
    Length: From<RX> + From<RY>,
    Angle: From<START> + From<SWEEP> + From<X>,
    R: Renderer,
{
    let raddii = (rx.into(), ry.into());
    let start_angle = start_angle.into();
    let sweep_angle = sweep_angle.into();
    let x_rotation = x_rotation.into();

    move |renderer| {
        renderer.arc(None, raddii, start_angle, sweep_angle, x_rotation);
        Ok(())
    }
}
