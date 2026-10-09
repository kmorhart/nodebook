use axum::{Json, extract::{FromRequest, Request}, http::StatusCode};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_with::skip_serializing_none;
use validator::Validate;

use crate::errors::AppError;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request(req: Request, state: &S) -> Result<Self, (StatusCode, Json<ApiResponse<()>>)> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| 
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::<()>::err(&AppError::BadRequest("Invalid JSON", Some(e.to_string())))),
                ))?;

        value.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::err(&AppError::BadRequest("Validation failed", Some(e.to_string())))),
            )
        })?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Must be a valid email address"), length(min = 5, max = 254))]
    pub email: String,
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub message: String,
    pub data: Option<T>,
    pub error: Option<ErrorDetail>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct ErrorDetail {
    pub detail: Option<String>,
    pub error_message: Option<String>,
    pub trace: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(message: String, data: T) -> Self {
        Self {
            message: message,
            data: Some(data),
            error: None,
        }
    }
    pub fn err(app_error: &AppError) -> Self {
        let detail = app_error.detail();
        let error_message = app_error.error_message();
        let trace = app_error.trace();

        let error = if detail.is_none() && trace.is_none() {
            None
        } else {
            Some(ErrorDetail { detail, error_message, trace })
        };
        
        Self {
            message: app_error.message().to_string(),
            data: None,
            error: error,
        }
    }
}