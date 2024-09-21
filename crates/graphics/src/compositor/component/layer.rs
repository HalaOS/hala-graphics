use wgpu::Texture;

use crate::{Transform2D, Viewport};

/// A rendering layer.
pub struct LayerComponent {
    /// The layer's view port.
    pub viewport: Viewport,
    /// The transform matrix for this layer used by compositing process.
    pub transform: Transform2D,
    /// The GPU render attached texture for this layer.
    pub texture: Texture,
}
