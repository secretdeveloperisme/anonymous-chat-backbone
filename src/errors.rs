use axum::{http::StatusCode, response::IntoResponse};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
  #[error("Failed to query from database {}", 0)]
  QueryError(String),

  #[error("Failed to get a connection: {}", 0.to_string())]
  ConnectionError(#[from] r2d2::Error),

  #[error("Constraint violation: {0}")]
  ConstraintViolation(String),

  #[error("TransactionError: {0}")]
  TransactionError(String),
}

impl IntoResponse for DBError {
  fn into_response(self) -> axum::response::Response {
    match self {
      Self::QueryError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
      DBError::ConstraintViolation(err) => {
        (StatusCode::BAD_REQUEST, err.to_string()).into_response()
      }
      Self::ConnectionError(err) => {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
      }
      Self::TransactionError(err) => {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
      }
    }
  }
}

impl From<diesel::result::Error> for DBError {
  fn from(error: diesel::result::Error) -> Self {
    match error {
      diesel::result::Error::DatabaseError(_, _) => DBError::QueryError(error.to_string()),
      diesel::result::Error::NotFound => DBError::QueryError("Record not found".to_string()),
      _ => DBError::TransactionError(error.to_string()), // Use TransactionError as a fallback
    }
  }
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
  #[error("Database error: cause {}", 0.to_string())]
  DatabaseError(DBError),

  #[error("The resource is not found: {0}")]
  NotFound(String),

  #[error("{0}")]
  ExistedResource(String),

  #[error("The user already joined the group")]
  AlreadyJoined,

  #[error("The current user doesn't have permission to access the resource")]
  Forbidden,

  #[error("The current user doesn't have right to access the resource")]
  Unauthorized,

  #[error("The request is missing {0}")]
  MissingField(String),

  #[error("Unknown error")]
  Unknown,
}
impl ApiError {
  pub fn new_database_query_err(cause: &str) -> Self {
    Self::DatabaseError(DBError::QueryError(cause.to_string()))
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> axum::response::Response {
    return match self {
      Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
      Self::AlreadyJoined => (StatusCode::BAD_REQUEST, self.to_string()),
      Self::ExistedResource(_) => (StatusCode::BAD_REQUEST, self.to_string()),
      Self::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
      Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
      Self::MissingField(_) => (StatusCode::BAD_REQUEST, self.to_string()),
      // Yes we want to hide internal message error from user
      err => {
        tracing::error!("Error Cause: {}", err.to_string());
        (
          StatusCode::SERVICE_UNAVAILABLE,
          "Service unavailable".to_string(),
        )
      }
    }
    .into_response();
  }
}
