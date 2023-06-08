use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrcomError {
  #[error("io error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("challenge error: {0}")]
  ChallengeError(String),

  #[error("login error: {0}")]
  LoginError(String),

  #[error("logout error: {0}")]
  LogoutError(String),
}

// use DrResult = Result<(), DrcomError>;
pub type DrResult<T> = Result<T, DrcomError>;
