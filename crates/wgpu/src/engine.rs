use std::collections::HashMap;

use uuid::Uuid;
use wgpu::{
    util::DeviceExt, CommandBuffer, Device, Extent3d, Queue, RenderPipeline, RequestDeviceError,
    Texture, TextureView, TextureViewDescriptor,
};

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Not found valid adapters.")]
    RequestAdapterError,

    /// Wrapper of [`wgpu::RequestDeviceError`]
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}

/// `Result` type for wgpu engine crate.
pub type Result<T> = std::result::Result<T, RenderError>;

struct RenderLayerContext {
    attached: bool,
    /// The id of parent layer or `None` for direct surface child layer.
    parent: Option<Uuid>,
    /// x-axis offset of the rectangular origin within the parent coordinates
    x: u32,
    /// y-axis offset of the rectangular origin within the parent coordinates
    y: u32,
    /// Rendering layer width in pixels
    width: u32,
    /// Rendering layer height in pixels
    height: u32,
    /// The wgpu texture associated with this rendering layer.
    texture: Texture,
    /// The wgpu pipeline associated with this rendering layer.
    pipeline: RenderPipeline,
}

/// Hala graphics rendering engine with wgpu backend.
#[allow(unused)]
pub struct RenderEngine {
    device: Device,
    queue: Queue,
    layers: HashMap<Uuid, RenderLayerContext>,
    frame_command_buffers: Vec<CommandBuffer>,
}

impl RenderEngine {
    /// Create a new `RenderEngine` instance.
    pub fn new() -> Result<Self> {
        pollster::block_on(Self::new_async())
    }

    async fn new_async() -> Result<Self> {
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
            .ok_or(RenderError::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        Ok(Self {
            device,
            queue,
            frame_command_buffers: Default::default(),
            layers: Default::default(),
        })
    }

    pub fn attach<L: RenderLayer>(
        &mut self,
        parent: Option<&Uuid>,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        layer: &mut L,
    ) {
        // first attach
        if layer.id().is_some() {
            let context = self
                .layers
                .get_mut(layer.id().expect("Call attach first"))
                .unwrap();

            if context.attached {
                panic!("Call attach twice without calling detach.");
            }

            layer.on_attach(parent);

            if context.width != width || context.height != height {
                let texture = Self::create_texture(&self.device, width, height);
                context.texture = texture;
                layer.on_resize(width, height);
            }

            context.x = x;
            context.y = y;
            context.width = width;
            context.height = height;
            context.parent = parent.map(|id| id.clone());
        } else {
            let id = Uuid::new_v4();
            let pipeline = layer.on_create(id, &self.device);

            let texture = Self::create_texture(&self.device, width, height);

            let context = RenderLayerContext {
                attached: true,
                parent: parent.map(|p| p.clone()),
                x,
                y,
                width,
                height,
                pipeline,
                texture,
            };

            self.layers.insert(id, context);

            layer.on_attach(parent);
            layer.on_resize(width, height);
        }
    }

    pub fn detach<L: RenderLayer>(&mut self, layer: &mut L) {
        let context = self
            .layers
            .get_mut(layer.id().expect("Call attach first"))
            .unwrap();

        context.attached = false;
    }

    pub fn resize<L: RenderLayer>(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        layer: &mut L,
    ) {
        let context = self
            .layers
            .get_mut(layer.id().expect("Call attach first"))
            .unwrap();

        if context.width != width || context.height != height {
            let texture = Self::create_texture(&self.device, width, height);
            context.texture = texture;
            layer.on_resize(width, height);
        }

        context.x = x;
        context.y = y;
        context.width = width;
        context.height = height;
    }

    pub fn render<L: RenderLayer>(&mut self, layer: &mut L) {
        let context = self
            .layers
            .get_mut(layer.id().expect("Call attach first"))
            .unwrap();

        let command_buffer = layer.on_render(
            &self.device,
            &context.pipeline,
            &context
                .texture
                .create_view(&TextureViewDescriptor::default()),
        );

        self.frame_command_buffers.push(command_buffer);
    }

    fn create_texture(device: &Device, width: u32, height: u32) -> Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some(""),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
        })
    }
}

pub trait RenderLayer {
    fn id(&self) -> Option<&Uuid>;

    fn on_create(&mut self, uuid: Uuid, device: &Device) -> RenderPipeline;

    fn on_attach(&mut self, parent: Option<&Uuid>);

    fn on_resize(&mut self, width: u32, height: u32);

    fn on_render(
        &mut self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        texture: &TextureView,
    ) -> CommandBuffer;

    fn on_detach(&mut self);

    fn on_destroy(&mut self);
}

/// Vertex object for copying data from the vertex buffer or copying data to the vertex buffer.
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    /// Get format object of this type.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        const ATTRIBS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

pub struct TessellatorRenderLayer<'a> {
    id: Option<Uuid>,
    label: Option<String>,
    vertices: Vec<Vertex>,
    indeces: Vec<u32>,
    shader_source: wgpu::ShaderSource<'a>,
}

impl<'a> TessellatorRenderLayer<'a> {
    pub fn new(shader_source: wgpu::ShaderSource<'a>, label: Option<&str>) -> Self {
        Self {
            id: None,
            shader_source,
            vertices: Default::default(),
            indeces: Default::default(),
            label: label.map(|label| label.to_owned()),
        }
    }

    pub fn update_vertices(&mut self, vertices: Vec<Vertex>, indeces: Vec<u32>) {
        self.vertices = vertices;
        self.indeces = indeces;
    }
}

impl<'a> RenderLayer for TessellatorRenderLayer<'a> {
    fn id(&self) -> Option<&Uuid> {
        self.id.as_ref()
    }

    fn on_create(&mut self, _uuid: Uuid, device: &Device) -> RenderPipeline {
        log::trace!(
            "RenderLayer({}): event on_create",
            self.label.as_deref().unwrap_or("Tessellator")
        );

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CanvasLayer"),
            source: self.shader_source.clone(),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Hala graphic shader layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Hala graphic render pipeline"),
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
        });

        render_pipeline
    }

    fn on_attach(&mut self, parent: Option<&Uuid>) {
        log::trace!(
            "RenderLayer({}): event on_attach, parent={:?}",
            self.label.as_deref().unwrap_or("Tessellator"),
            parent,
        );
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        log::trace!(
            "RenderLayer({}): event on_resize, width={}, height={}",
            self.label.as_deref().unwrap_or("Tessellator"),
            width,
            height
        );
    }

    fn on_render(
        &mut self,
        device: &Device,
        render_pipeline: &RenderPipeline,
        texture: &TextureView,
    ) -> CommandBuffer {
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("CanvasLayer"),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("hala graphic vertex buffer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("hala graphic index buffer"),
            contents: bytemuck::cast_slice(&self.indeces),
            usage: wgpu::BufferUsages::INDEX,
        });

        {
            let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("CanvasLayer"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture,
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
            render_pass.draw_indexed(0..self.indeces.len() as u32, 0, 0..1);
        }

        command_encoder.finish()
    }

    fn on_detach(&mut self) {
        log::trace!(
            "RenderLayer({}): event on_detach",
            self.label.as_deref().unwrap_or("Tessellator"),
        );
    }

    fn on_destroy(&mut self) {
        log::trace!(
            "RenderLayer({}): event on_destroy",
            self.label.as_deref().unwrap_or("Tessellator"),
        );
    }
}
