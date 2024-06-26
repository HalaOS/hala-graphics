use std::{collections::HashMap, sync::Arc};

use futures::{channel::mpsc::SendError, lock::Mutex};

use uuid::Uuid;
use wgpu::{CommandBuffer, Device, Queue, RequestDeviceError, Texture};

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

/// The inner rendering layer object for `RenderEngine`
#[allow(unused)]
struct RenderLayer {
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
}

/// Hala graphics rendering engine with wgpu backend.
#[allow(unused)]
pub struct RenderEngine {
    device: Device,
    queue: Queue,
    layers: HashMap<Uuid, RenderLayer>,
    frame_command_buffers: Vec<CommandBuffer>,
}
