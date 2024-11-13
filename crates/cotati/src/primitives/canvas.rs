use super::{Length, PreserveAspectRatio};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct ViewBox {
    /// ViewBox left-top x coordinate,
    pub x: Length,
    /// ViewBox left-top y coordinate,
    pub y: Length,
    /// ViewBox width dimension.
    pub width: Length,
    /// ViewBox height dimension.
    pub height: Length,
    /// clip preserve aspect ratio.
    pub aspect: Option<PreserveAspectRatio>,
}

/// A canvas configuration.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Canvas {
    /// Canvas width dimension.
    pub width: Length,
    /// Canvas height dimension.
    pub height: Length,
    /// clip viewbox.
    pub viewbox: Option<ViewBox>,
}

impl<W, H> Into<Canvas> for (W, H)
where
    W: Into<Length>,
    H: Into<Length>,
{
    fn into(self) -> Canvas {
        Canvas {
            width: self.0.into(),
            height: self.1.into(),
            viewbox: None,
        }
    }
}
