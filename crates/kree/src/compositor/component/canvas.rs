use ecsrs::Id;
use wgpu_derive::Vertex;

#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Canvas2DVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

/// A 2d canvas rendering element.
pub struct Canvas2DComponent {
    pub id: Id,
}
