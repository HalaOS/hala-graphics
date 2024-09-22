use std::{borrow::Cow, fmt::Display, fs, ops::Deref, path::Path, str::FromStr};

use regex::Regex;

use crate::{Error, Result};

/// Transform matrix for 2d element.
pub type Transform2D = euclid::default::Transform2D<f32>;

/// Transform matrix for 3d element.
pub type Transform3D = euclid::default::Transform3D<f32>;

/// Unit pixels.
pub struct Pixels;

/// Viewport dimensions
#[derive(Debug, Clone, Copy)]
pub struct Viewport(euclid::Size2D<u32, Pixels>);

impl Deref for Viewport {
    type Target = euclid::Size2D<u32, Pixels>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Viewport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "viewport({},{})", self.width, self.height)
    }
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Self(euclid::Size2D::new(width, height))
    }
}

impl FromStr for Viewport {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let regex = Regex::new(r"\((?<width>\d{1,4}),(?<height>\d{1,4})\)").unwrap();

        let captures = regex
            .captures(s)
            .ok_or(Error::InvalidViewPortStr(s.to_string()))?;

        let width: u32 = captures["width"]
            .parse()
            .map_err(|_| Error::InvalidViewPortStr(s.to_string()))?;

        let height: u32 = captures["height"]
            .parse()
            .map_err(|_| Error::InvalidViewPortStr(s.to_string()))?;

        Ok(Viewport::new(width, height))
    }
}

/// Calculates the size of the buffer used to hold the render target with `viewport` dimensions.
pub trait BufferSizeOf {
    fn buffer_size_of(&self) -> u64;
}

impl BufferSizeOf for Viewport {
    fn buffer_size_of(&self) -> u64 {
        (size_of::<u32>() as u32 * self.width * self.height) as wgpu::BufferAddress
    }
}

/// a png image.
pub enum Png<'a> {
    Data(Cow<'a, [u8]>),

    Path(&'a Path),
}

impl<'a> Png<'a> {
    /// Save png to file.
    pub fn save<P: AsRef<Path>>(&self, to: P) -> Result<()> {
        match self {
            Png::Data(cow) => {
                fs::write(to, cow)?;
            }
            Png::Path(src) => _ = fs::copy(src, to)?,
        }

        Ok(())
    }

    /// Load a png into memory.
    pub fn load(self) -> Result<Png<'static>> {
        match self {
            Png::Path(src) => Ok(Png::Data(fs::read(src)?.into())),
            Png::Data(data) => Ok(Png::Data(data.into_owned().into())),
        }
    }
}
