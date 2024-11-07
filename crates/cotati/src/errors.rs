use std::num::{ParseFloatError, ParseIntError};

/// Error variant used by this mod.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid viewport string: {0}")]
    InvalidViewPortStr(String),

    #[error("Unknown color: {0}")]
    UnrecognizedColor(String),

    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    /// Error raised by [`read`](svg::read) or [`open`](svg::open) fns.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Invalid length string: {0}")]
    LengthStr(String),

    #[error("Invalid length unit: {0}")]
    LengthUnit(String),

    #[error("Invalid transform string: {0}")]
    TransformStr(String),
}

/// Result type used by this mod.
pub type Result<T> = std::result::Result<T, Error>;
