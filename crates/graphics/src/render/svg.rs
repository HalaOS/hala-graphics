use std::sync::Arc;

use spin::Mutex;
use wgpu::{CommandEncoder, RenderPipeline, RenderPipelineDescriptor, Texture};
use wgpu_derive::Vertex;

use crate::Viewport;

use super::render_syscall::{DriverElement, DriverElement2D};

#[repr(C)]
#[derive(Copy, Clone, Debug, Vertex, bytemuck::Pod, bytemuck::Zeroable)]
struct SvgVectex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Default)]
struct RawSvgElement {
    render_pipeline: Option<RenderPipeline>,
    transform: Option<crate::Transform2D>,
}

/// A rendering element for svg image.
#[derive(Clone)]
pub struct SvgElement {
    label: Option<String>,
    raw: Arc<Mutex<RawSvgElement>>,
}

impl SvgElement {
    /// Create a new [`SvgElement`]
    pub fn new(label: Option<&str>) -> Self {
        Self {
            label: label.map(|label| label.to_owned()),
            raw: Default::default(),
        }
    }
}

impl DriverElement for SvgElement {
    fn attach(&self, device: &wgpu::Device) {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CanvasLayer"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shader/svg.wgsl").into()),
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: self.label.as_deref(),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[SvgVectex::vertex_buff_layout()],
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

        let mut raw = self.raw.lock();

        raw.render_pipeline = Some(render_pipeline);
    }

    fn detach(&self) {}

    fn is_attached(&self) -> bool {
        self.raw.lock().render_pipeline.is_some()
    }

    fn submit(&self, _device: &wgpu::Device) {}

    #[allow(unused)]
    fn before_redraw(
        &self,
        device: &wgpu::Device,
        render_attachment: &Texture,
        command_encoder: &mut CommandEncoder,
        viewport: &Viewport,
    ) {
    }

    #[allow(unused)]
    fn redraw(
        &self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass<'_>,
        viewport: &Viewport,
    ) {
    }

    #[allow(unused)]
    fn after_redraw(
        &self,
        device: &wgpu::Device,
        render_attachment: &Texture,
        command_encoder: &mut CommandEncoder,
        viewport: &Viewport,
    ) {
    }
}

impl DriverElement2D for SvgElement {
    fn transform(&self, transform: crate::Transform2D) {
        self.raw.lock().transform = Some(transform);
    }
}
