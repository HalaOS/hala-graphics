use wgpu::{util::DeviceExt, Extent3d};

use crate::{RenderLayer, RenderSize};

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

/// A rendering layer that implement canvas logic.
pub struct CanvasLayer<'a> {
    render_size: RenderSize,
    texture: Option<wgpu::Texture>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertices: Vec<Vertex>,
    indeces: Vec<u32>,
    redraw: bool,
    shader_source: wgpu::ShaderSource<'a>,
}

impl<'a> CanvasLayer<'a> {
    /// Create new canvas layer with provided `shader_source`.
    pub fn new(render_size: RenderSize, shader_source: wgpu::ShaderSource<'a>) -> Self {
        CanvasLayer {
            render_size,
            texture: None,
            render_pipeline: None,
            redraw: false,
            vertices: vec![],
            indeces: vec![],
            shader_source,
        }
    }

    /// Update render vertex and index datas.
    pub fn update<V: ToOwned<Owned = Vec<Vertex>>, I: ToOwned<Owned = Vec<u32>>>(
        &mut self,
        vertices: V,
        indeces: I,
    ) {
        self.vertices = vertices.to_owned();
        self.indeces = indeces.to_owned();
        self.redraw = true;
    }

    fn create_texture(&mut self, device: &wgpu::Device) {
        if self.texture.is_none() {
            self.texture = Some(device.create_texture(&wgpu::TextureDescriptor {
                label: Some(""),
                size: Extent3d {
                    width: self.render_size.0,
                    height: self.render_size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            }));
        }
    }

    fn create_pipeline(&mut self, device: &wgpu::Device) {
        if self.render_pipeline.is_none() {
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

            self.render_pipeline = Some(render_pipeline);
        }
    }
}

impl<'a> RenderLayer for CanvasLayer<'a> {
    fn render(&mut self, device: &wgpu::Device) -> wgpu::CommandEncoder {
        self.create_texture(device);
        self.create_pipeline(device);

        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("CanvasLayer"),
        });

        let texture_view = self.create_view(&Default::default());

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CanvasLayer"),
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CanvasLayer"),
            contents: bytemuck::cast_slice(&self.indeces),
            usage: wgpu::BufferUsages::INDEX,
        });

        {
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

            render_pass.set_pipeline(self.render_pipeline.as_ref().unwrap()); // 2.
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.indeces.len() as u32, 0, 0..1);
        }

        self.redraw = false;

        command_encoder
    }

    fn create_view(&self, desc: &wgpu::TextureViewDescriptor) -> wgpu::TextureView {
        self.texture
            .as_ref()
            .expect("Call render first")
            .create_view(desc)
    }

    fn redraw(&self) -> bool {
        self.texture.is_none() || self.redraw
    }
}
