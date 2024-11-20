/// Error variant used by `cotati`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsatisfied variant with register({0})")]
    UnsatisfiedVariant(usize),

    #[error("unrecognized color: {0}")]
    UnrecognizedColor(String),
}

/// Result type used by this `cotati`.
pub type Result<T> = std::result::Result<T, Error>;
