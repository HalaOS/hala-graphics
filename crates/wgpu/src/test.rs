use std::sync::Arc;

use wgpu::{
    CreateSurfaceError, Features, FragmentState, MultisampleState, RequestDeviceError,
    SurfaceError, VertexState,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

/// Test error variant.
#[derive(thiserror::Error, Debug)]
pub enum TestError {
    /// Wrapper of [`winit::error::EventLoopError`]
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),

    /// Wrapper of [`wgpu::SurfaceError`]
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),

    /// Wrapper of [`wgpu::CreateSurfaceError`]
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),

    /// Wrapper of [`wgpu::RequestDeviceError`]
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),

    #[error("Not found valid adapters.")]
    RequestAdapterError,
}

/// Result type for mod test.
pub type Result<T> = std::result::Result<T, TestError>;

#[derive(Default)]
pub struct TestRunner {
    window: Option<Arc<Window>>,
    winit_wgpu_state: Option<WinitWgpuState>,
}

impl ApplicationHandler for TestRunner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.window = Some(window.clone());

        let state = match pollster::block_on(async move { WinitWgpuState::new(window).await }) {
            Ok(state) => state,
            Err(err) => {
                log::error!("Create winit wgpu state error: {}", err);
                event_loop.exit();
                return;
            }
        };

        self.winit_wgpu_state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::debug!("The close button was pressed.");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.winit_wgpu_state.as_mut().unwrap().render() {
                    log::error!("Redraw failed: {}", err);
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(new_size) => {
                self.winit_wgpu_state.as_mut().unwrap().resize(new_size);
                self.window.as_ref().unwrap().request_redraw();
            }
            event => {
                if !self.winit_wgpu_state.as_mut().unwrap().input(&event) {
                    // log::trace!("Unhandle window event: {:?}", event);
                }
            }
        }
    }
}

impl TestRunner {
    /// Create a new `WinitRunner` instance and run it.
    pub fn run() -> Result<()> {
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = TestRunner::default();

        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

/// Wgpu application state object.
#[allow(unused)]
struct WinitWgpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
}

#[allow(unused)]
impl WinitWgpuState {
    async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or(TestError::RequestAdapterError)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: Features::default(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![wgpu::TextureFormat::Bgra8UnormSrgb],
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Hala graphic shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
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
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: Default::default(),
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },

            multiview: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        })
    }

    fn resize(&mut self, new_size: PhysicalSize<u32>) {
        log::trace!("Window resize: {:?}", new_size);
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn render(&mut self) -> Result<()> {
        log::trace!("render...");

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("HalaOS wgpu encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.5,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
