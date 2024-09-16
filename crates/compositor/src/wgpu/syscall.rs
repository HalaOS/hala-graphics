use async_trait::async_trait;
use wgpu::{CommandBuffer, CommandEncoder, Device, RenderPipeline, TextureView};

use crate::{syscall::DriverCanvas, Geometry, Result};

use super::WgpuRendering;

#[async_trait]
/// Trait used by compositor layer.
pub(super) trait DriverWgpuLayer: DriverCanvas {
    /// Returns `None`, if this layer is closed.
    fn render(
        &self,
        render: &WgpuRendering,
        device: &Device,
        render_pipeline: &RenderPipeline,
        width: u32,
        height: u32,
    ) -> Result<Option<CommandBuffer>>;

    fn sync(&self, device: &Device);
}

pub trait DriverWgpuRendering: Sync + Send {
    fn create_piple_line(&self, device: &Device) -> RenderPipeline;

    fn render_pass(
        &self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        texture_view: &TextureView,
        command_encoder: &mut CommandEncoder,
        geometry: Geometry,
    );
}
