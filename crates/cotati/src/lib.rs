//! cotati is a general-purpose vector drawing language that does not restrict binding to a back-end renderer.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]

mod errors;
pub use errors::*;

pub mod ir;
