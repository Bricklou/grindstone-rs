/// Custom Result type with [`GrindstoneError`](GrindstoneError) as Error type
pub type GrindstoneResult<T> = Result<T, GrindstoneError>;

#[derive(Debug, thiserror::Error)]
/// Custom Error type for the whole library
pub enum GrindstoneError {
    #[error("Invalid configuration `{0}`")]
    InvalidConfig(String),

    /// Standard IO Error.
    #[error("{0}")]
    IO(std::io::Error),
}

impl From<std::io::Error> for GrindstoneError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
