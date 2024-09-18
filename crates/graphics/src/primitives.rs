use std::{borrow::Cow, fs, path::Path};

use crate::Result;

/// Transform matrix for 2d element.
pub type Transform2D = euclid::default::Transform2D<f32>;

/// Transform matrix for 3d element.
pub type Transform3D = euclid::default::Transform3D<f32>;

/// Unit pixels.
pub struct Pixels;

/// Viewport dimensions
pub type Viewport = euclid::Size2D<u32, Pixels>;

/// a png image.
pub enum Png<'a> {
    Data(Cow<'a, [u8]>),

    Path(&'a Path),
}

impl<'a> Png<'a> {
    /// Save png to file.
    pub async fn save<P: AsRef<Path>>(&self, to: P) -> Result<()> {
        match self {
            Png::Data(cow) => {
                fs::write(to, cow)?;
            }
            Png::Path(src) => _ = fs::copy(src, to)?,
        }

        Ok(())
    }
}
