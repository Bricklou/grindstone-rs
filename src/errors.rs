/// Custom Result type with [`GrindstoneError`](GrindstoneError) as Error type
pub type GrindstoneResult<T> = Result<T, GrindstoneError>;

#[derive(Debug, thiserror::Error)]
/// Custom Error type for the whole library
pub enum GrindstoneError {}
