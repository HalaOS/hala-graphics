use crate::{Canvas, Drawing, Length, PreserveAspectRatio, Renderer, ViewBox};

/// A builder returns by [`viewbox`] function.
pub struct ViewBoxBuilder(Canvas);

impl Into<Canvas> for ViewBoxBuilder {
    fn into(self) -> Canvas {
        self.0
    }
}

/// Create a new viewbox,
pub fn viewbox<W, H, C>(config: C, width: W, height: H) -> ViewBoxBuilder
where
    C: Into<Canvas>,
    Length: From<W> + From<H>,
{
    let mut canvas: Canvas = config.into();

    canvas.viewbox = Some(ViewBox {
        width: width.into(),
        height: height.into(),
        aspect: None,
    });

    ViewBoxBuilder(canvas)
}

/// Apply aspect to viewbox.
pub fn aspect(viewbox: ViewBoxBuilder, value: PreserveAspectRatio) -> Canvas {
    let mut canvas = viewbox.0;

    canvas.viewbox.as_mut().unwrap().aspect = Some(value);

    canvas
}

/// Create a new `canvas`.
pub fn canvas<R, C, D, E>(config: C, draw: D) -> impl Fn(&mut R) -> Result<(), E>
where
    R: Renderer,
    D: Drawing<R, Error = E>,
    C: Into<Canvas>,
{
    let config = config.into();

    move |renderer| {
        renderer.push_canvas(config.clone());

        draw.render(renderer)?;

        renderer.pop(1);

        Ok(())
    }
}
