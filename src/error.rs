use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unknown error {0}")]
    Generic(String)
}