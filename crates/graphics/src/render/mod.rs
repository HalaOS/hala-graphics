//! a rendering system with [`ECS`] pattern.
//!
//! [`ECS`]: https://www.wikiwand.com/en/articles/Entity_component_system

mod component;
pub use component::*;

mod system;
pub use system::*;

mod compositor;
pub use compositor::*;
