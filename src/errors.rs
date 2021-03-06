use actix_web::{error::ResponseError, http::StatusCode, HttpResponse, HttpResponseBuilder};

use serde::Serialize;

use derive_more::Display;

#[derive(Serialize)]
struct JsonError {
    status: u16,
    message: String,
}

#[derive(Debug, Display)]
pub enum PinkError {
    #[display(fmt = "not found")]
    NotFound,

    #[allow(dead_code)]
    #[display(fmt = "internal error")]
    InternalServerError,

    #[display(fmt = "bad request: {}", message)]
    BadRequest { message: String },

    #[display(fmt = "unauthorized: {}", message)]
    Unauthorized { message: String },
}

impl ResponseError for PinkError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        HttpResponseBuilder::new(status).json(JsonError {
            status: status.as_u16(),
            message: self.to_string(),
        })
    }
}
