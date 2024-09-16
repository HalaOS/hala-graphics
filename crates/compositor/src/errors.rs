/// Errors returns by this mod.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error raised by [`wgpu`] mod.
    #[cfg(feature = "wgpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wgpu")))]
    #[error(transparent)]
    Wgpu(#[from] wgpu::Error),

    /// An error raised by [`request_device`](wgpu::Adapter::request_device) function.
    #[cfg(feature = "wgpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wgpu")))]
    #[error(transparent)]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    /// An error raised by [`wgpu`] mod.
    #[cfg(feature = "wgpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "wgpu")))]
    #[error(transparent)]
    BufferAsyncError(#[from] wgpu::BufferAsyncError),

    #[error("GPU adapter not found.")]
    RequestAdapterError,

    #[error("The canvas/windows is dropped.")]
    Done,
}

/// Result type returns by this mod.
pub type Result<T> = std::result::Result<T, Error>;
