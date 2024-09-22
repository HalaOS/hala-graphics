mod canvas;
pub use canvas::*;

mod layer;
pub use layer::*;

mod capture;
pub use capture::*;

mod redraw;
pub use redraw::*;

mod transform;

pub use transform::*;

// Attention!!! when add new component, don't change this order.
// Attention!!! never to remove deprecated component from this list.
ecsrs::ecs_system!(
    LayerComponent,
    RedrawComponent,
    Canvas2DComponent,
    CaptureComponent
);
