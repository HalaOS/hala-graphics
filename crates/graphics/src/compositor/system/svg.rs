use wgpu::{Device, RenderPipeline, RenderPipelineDescriptor, ShaderSource};

use crate::compositor::{SvgComponent, SvgVertex};

use super::RenderSystem;

pub struct SvgSystem {
    /// The debug label for this system.
    #[allow(unused)]
    label: Option<String>,

    /// Render pipeline of this system.
    #[allow(unused)]
    render_pipeline: RenderPipeline,
}

impl SvgSystem {
    /// Create new [`SvgSystem`] with default shader.
    pub fn new(label: Option<&str>, device: &Device) -> Self {
        Self::with_shader(
            label,
            device,
            ShaderSource::Wgsl(include_str!("./shader/svg.wgsl").into()),
        )
    }

    /// Create [`SvgSystem`] with provided [`ShaderSource`]
    pub fn with_shader(label: Option<&str>, device: &Device, shader_source: ShaderSource) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("CanvasLayer"),
            source: shader_source,
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label,
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[SvgVertex::vertex_buff_layout()],
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

        SvgSystem {
            label: label.map(|label| label.to_owned()),
            render_pipeline,
        }
    }
}

impl RenderSystem for SvgSystem {
    fn prepare(
        &self,
        _world: &mut ecsrs::World,
        _viewport: &crate::Viewport,
        _command_encoder: &mut wgpu::CommandEncoder,
    ) {
    }

    fn redraw<'a>(
        &self,
        world: &mut ecsrs::World,
        _viewport: &crate::Viewport,
        _render_pass: &mut wgpu::RenderPass<'a>,
    ) {
        for component in world.component_iter::<SvgComponent>() {
            log::trace!(
                "svg({}), vertex({}) indecs({})",
                component.id,
                component.tessellated.vertexes.len(),
                component.tessellated.indecs.len()
            );
        }
    }

    fn composite(
        &self,
        _world: &mut ecsrs::World,
        _viewport: &crate::Viewport,
        _command_encoder: &mut wgpu::CommandEncoder,
    ) {
    }
}
