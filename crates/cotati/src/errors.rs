//! `errors`,`result` types used by this crate.

/// Error variant used by `cotati`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsatisfied variant with register({0})")]
    UnsatisfiedVariant(String),

    #[error("unrecognized color: {0}")]
    UnrecognizedColor(String),
}

/// Result type used by this `cotati`.
pub type Result<T> = std::result::Result<T, Error>;
