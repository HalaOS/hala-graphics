//! cotati is a general-purpose vector drawing interface that does not restrict binding to a back-end renderer.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
pub use errors::*;
mod primitives;
pub use primitives::*;
mod renderers;
pub use renderers::*;
mod drawing;
pub use drawing::*;

pub mod combinator;
