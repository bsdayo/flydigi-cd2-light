use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("USB error: {0}")]
    Rusb(#[from] rusb::Error),

    #[error("LED count mismatch: expected {expected}, got {actual}")]
    LedCountMismatch { expected: usize, actual: usize },
}
