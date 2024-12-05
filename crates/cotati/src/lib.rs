//! cotati is a general-purpose vector graphic language system.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod errors;

#[cfg(feature = "ir")]
#[cfg_attr(docsrs, doc(cfg(feature = "ir")))]
pub mod ir;

#[cfg(feature = "device")]
#[cfg_attr(docsrs, doc(cfg(feature = "device")))]
pub mod device;

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
