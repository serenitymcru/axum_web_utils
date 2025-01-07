use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<'a> {
  message: &'a str
}

pub struct Error(anyhow::Error);

pub type Result<T> = core::result::Result<T, Error>;

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(ErrorResponse {
        message: &self.0.to_string()
      })
    ).into_response()
  }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}