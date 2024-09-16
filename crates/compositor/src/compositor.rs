use uuid::Uuid;

use crate::macros::driver_wrapper;

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

/// The compositor layer id.
pub type LayerId = Uuid;

/// Geometry data used by canvas layer.
#[derive(Clone, Default)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub indeces: Vec<u32>,
}

/// A compositor implementation must implement traits in this mod.
pub mod syscall {
    use async_trait::async_trait;

    use crate::Result;

    use super::*;

    #[async_trait]
    pub trait DriverCompositor: Sync + Send {
        /// Clone self.
        fn clone(&self) -> Compositor;

        /// Adjust the compositor rendering size.
        async fn resize(&self, width: u32, height: u32) -> Result<()>;

        /// Returns the rendering size.
        async fn size(&self) -> Result<(u32, u32)>;

        /// Create a new canvas layer with initial position and size.
        async fn create_canvas(&self, resize: Option<Rect>) -> Result<Canvas>;

        /// Display compositing effects.
        async fn compositing(&self) -> Result<()>;
    }

    #[async_trait]
    pub trait DriverCanvas: Sync + Send {
        /// Returns the layer id.
        fn id(&self) -> &LayerId;
        /// Move this canvas's position and size.
        async fn layer_move(&self, rect: Rect) -> Result<()>;

        /// Update rendering data.
        async fn update(&self, vertices: Vec<Vertex>, indeces: Vec<u32>) -> Result<()>;

        /// Capture layer content with bitmap.
        async fn capture(&self) -> Result<Vec<u8>>;
    }
}

driver_wrapper!(
    ["A type wrapper of [`DriverCompositor`](syscall::DriverCompositor)"]
    Compositor[syscall::DriverCompositor]
);

driver_wrapper!(
    ["A type wrapper of [`DriverCanvas`](syscall::DriverCanvas)"]
    Canvas[syscall::DriverCanvas]
);
