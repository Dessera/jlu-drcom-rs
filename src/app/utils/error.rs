
#[derive(Debug)]
pub enum DrcomError {
  DataChallengeError(String),
}

// Error implement
impl std::fmt::Display for DrcomError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      DrcomError::DataChallengeError(msg) => write!(f, "DataChallengeError: {}", msg),
    }
  }
}

// use DrResult = Result<(), DrcomError>;
pub type DrResult<T> = Result<T, DrcomError>;
