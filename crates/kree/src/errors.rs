use png::EncodingError;
use wgpu::{BufferAsyncError, CreateSurfaceError, RequestDeviceError, SurfaceError};

/// The error type used by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No valid adapter.")]
    RequestAdapterError,

    /// Error returns by [`request_device`](wgpu::Adapter::request_device)
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),

    /// Error returns by [`map_async`](https://docs.rs/wgpu/latest/wgpu/struct.BufferSlice.html#method.map_async)
    #[error(transparent)]
    BufferAsyncError(#[from] BufferAsyncError),

    /// Error returns by png encoding.
    #[error(transparent)]
    PngEncodingError(#[from] EncodingError),

    /// std::io::Error
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Error returns by [`create_surface`](wgpu::Instance::create_surface)
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),

    /// Error returns by [`get_current_texture`](wgpu::Surface::get_current_texture)
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),

    #[error("Invalid viewport string: {0}")]
    InvalidViewPortStr(String),

    #[error("Unknown color: {0}")]
    UnrecognizedColor(String),
}

/// The result type used by this crate.
pub type Result<T> = std::result::Result<T, Error>;
