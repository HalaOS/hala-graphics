use std::ops::Deref;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use async_trait::async_trait;
use futures::future::poll_fn;
use spin::mutex::Mutex;

use syscall::DriverWgpuLayer;
use uuid::Uuid;
use wgpu::util::DeviceExt;
use wgpu::{
    Buffer, CommandBuffer, CommandEncoder, Device, Extent3d, Queue, RenderPipeline, ShaderSource,
    Texture, TextureView,
};

use crate::syscall::{DriverCanvas, DriverCompositor};
use crate::{Canvas, Error, Geometry, LayerId, Rect, Result, Vertex};

use crate::macros::driver_wrapper;

mod syscall {

    use super::*;

    #[async_trait]
    /// Trait used by compositor layer.
    pub trait DriverWgpuLayer: DriverCanvas {
        /// Returns `None`, if this layer is closed.
        fn render(
            &self,
            device: &Device,
            render_pipeline: &RenderPipeline,
            width: u32,
            height: u32,
            target: &TextureView,
        ) -> Result<Option<CommandBuffer>>;

        fn sync(&self, device: &Device);
    }
}

driver_wrapper!(
    ["A type wrapper of [`DriverCompositor`](syscall::DriverCompositor)"]
    WgpuLayer[syscall::DriverWgpuLayer]
);

fn create_layer_texture(device: &Device, width: u32, height: u32) -> Texture {
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

static U32_SIZE: u32 = std::mem::size_of::<u32>() as u32;

pub struct WgpuCompositorOps {
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
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Default::default(),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

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
    async fn create_canvas(&self, rect: Option<Rect>) -> Result<Canvas> {
        let canvas = WgpuCanvas {
            id: Uuid::new_v4(),
            mutable: Arc::new(Mutex::new(MutableWgpuCanvas {
                resize: rect,
                ..Default::default()
            })),
        };

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
            match layer.render(
                &self.ops.device,
                &self.ops.canvas_pipeline,
                width,
                height,
                &self.ops.target,
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

enum CaptureState {
    Init,
    Rendering(Arc<Buffer>),
    Sync(Result<Vec<u8>>),
}

#[derive(Default)]
struct Capture {
    waker: Option<Waker>,
    state: Option<CaptureState>,
}

impl Capture {
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Result<Vec<u8>>> {
        if self.state.is_none() {
            self.state = Some(CaptureState::Init);
            self.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }

        match self.state.take().unwrap() {
            CaptureState::Sync(r) => return Poll::Ready(r),
            state => {
                self.state = Some(state);
                self.waker = Some(cx.waker().clone());
                return Poll::Pending;
            }
        }
    }

    fn capture(&mut self, device: &Device, width: u32, height: u32) -> Option<Arc<Buffer>> {
        if self.state.is_none() {
            return None;
        }

        let buffer_size = (U32_SIZE * width * height) as wgpu::BufferAddress;

        if let Some(state) = self.state.as_ref() {
            match state {
                CaptureState::Rendering(_) | CaptureState::Sync(_) => return None,
                _ => {}
            }
        }

        let desc = wgpu::BufferDescriptor {
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            label: Some("CanvasLayer"),
            mapped_at_creation: false,
        };

        let buffer = Arc::new(device.create_buffer(&desc));

        self.state = Some(CaptureState::Rendering(buffer.clone()));

        return Some(buffer);
    }

    fn sync(&mut self) -> Option<Arc<Buffer>> {
        if let Some(CaptureState::Rendering(buffer)) = self.state.as_ref() {
            Some(buffer.clone())
        } else {
            None
        }
    }

    fn result(&mut self, result: Result<Vec<u8>>) -> Option<Waker> {
        if let Some(waker) = self.waker.take() {
            self.state = Some(CaptureState::Sync(result));
            Some(waker)
        } else {
            None
        }
    }
}

#[derive(Default)]
struct MutableWgpuCanvas {
    is_closed: bool,
    resize: Option<Rect>,
    geometry: Option<Geometry>,
    texture: Option<Texture>,
    capture: Capture,
}

impl MutableWgpuCanvas {
    fn texture_view(&mut self, device: &Device, width: u32, height: u32) -> TextureView {
        if let Some(texture) = self.texture.as_ref() {
            if texture.width() == width && texture.height() == height {
                return texture.create_view(&Default::default());
            }
        }

        let texture = create_layer_texture(device, width, height);

        let texture_view = texture.create_view(&Default::default());

        self.texture = Some(texture);

        texture_view
    }
}

#[derive(Clone, Default)]
struct WgpuCanvas {
    id: LayerId,
    mutable: Arc<Mutex<MutableWgpuCanvas>>,
}

impl Drop for WgpuCanvas {
    fn drop(&mut self) {
        self.mutable.lock().is_closed = true;
    }
}

impl WgpuCanvas {
    fn redraw(
        &self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        command_encoder: &mut CommandEncoder,
        width: u32,
        height: u32,
    ) -> Result<()> {
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
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("CanvasLayer"),
                contents: bytemuck::cast_slice(&geometry.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("CanvasLayer"),
                contents: bytemuck::cast_slice(&geometry.indeces),
                usage: wgpu::BufferUsages::INDEX,
            });

            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("CanvasLayer"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 0.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(render_pipeline); // 2.
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..geometry.indeces.len() as u32, 0, 0..1);
        }

        Ok(())
    }

    fn capture(
        &self,
        device: &Device,
        command_encoder: &mut CommandEncoder,
        width: u32,
        height: u32,
    ) {
        let mut mutable = self.mutable.lock();

        if let Some(buffer) = mutable.capture.capture(device, width, height) {
            command_encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: mutable.texture.as_ref().expect("Call redraw first"),
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(U32_SIZE * width),
                        rows_per_image: Some(height),
                    },
                },
                Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
        }
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
        poll_fn(|cx| {
            let mut mutable = self.mutable.lock();

            mutable.capture.poll(cx)
        })
        .await
    }
}

