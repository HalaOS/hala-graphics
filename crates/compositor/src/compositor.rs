use crate::macros::driver_wrapper;

/// A compositor implementation must implement traits in this mod.
pub mod syscall {
    use async_trait::async_trait;

    use crate::{Id, Rect, Result, Vertex};

    use super::*;

    #[async_trait]
    pub trait DriverCompositor: Sync + Send {
        /// Clone self.
        fn clone(&self) -> Compositor;

        /// Adjust the compositor rendering size.
        async fn resize(&self, width: u32, height: u32) -> Result<()>;

        /// Returns the rendering size.
        async fn size(&self) -> Result<(u32, u32)>;

        /// Create a new canvas layer with initial position and size.
        async fn create_canvas(&self, resize: Option<Rect>) -> Result<Canvas>;

        /// Create a svg layer.
        async fn create_svg(&self) -> Result<Svg>;

        /// Display compositing effects.
        async fn compositing(&self) -> Result<()>;
    }

    /// The canvas layer.
    #[async_trait]
    pub trait DriverCanvas: Sync + Send {
        /// Returns the layer id.
        fn id(&self) -> &Id;
        /// Move this canvas's position and size.
        async fn layer_move(&self, rect: Rect) -> Result<()>;

        /// Update rendering data.
        async fn update(&self, vertices: Vec<Vertex>, indeces: Vec<u32>) -> Result<()>;

        /// Capture layer content with bitmap.
        async fn capture(&self) -> Result<Vec<u8>>;
    }

    /// A svg layer.
    #[async_trait]
    pub trait DriverSvg {}
}

driver_wrapper!(
    ["A type wrapper of [`DriverCompositor`](syscall::DriverCompositor)"]
    Compositor[syscall::DriverCompositor]
);

driver_wrapper!(
    ["A type wrapper of [`DriverCanvas`](syscall::DriverCanvas)"]
    Canvas[syscall::DriverCanvas]
);

driver_wrapper!(
    ["A type wrapper of [`DriverSvg`](syscall::DriverSvg)"]
    Svg[syscall::DriverSvg]
);
