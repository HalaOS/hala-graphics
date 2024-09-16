use async_trait::async_trait;
use wgpu::{CommandBuffer, Device, RenderPipeline};

use crate::{syscall::DriverCanvas, Result};

#[async_trait]
/// Trait used by compositor layer.
pub(super) trait DriverWgpuLayer: DriverCanvas {
    /// Returns `None`, if this layer is closed.
    fn render(
        &self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        width: u32,
        height: u32,
    ) -> Result<Option<CommandBuffer>>;

    fn sync(&self, device: &Device);
}