impl DriverWgpuLayer for WgpuCanvas {
    fn render(
        &self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        width: u32,
        height: u32,
        _target: &TextureView,
    ) -> Result<Option<CommandBuffer>> {
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("CanvasLayer"),
        });

        self.redraw(device, render_pipeline, &mut command_encoder, width, height)?;

        self.capture(device, &mut command_encoder, width, height);

        Ok(Some(command_encoder.finish()))
    }

    fn sync(&self, _device: &Device) {
        if let Some(buffer) = self.mutable.lock().capture.sync() {
            let capturable = buffer.clone();

            let this = self.clone();

            buffer
                .slice(..)
                .map_async(wgpu::MapMode::Read, move |result| {
                    let waker = match result {
                        Ok(_) => {
                            let view = capturable.slice(..).get_mapped_range();

                            let buf = view.to_vec();

                            drop(view);

                            capturable.unmap();

                            this.mutable.lock().capture.result(Ok(buf))
                        }

                        Err(err) => this.mutable.lock().capture.result(Err(err.into())),
                    };

                    if let Some(waker) = waker {
                        waker.wake();
                    }
                });
        }
    }
}

#[cfg(test)]
mod tests {

    use futures::poll;
    use image::{ImageBuffer, Rgb};
    use lyon::{
        geom::Box2D,
        math::point,
        path::{builder::BorderRadii, Winding},
        tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers},
    };

    use crate::Compositor;

    use super::*;

    async fn init() -> Result<(Compositor, Texture)> {
        let (compositor, texture) = WgpuCompositor::new().to_texture(1024, 1024).await?;

        Ok((compositor.into(), texture))
    }

    #[futures_test::test]
    async fn canvas_test() {
        let (compositor, _texture) = init().await.unwrap();

        let canvas = compositor.create_canvas(None).await.unwrap();

        let options = FillOptions::tolerance(0.1);
        let mut tessellator = FillTessellator::new();

        let mut geometry: VertexBuffers<Vertex, u32> = VertexBuffers::new();

        let mut builder = BuffersBuilder::new(&mut geometry, |vertex: FillVertex| {
            let position = vertex.position().to_array();
            Vertex {
                position: [position[0], position[1], 0.0],
                color: [1.0, 0.0, 0.0],
            }
        });

        let mut builder = tessellator.builder(&options, &mut builder);

        builder.add_rounded_rectangle(
            &Box2D {
                min: point(0.0, 0.0),
                max: point(100.0, 50.0),
            },
            &BorderRadii {
                top_left: 10.0,
                top_right: 5.0,
                bottom_left: 20.0,
                bottom_right: 25.0,
            },
            Winding::Positive,
        );

        builder.build().unwrap();

        canvas
            .update(geometry.vertices, geometry.indices)
            .await
            .unwrap();

        let mut capture = canvas.capture();

        assert!(poll!(&mut capture).is_pending());

        compositor.compositing().await.unwrap();

        let data = capture.await.unwrap();

        let (width, height) = compositor.size().await.unwrap();

        let buffer = ImageBuffer::<Rgb<u8>, _>::from_raw(width, height, data).unwrap();

        buffer.save("image.png").unwrap();
    }
}
