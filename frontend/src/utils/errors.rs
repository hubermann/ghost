use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("network: {0}")]
    Network(String),
    #[error("http {0}")]
    Http(u16),
    #[error("decode")]
    Decode,
    #[error("unknown")]
    Unknown,
}
