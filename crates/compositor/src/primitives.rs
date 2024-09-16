use uuid::Uuid;

/// A 2d Rectangle
pub type Rect = lyon::geom::euclid::Rect<f32, f32>;

/// A 2d Point tagged
pub type Point = lyon::geom::euclid::Point2D<f32, f32>;

/// A 2d size
pub type Size = lyon::geom::euclid::Size2D<f32, f32>;

/// Vertex object for copying data from the vertex buffer or copying data to the vertex buffer.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    /// Get format object of this type.
    #[cfg(feature = "wgpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wgpu")))]
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        const ATTRIBS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

/// The id used by compositor layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub Uuid);

impl Default for Id {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

/// Geometry data used by compositor rendering.
#[derive(Clone, Default)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indeces: Vec<u32>,
}
