use ecsrs::{ecs_system, Id};
use wgpu::Texture;
use wgpu_derive::Vertex;

use crate::{Transform2D, Viewport};

/// A component that contains redraw entity list.
pub struct RedrawComponent(pub Vec<Id>);

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

#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SvgVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

/// A rendering element that can display a svg image.
pub struct SvgComponent {
    /// The transform matrix for this canvas.
    pub transform: Transform2D,
    /// The rendering vertex buffer.
    pub vertexes: Vec<Canvas2DVertex>,
    /// The rendering vertex index buffer.
    pub indecs: Vec<u32>,
}

/// The component that support capture rendering element.
pub struct CaptureComponent {
    /// The rendering elements entity id that need be captured.
    pub next_frame: Option<Vec<Id>>,
}

/// A rendering layer.
pub struct LayerComponent {
    /// The layer's view port.
    pub viewport: Viewport,
    /// The transform matrix for this layer used by compositing process.
    pub transform: Transform2D,
    /// The GPU render attached texture for this layer.
    pub texture: Texture,
}

// Attention!!! when add new component, don't change this order.
// Attention!!! never to remove deprecated component from this list.
ecs_system!(
    LayerComponent,
    RedrawComponent,
    Canvas2DComponent,
    SvgComponent,
    CaptureComponent
);
