use std::sync::Arc;

use futures::{channel::mpsc::SendError, lock::Mutex};

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

#[derive(Debug)]
struct WgpuContext {
    device: Device,
    queue: Queue,
}

/// The context object for render engine.
#[derive(Debug, Clone)]
#[allow(unused)]
pub struct RenderContext {
    wgpu_context: Arc<WgpuContext>,
    raw: Arc<Mutex<RenderContextRaw>>,
}

struct RenderContextRaw {
    /// cache of command buffers for current frame.
    command_buffers: Vec<CommandBuffer>,
}
