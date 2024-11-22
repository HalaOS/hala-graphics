//! An intermediate representation of the `VGL` language.
//!

mod animation;
pub use animation::*;

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

mod path;
pub use path::*;

mod filter;
pub use filter::*;
