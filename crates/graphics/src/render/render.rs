use crate::driver_wrapper;

/// A rendering widget must impl the Driver-* traits in this mod.
pub mod render_syscall {
    use crate::{Transform2D, Transform3D, Viewport};
    use wgpu::{CommandEncoder, Device, RenderPass, Texture};

    /// A rendering widget used by wgpu backend.
    pub trait DriverElement: Sync + Send {
        /// Attach this element to a wgpu [`Device`]
        fn attach(&self, device: &Device);

        /// Detach this element from wgpu [`Device`]
        fn detach(&self);

        /// Returns true if this element had already attached to one wgpu [`Device`].
        fn is_attached(&self) -> bool;

        fn before_redraw(
            &self,
            device: &Device,
            render_attachment: &Texture,
            command_encoder: &mut CommandEncoder,
            viewport: &Viewport,
        );

        /// Redraw this widget.
        fn redraw(&self, device: &Device, render_pass: &mut RenderPass<'_>, viewport: &Viewport);

        fn after_redraw(
            &self,
            device: &Device,
            render_attachment: &Texture,
            command_encoder: &mut CommandEncoder,
            viewport: &Viewport,
        );

        fn submit(&self, device: &Device);
    }

    /// A 2d rendering element .
    pub trait DriverElement2D: DriverElement {
        fn transform(&self, transform: Transform2D);
    }

    /// A 3d rendering element .
    pub trait DriverElement3D: DriverElement {
        fn transform(&self, transform: Transform3D);
    }
}

driver_wrapper!(
    ["A type wrapper of [`DriverElement`](render_syscall::DriverElement)"]
    Element[render_syscall::DriverElement]
);
