use wgpu_derive::Vertex;

use crate::Transform2D;

#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Canvas2DVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

/// A 2d canvas rendering element.
pub struct Canvas2DComponent {
    /// The transform matrix for this canvas.
    pub transform: Transform2D,
    /// The rendering vertex buffer.
    pub vertexes: Vec<Canvas2DVertex>,
    /// The rendering vertex index buffer.
    pub indecs: Vec<u32>,
}
