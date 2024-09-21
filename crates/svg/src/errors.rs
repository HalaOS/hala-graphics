/// Error variant used by this mod.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SvgParseError(#[from] svg::parser::Error),

    /// Error raised by [`read`](svg::read) or [`open`](svg::open) fns.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Result type used by this mod.
pub type Result<T> = std::result::Result<T, Error>;
