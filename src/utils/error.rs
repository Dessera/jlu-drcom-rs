use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrcomError {
  #[error("Io error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Parse error: {0}")]
  ParseError(#[from] std::num::ParseIntError),

  #[error("lock error: {0}")]
  LockError(String),

  #[error("Os error: {0}")]
  OsError(String),

  #[error("Challenge error: {0}")]
  ChallengeError(String),

  #[error("Login error: {0}")]
  LoginError(String),

  #[error("Logout error: {0}")]
  LogoutError(String),
}

// use DrResult = Result<(), DrcomError>;
pub type DrResult<T> = Result<T, DrcomError>;
