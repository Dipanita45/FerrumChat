use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize, de::value::Error};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ChatNotFound,
    ChatNotCreated,
    ConnectionOff,
    WrongCredentials,
    Unauthorized,
    InvalidChat,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    UserNotAuthenticated,
}

impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        match self {
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password cannot exceed {} characters", max_length)
            }
            ErrorMessage::InvalidHashFormat => "Invalid hash format".to_string(),
            ErrorMessage::HashingError => "Error hashing password".to_string(),
            ErrorMessage::InvalidToken => "Invalid token".to_string(),
            ErrorMessage::ChatNotFound => "Chat not found".to_string(),
            ErrorMessage::ChatNotCreated => "Chat could not be created".to_string(),
            ErrorMessage::ConnectionOff => "Connection is off".to_string(),
            ErrorMessage::WrongCredentials => "Wrong credentials".to_string(),
            ErrorMessage::Unauthorized => "Unauthorized".to_string(),
            ErrorMessage::EmailExist => "Email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User no longer exists".to_string(),
            ErrorMessage::InvalidChat => "Invalid chat".to_string(),
            ErrorMessage::TokenNotProvided => "Token not provided".to_string(),
            ErrorMessage::UserNotAuthenticated => "User not authenticated".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::NOT_FOUND,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",
            self.message, self.status
        )
    }
}

impl std::error::Error for HttpError {}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
