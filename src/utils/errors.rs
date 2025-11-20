use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    InternalServer(String),
    Database(String),
    RabbitMQ(String),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::InternalServer(msg) => write!(f, "Internal Server Error: {}", msg),
            AppError::Database(msg) => write!(f, "Database Error: {}", msg),
            AppError::RabbitMQ(msg) => write!(f, "RabbitMQ Error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad Request"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Forbidden(_) => (StatusCode::FORBIDDEN, "Forbidden"),
            AppError::InternalServer(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database Error"),
            AppError::RabbitMQ(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Message Queue Error"),
            AppError::Validation(_) => (StatusCode::BAD_REQUEST, "Validation Error"),
        };
        
        let error_response = ErrorResponse {
            status: status.as_u16(),
            message: self.to_string(),
            error: error_type.to_string(),
            details: None,
        };
        
        HttpResponse::build(status).json(error_response)
    }
}

// Conversiones automáticas
impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<lapin::Error> for AppError {
    fn from(err: lapin::Error) -> Self {
        AppError::RabbitMQ(err.to_string())
    }
}