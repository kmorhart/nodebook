pub mod messages;

use messages::get;
use axum::http::StatusCode;
use std::env;
use std::backtrace::Backtrace;

fn is_dev() -> bool {
    env::var("APP_ENV")
        .map(|v| v == "development")
        .unwrap_or(false)
}

#[derive(Debug)]
pub enum AppError {
    Unauthorized(&'static str, Option<String>),
    MethodNotAllowed(&'static str, Option<String>),
    BadRequest(&'static str, Option<String>),
    Conflict(&'static str, Option<String>),
    NotFound(&'static str, Option<String>),
    Internal(&'static str, Option<String>),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::Unauthorized(_, _) => StatusCode::UNAUTHORIZED,
            AppError::MethodNotAllowed(_, _) => StatusCode::METHOD_NOT_ALLOWED,
            AppError::BadRequest(_, _) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_, _) => StatusCode::CONFLICT,
            AppError::NotFound(_, _) => StatusCode::NOT_FOUND,
            AppError::Internal(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn message(&self) -> &'static str {
        if is_dev() {
            return self.dev_message();
        }
        self.prod_message()
    }

    fn prod_message(&self) -> &'static str {
        match self {
            AppError::Unauthorized(key, _) => get(key),
            AppError::MethodNotAllowed(key, _) => get(key),
            AppError::BadRequest(key, _) => get(key),
            AppError::Conflict(key, _) => get(key),
            AppError::NotFound(_, _) => get("auth.invalid_credentials"),
            AppError::Internal(_, _) => get("general.internal"),
        }
    }

    fn dev_message(&self) -> &'static str {
        match self {
            AppError::Unauthorized(key, _) => get(key),
            AppError::MethodNotAllowed(key, _) => get(key),
            AppError::BadRequest(key, _) => get(key),
            AppError::NotFound(key, _) => get(key),
            AppError::Conflict(key, _) => get(key),
            AppError::Internal(key, _) => get(key),
        }
    }

    pub fn detail(&self) -> Option<String> {
        if !is_dev() {
            return None;
        }

        Some(match self {
            AppError::Unauthorized(key, _) => format!("[unauthorized] key={}", key),
            AppError::MethodNotAllowed(key, _) => format!("[method_not_allowed] key={}", key),
            AppError::BadRequest(key, _) => format!("[bad_request] key={}", key),
            AppError::Conflict(key, _) => format!("[conflict] key={}", key),
            AppError::NotFound(key, _) => format!("[not_found] key={}", key),
            AppError::Internal(key, _) => format!("[internal] key={}", key),
        })
    }

    pub fn error_message(&self) -> Option<String> {
        if !is_dev() {
            return None;
        }

        Some(match self {
            AppError::Unauthorized(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
            AppError::MethodNotAllowed(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
            AppError::BadRequest(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
            AppError::Conflict(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
            AppError::NotFound(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
            AppError::Internal(_, e) => e.clone().unwrap_or_else(|| "No error message provided".to_string()),
        })
    }

    pub fn trace(&self) -> Option<String> {
        if !is_dev() {
            return None;
        }

        let backtrace = Backtrace::capture();
        Some(format!("{}", backtrace))
    }
}