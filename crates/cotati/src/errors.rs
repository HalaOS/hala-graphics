use std::num::{ParseFloatError, ParseIntError};

/// Error variant used by `cotati`.
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

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Invalid length string: {0}")]
    LengthStr(String),

    #[error("Invalid length unit: {0}")]
    LengthUnit(String),

    #[error("Invalid transform string: {0}")]
    TransformStr(String),

    #[error("Invalid angle string: {0}")]
    Angle(String),

    #[error("{0}")]
    XmlDOM(#[from] xml_dom::level2::Error),

    #[error("The stack is less than {0}")]
    Pop(usize),
}

/// Result type used by this `cotati`.
pub type Result<T> = std::result::Result<T, Error>;
