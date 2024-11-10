mod renderer;
pub use renderer::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
#[allow(unused)]
pub use mock::*;

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
