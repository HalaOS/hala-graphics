use std::sync::Arc;

use async_trait::async_trait;
use futures::future::poll_fn;
use spin::mutex::Mutex;
use uuid::Uuid;
use wgpu::{
    CommandBuffer, CommandEncoder, Device, RenderPipeline, Texture, TextureView,
    TextureViewDescriptor, TextureViewDimension,
};

use crate::{syscall::DriverCanvas, Error, Geometry, LayerId, Rect, Result, Vertex};

use super::{capture::WgpuCapture, create_layer_texture, syscall::DriverWgpuLayer, WgpuRendering};

#[derive(Default)]
struct MutableWgpuCanvas {
    is_closed: bool,
    resize: Option<Rect>,
    geometry: Option<Geometry>,
    texture: Option<Texture>,
}

impl MutableWgpuCanvas {
    fn texture_view(&mut self, device: &Device, width: u32, height: u32) -> TextureView {
        let desc = TextureViewDescriptor {
            dimension: Some(TextureViewDimension::D2),
            ..Default::default()
        };

        if let Some(texture) = self.texture.as_ref() {
            if texture.width() == width && texture.height() == height {
                return texture.create_view(&desc);
            }
        }

        log::trace!("create canvas texture({},{})", width, height);

        let texture = create_layer_texture(device, width, height);

        let texture_view = texture.create_view(&desc);

        self.texture = Some(texture);

        texture_view
    }
}

#[derive(Clone, Default)]
pub(super) struct WgpuCanvas {
    id: LayerId,
    mutable: Arc<Mutex<MutableWgpuCanvas>>,
    capture: WgpuCapture,
}

impl Drop for WgpuCanvas {
    fn drop(&mut self) {
        self.mutable.lock().is_closed = true;
    }
}

impl WgpuCanvas {
    pub fn new(resize: Option<Rect>) -> Self {
        Self {
            id: Uuid::new_v4(),
            mutable: Arc::new(Mutex::new(MutableWgpuCanvas {
                resize,
                ..Default::default()
            })),
            capture: Default::default(),
        }
    }
    fn redraw(
        &self,
        render: &WgpuRendering,
        device: &Device,
        render_pipeline: &RenderPipeline,
        command_encoder: &mut CommandEncoder,
        width: u32,
        height: u32,
    ) -> Result<TextureView> {
        let (texture_view, geometry) = {
            let mut mutable = self.mutable.lock();

            if mutable.is_closed {
                return Err(Error::Done);
            }

            (
                mutable.texture_view(device, width, height),
                mutable.geometry.take(),
            )
        };

        if let Some(geometry) = geometry {
            render.render_pass(
                device,
                render_pipeline,
                &texture_view,
                command_encoder,
                geometry,
            );
        }

        Ok(texture_view)
    }

    fn capture(
        &self,
        device: &Device,
        command_encoder: &mut CommandEncoder,
        width: u32,
        height: u32,
    ) {
        let mutable = self.mutable.lock();

        self.capture.capture(
            device,
            command_encoder,
            mutable.texture.as_ref().expect("Call redraw first"),
            width,
            height,
        );
    }
}

#[async_trait]
impl DriverCanvas for WgpuCanvas {
    /// Returns the layer id.
    fn id(&self) -> &LayerId {
        &self.id
    }
    /// Move this canvas's position and size.
    async fn layer_move(&self, rect: Rect) -> Result<()> {
        self.mutable.lock().resize = Some(rect);

        Ok(())
    }

    /// Update rendering data.
    async fn update(&self, vertices: Vec<Vertex>, indeces: Vec<u32>) -> Result<()> {
        self.mutable.lock().geometry = Some(Geometry { vertices, indeces });

        Ok(())
    }

    async fn capture(&self) -> Result<Vec<u8>> {
        poll_fn(|cx| self.capture.poll(cx)).await
    }
}

impl DriverWgpuLayer for WgpuCanvas {
    fn render(
        &self,
        render: &WgpuRendering,
        device: &Device,
        render_pipeline: &RenderPipeline,
        width: u32,
        height: u32,
    ) -> Result<Option<CommandBuffer>> {
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("CanvasLayer"),
        });

        self.redraw(
            render,
            device,
            render_pipeline,
            &mut command_encoder,
            width,
            height,
        )?;

        self.capture(device, &mut command_encoder, width, height);

        Ok(Some(command_encoder.finish()))
    }

    fn sync(&self, _device: &Device) {
        self.capture.sync();
    }
}
