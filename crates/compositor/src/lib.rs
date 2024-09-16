//! The compositor for hala graphics system.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod compositor;
pub use compositor::*;

mod errors;
pub use errors::*;

mod primitives;
pub use primitives::*;

#[cfg(feature = "wgpu")]
#[cfg_attr(docsrs, doc(cfg(feature = "wgpu")))]
mod wgpu;

#[cfg(feature = "wgpu")]
pub use wgpu::*;

mod macros;
