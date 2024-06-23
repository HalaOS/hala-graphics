use std::ops::Deref;

use wgpu::RequestDeviceError;

/// Test error variant.
#[derive(thiserror::Error, Debug)]
pub enum RenderError {
    /// Wrapper of [`wgpu::RequestDeviceError`]
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),

    #[error("Not found valid adapters.")]
    RequestAdapterError,
}

/// Result type for mod test.
pub type Result<T> = std::result::Result<T, RenderError>;

/// The wgpu context object for hala graphics program.
pub struct RenderEngine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    /// cached frame update commands.
    frame_update_cmds: Vec<wgpu::CommandBuffer>,
}

impl RenderEngine {
    /// Create a new wgpu context object with default configuration.
    pub async fn new() -> Result<Self> {
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
            frame_update_cmds: Vec::new(),
        })
    }

    /// Create new wgpu texture instance.
    pub fn create_texture(&self, desc: &wgpu::TextureDescriptor) -> wgpu::Texture {
        self.device.create_texture(desc)
    }

    /// Starts a new frame rendering process
    pub fn begin_frame_update(&mut self) {
        self.frame_update_cmds.drain(..);
    }

    /// render provided layer.
    pub fn render_layer<L: RenderLayer>(&mut self, layer: &mut L) {
        if layer.redraw() {
            let encoder = layer.render(&self.device);

            self.frame_update_cmds.push(encoder.finish());
        }
    }

    /// Close the frame update process and write the changes of the current frame to the texture.
    pub fn end_frame_update(&mut self, _text_view: Option<&wgpu::TextureView>) {
        self.queue
            .submit(self.frame_update_cmds.drain(..).into_iter());
    }
}

impl Deref for RenderEngine {
    type Target = wgpu::Device;
    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

/// The rendering layer is the smallest unit of rendering logic in the engine
/// and is responsible for managing the rendering pipeline and inputs / outputs
pub trait RenderLayer {
    /// Calls this function to redraw this rendering layer and returns [`wgpu::CommandEncoder`] if successfully.
    ///
    /// After calling this function, the `RenderLayer` implementation must clean the flag of redraw.
    fn render(&mut self, device: &wgpu::Device) -> wgpu::CommandEncoder;

    /// Creates a view of this layer's output texture.
    fn create_view(&self, descriptor: &wgpu::TextureViewDescriptor) -> wgpu::TextureView;

    /// Returns true if the layer needs to be redrawn.
    fn redraw(&self) -> bool;
}

/// Render texture size.
pub struct RenderSize(pub u32, pub u32);
