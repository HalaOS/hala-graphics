/// Error variant used by `cotati`.
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Result type used by this `cotati`.
pub type Result<T> = std::result::Result<T, Error>;
