//! An intermediate representation of the `VGL` language.
//!

mod context;
pub use context::*;

mod color;
pub use color::*;

mod dimension;
pub use dimension::*;

mod transform;
pub use transform::*;

mod ir;
pub use ir::*;

mod painting;
pub use painting::*;

mod shapes;
pub use shapes::*;

mod layer;
pub use layer::*;

mod gradients;
pub use gradients::*;

mod pattern;
pub use pattern::*;
