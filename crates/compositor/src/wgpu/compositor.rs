use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use spin::Mutex;
use wgpu::{Device, Queue, RenderPipeline, ShaderSource};

use crate::{syscall::DriverCompositor, Canvas, Compositor, Error, Rect, Result};

use super::{
    layers::WgpuCanvas, rendering::WgpCanvasRender, wgpu_syscall::DriverWgpuRenderer, WgpuLayer,
    WgpuRenderer,
};

/// Options of one wgpu [`Compositor`](crate::Compositor)
pub struct WgpuCompositorOps {
    /// Render pipeline for canvas layer.
    canvas_pipeline: RenderPipeline,
    /// Render for canvas layer.
    canvas_render: WgpuRenderer,
    /// Device associated with this `WgpuCompositor`
    device: Device,
    /// Queue associated with this `WgpuCompositor`
    queue: Queue,
}

/// The builder of [`WgpuCompositor`]
pub struct WgpuCompositorBuilder {
    canvas_render: WgpuRenderer,
}

impl Default for WgpuCompositorBuilder {
    fn default() -> Self {
        Self {
            canvas_render: WgpCanvasRender(ShaderSource::Wgsl(
                include_str!("./canvas.wgsl").into(),
            ))
            .into(),
        }
    }
}

impl WgpuCompositorBuilder {
    /// Override default canvas shader.
    pub fn canvas_shader_source(mut self, shader_source: ShaderSource<'static>) -> Self {
        self.canvas_render = WgpCanvasRender(shader_source).into();
        self
    }

    /// Override default canvas rendering.
    pub fn canvas_rendering<R>(mut self, rendering: R) -> Self
    where
        R: DriverWgpuRenderer + 'static,
    {
        self.canvas_render = rendering.into();
        self
    }

    /// Create a new [`WgpuCompositor`] with initial width and height in pixels.
    pub async fn create(self, width: u32, height: u32) -> Result<WgpuCompositor> {
        let (device, queue) = Self::init_wgpu().await?;

        let canvas_pipeline = self.canvas_render.create_piple_line(&device);

        Ok(WgpuCompositor {
            ops: Arc::new(WgpuCompositorOps {
                canvas_pipeline,
                canvas_render: self.canvas_render,
                device,
                queue,
            }),

            mutable: Arc::new(Mutex::new(MutableWgpuCompositor {
                width,
                height,
                layers: Default::default(),
            })),
        })
    }

    async fn init_wgpu() -> Result<(Device, Queue)> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .ok_or(Error::RequestAdapterError)?;

        Ok(adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WgpuCompositor"),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                    ..Default::default()
                },
                None,
            )
            .await?)
    }
}

#[derive(Default)]
struct MutableWgpuCompositor {
    width: u32,
    height: u32,
    layers: Vec<WgpuLayer>,
}

impl MutableWgpuCompositor {
    fn compositing(&mut self) -> (Vec<WgpuLayer>, u32, u32) {
        (self.layers.drain(..).collect(), self.width, self.height)
    }

    fn insert_layer(&mut self, layer: WgpuLayer) {
        self.layers.push(layer);
    }

    fn append_layer(&mut self, mut layers: Vec<WgpuLayer>) {
        self.layers.append(&mut layers);
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn size(&mut self) -> (u32, u32) {
        (self.width, self.height)
    }
}

/// [`Compositor`](crate::Compositor) implementation backed with [`wgpu`](https://docs.rs/wgpu/latest/wgpu/)
#[derive(Clone)]
pub struct WgpuCompositor {
    ops: Arc<WgpuCompositorOps>,
    mutable: Arc<Mutex<MutableWgpuCompositor>>,
}

impl Deref for WgpuCompositor {
    type Target = WgpuCompositorOps;
    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl WgpuCompositor {
    /// Returns a `WgpuCompositor` builder.
    pub fn new() -> WgpuCompositorBuilder {
        WgpuCompositorBuilder::default()
    }
}

#[async_trait]
impl DriverCompositor for WgpuCompositor {
    fn clone(&self) -> Compositor {
        Clone::clone(self).into()
    }

    async fn resize(&self, width: u32, height: u32) -> Result<()> {
        self.mutable.lock().resize(width, height);
        Ok(())
    }

    async fn size(&self) -> Result<(u32, u32)> {
        Ok(self.mutable.lock().size())
    }

    async fn create_canvas(&self, resize: Option<Rect>) -> Result<Canvas> {
        let canvas = WgpuCanvas::new(resize);

        self.mutable
            .lock()
            .insert_layer(Clone::clone(&canvas).into());

        Ok(canvas.into())
    }

    async fn compositing(&self) -> Result<()> {
        let (layers, width, height) = self.mutable.lock().compositing();

        let mut command_buffers = vec![];
        let mut valid_layers = vec![];

        for layer in layers {
            match layer.render(
                &self.ops.canvas_render,
                &self.ops.device,
                &self.ops.canvas_pipeline,
                width,
                height,
            ) {
                Ok(buff) => {
                    if let Some(buff) = buff {
                        command_buffers.push(buff);
                    }

                    valid_layers.push(layer);
                }
                Err(Error::Done) => {
                    continue;
                }
                Err(err) => return Err(err),
            }
        }

        self.ops.queue.submit(command_buffers);

        for layer in &valid_layers {
            layer.sync(&self.ops.device);
        }

        self.mutable.lock().append_layer(valid_layers);

        self.ops.device.poll(wgpu::MaintainBase::Wait);

        Ok(())
    }
}
