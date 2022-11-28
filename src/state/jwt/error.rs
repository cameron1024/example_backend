use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum JwtError {
    #[error("nbf field before now")]
    TooEarly,
    #[error("exp field after now")]
    TooLate,
    #[error("invalid signature")]
    InvalidSig,
    #[error("unknown error")]
    Unknown(#[from] jsonwebtoken::errors::Error),
}
