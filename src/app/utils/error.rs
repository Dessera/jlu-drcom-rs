use thiserror::Error;

#[derive(Debug, Error)]
pub enum DrcomError {
  #[error("Io error: {0}")]
  IoError(#[from] std::io::Error),

  #[error("Lock error: {0}")]
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
