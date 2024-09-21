use std::{fs, path::Path};

use ecsrs::Id;
use wgpu_derive::Vertex;

use crate::Result;

#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SvgVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

/// A rendering element that can display a svg image.
pub struct SvgComponent {
    pub(crate) id: Id,
    pub(crate) tessellated: SvgTessellated,
}

/// A gpu tessellator for svg image.
#[derive(Default, Clone)]
#[allow(unused)]
pub struct SvgTessellated {
    /// The rendering vertex buffer.
    pub(crate) vertexes: Vec<SvgVertex>,
    /// The rendering vertex index buffer.
    pub(crate) indecs: Vec<u32>,
}

impl SvgTessellated {
    /// Load a svg file and tessellate it.
    pub fn tessellate_with<P: AsRef<Path>>(path: P) -> Result<Self> {
        Self::tessellate(fs::read(path)?)
    }

    pub fn tessellate(_buf: Vec<u8>) -> Result<Self> {
        Ok(Self {
            ..Default::default()
        })
    }
}
