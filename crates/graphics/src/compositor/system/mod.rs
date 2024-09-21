mod canvas;
pub use canvas::*;
use ecsrs::World;
use wgpu::{CommandEncoder, RenderPass};

use crate::Viewport;

/// A ecs system for rendering.
pub trait RenderSystem {
    fn prepare(&self, world: &mut World, viewport: &Viewport, command_encoder: &mut CommandEncoder);

    fn redraw<'a>(&self, world: &mut World, viewport: &Viewport, render_pass: &mut RenderPass<'a>);

    fn composite(
        &self,
        world: &mut World,
        viewport: &Viewport,
        command_encoder: &mut CommandEncoder,
    );
}
