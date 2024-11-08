//! cotati is a general-purpose vector drawing interface that does not restrict binding to a back-end renderer.

mod errors;
pub use errors::*;
mod primitives;
pub use primitives::*;
mod renderers;
pub use renderers::*;
mod draw;
pub use draw::*;

pub mod combinator;
