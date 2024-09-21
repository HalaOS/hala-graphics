//! Utilities for rendering svg images based on the [`compositor`](crate::compositor) system
//!
//!

#[cfg(not(target_arch = "wasm32"))]
mod native {

    use std::{path::Path, sync::Arc};

    use futures::executor::block_on;

    use winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        window::{Window, WindowId},
    };

    use winit::event_loop::{ControlFlow, EventLoop};

    use crate::{
        compositor::{Compositor, SurfaceCompositor, SvgTessellated},
        Viewport,
    };

    pub struct App<F> {
        title: String,
        viewport: Viewport,
        window: Option<Arc<Window>>,
        surface_compositor: Option<SurfaceCompositor<'static>>,
        on_resumed: F,
    }

    impl<F> App<F> {
        fn new<S: AsRef<str>>(title: S, viewport: Viewport, on_resumed: F) -> Self {
            Self {
                title: title.as_ref().to_string(),
                viewport,
                on_resumed,
                window: Default::default(),
                surface_compositor: Default::default(),
            }
        }
    }

    impl<F> ApplicationHandler for App<F>
    where
        F: FnMut(&mut SurfaceCompositor<'static>),
    {
        fn resumed(&mut self, event_loop: &ActiveEventLoop) {
            let attributes = Window::default_attributes()
                .with_inner_size(winit::dpi::PhysicalSize::new(
                    self.viewport.width,
                    self.viewport.height,
                ))
                .with_title(&self.title);

            let window = Arc::new(event_loop.create_window(attributes).unwrap());

            let mut compositor =
                block_on(Compositor::new().render_to_surface(window.clone(), self.viewport))
                    .unwrap();

            (self.on_resumed)(&mut compositor);

            self.window = Some(window);

            self.surface_compositor = Some(compositor);
        }

        fn window_event(
            &mut self,
            event_loop: &ActiveEventLoop,
            _id: WindowId,
            event: WindowEvent,
        ) {
            match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    self.surface_compositor
                        .as_mut()
                        .unwrap()
                        .compositing()
                        .unwrap();
                }
                WindowEvent::Resized(resize) => {
                    self.viewport = Viewport::new(resize.width, resize.height);

                    let compositor = self.surface_compositor.as_mut().unwrap();

                    compositor.resize(self.viewport);

                    self.window.as_ref().unwrap().request_redraw();
                }
                _ => (),
            }
        }
    }

    /// Render the provides svg on a new opening window.
    pub fn render_svg<P: AsRef<Path>>(
        label: Option<&str>,
        path: P,
        viewport: crate::Viewport,
    ) -> crate::Result<()> {
        let svg_tessellated = SvgTessellated::tessellate_with(path)?;

        let event_loop = EventLoop::new().unwrap();

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut app = App::new(
            label.unwrap_or("Render SVG"),
            viewport,
            |compositor: &mut SurfaceCompositor<'static>| {
                _ = compositor.new_svg(svg_tessellated.clone());
            },
        );

        Ok(event_loop.run_app(&mut app)?)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;
