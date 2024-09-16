use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use spin::Mutex;
use wgpu::{Device, Queue, RenderPipeline, ShaderSource, Texture, TextureView};

use crate::{syscall::DriverCompositor, Canvas, Error, Rect, Result, Vertex};

use super::{canvas::WgpuCanvas, WgpuLayer};

pub struct WgpuCompositorOps {
    #[allow(unused)]
    /// rendering target.
    target: TextureView,
    /// Render pipeline for canvas layer.
    canvas_pipeline: RenderPipeline,
    /// Device associated with this `WgpuCompositor`
    device: Device,
    /// Queue associated with this `WgpuCompositor`
    queue: Queue,
}

pub struct WgpuCompositorBuilder {
    canvas_shader: ShaderSource<'static>,
}

impl Default for WgpuCompositorBuilder {
    fn default() -> Self {
        Self {
            canvas_shader: ShaderSource::Wgsl(include_str!("./canvas.wgsl").into()),
        }
    }
}

impl WgpuCompositorBuilder {
    /// Create a new [`WgpuCompositor`] that render result to a memory texture.
    pub async fn to_texture(self, width: u32, height: u32) -> Result<(WgpuCompositor, Texture)> {
        let (device, queue) = Self::init_wgpu().await?;

        let canvas_pipeline = Self::create_canvas_pipeline(&device, self.canvas_shader);

        let texture = Self::create_texture(&device, width, height);

        let target = texture.create_view(&Default::default());

        Ok((
            WgpuCompositor {
                ops: Arc::new(WgpuCompositorOps {
                    canvas_pipeline,
                    device,
                    queue,
                    target,
                }),

                mutable: Arc::new(Mutex::new(MutableWgpuCompositor {
                    width,
                    height,
                    layers: Default::default(),
                })),
            },
            texture,
        ))
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

    fn create_texture(device: &Device, width: u32, height: u32) -> Texture {
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        };

        device.create_texture(&texture_desc)
    }

    fn create_canvas_pipeline(device: &Device, shader_source: ShaderSource<'_>) -> RenderPipeline {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CanvasLayer"),
            source: shader_source,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("CanvasLayer"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("CanvasLayer"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        render_pipeline
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
    async fn resize(&self, width: u32, height: u32) -> Result<()> {
        self.mutable.lock().resize(width, height);
        Ok(())
    }

    async fn size(&self) -> Result<(u32, u32)> {
        Ok(self.mutable.lock().size())
    }
    /// Create a new canvas layer with initial position and size.
    async fn create_canvas(&self, resize: Option<Rect>) -> Result<Canvas> {
        let canvas = WgpuCanvas::new(resize);

        self.mutable
            .lock()
            .insert_layer(Clone::clone(&canvas).into());

        Ok(canvas.into())
    }

    /// Display compositing effects.
    async fn compositing(&self) -> Result<()> {
        let (layers, width, height) = self.mutable.lock().compositing();

        let mut command_buffers = vec![];
        let mut valid_layers = vec![];

        for layer in layers {
            match layer.render(&self.ops.device, &self.ops.canvas_pipeline, width, height) {
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
