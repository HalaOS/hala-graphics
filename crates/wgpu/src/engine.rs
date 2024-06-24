use std::{ops::Deref, sync::Arc};

use futures::{
    channel::mpsc::{self, SendError},
    SinkExt,
};
use uuid::Uuid;
use wgpu::{CommandBuffer, Device, Queue, RequestDeviceError};

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error(transparent)]
    SendError(#[from] SendError),

    #[error("Not found valid adapters.")]
    RequestAdapterError,

    /// Wrapper of [`wgpu::RequestDeviceError`]
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
}

/// `Result` type for wgpu engine crate.
pub type Result<T> = std::result::Result<T, RenderError>;

/// A variant of render layer command.
pub(crate) enum RenderLayerCommand {
    /// Sync wgpu command buffer.
    CommandBuffers(Vec<wgpu::CommandBuffer>),

    /// Layer size changed.
    Resize(u32, u32),
}

/// The wgpu context for render layer.
#[derive(Clone)]
pub struct RenderLayerContext {
    id: Uuid,
    device: Arc<Device>,
    cmd_sender: mpsc::Sender<RenderLayerCommand>,
}

impl RenderLayerContext {
    /// Create new `RenderLayerContext` instance with provided [`Arc<Device>`]
    pub(crate) fn new(device: Arc<Device>, cmd_sender: mpsc::Sender<RenderLayerCommand>) -> Self {
        Self {
            id: Uuid::new_v4(),
            device,
            cmd_sender,
        }
    }

    /// Get the id of this `RenderLayer`
    pub fn to_id(&self) -> &Uuid {
        &self.id
    }

    /// Resize render layer.
    pub async fn resize(&mut self, width: u32, height: u32) -> Result<()> {
        self.cmd_sender
            .send(RenderLayerCommand::Resize(width, height))
            .await?;

        Ok(())
    }

    /// Submits a series of finished command buffers for execution.
    pub async fn submit<I: IntoIterator<Item = CommandBuffer>>(
        &mut self,
        command_buffers: I,
    ) -> Result<()> {
        self.cmd_sender
            .send(RenderLayerCommand::CommandBuffers(
                command_buffers.into_iter().collect(),
            ))
            .await?;

        Ok(())
    }
}

impl Deref for RenderLayerContext {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

/// Hala graphics render engine with wgpu backend.
pub struct RenderEngine {
    /// wgpu device.
    device: Arc<Device>,
    /// wgpu command queue.
    queue: Queue,

    cmd_sender: mpsc::Sender<RenderLayerCommand>,

    cmd_receiver: mpsc::Receiver<RenderLayerCommand>,
}

impl RenderEngine {
    /// Create a new wgpu render engine instance.
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

        let (cmd_sender, cmd_receiver) = mpsc::channel(1024);

        Ok(Self {
            device: Arc::new(device),
            queue,

            cmd_receiver,
            cmd_sender,
        })
    }

    /// Create a new associated render layer instance, and returns this layer's context.
    pub fn create_layer(&self) -> RenderLayerContext {
        RenderLayerContext::new(self.device.clone(), self.cmd_sender.clone())
    }
}
