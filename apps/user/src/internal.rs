use axum::extract::{State};
use axum::{Json};
use axum::http::{StatusCode};
use crate::AppState;
use crate::models::db::{UserData, UserProfile};
use crate::models::dto::{ ApiResponse, ValidatedJson, CreateUserInternalRequest, UpdateUserInternalRequest };
use crate::services::user::UserService;


#[axum::debug_handler]
pub async fn create_user_internal_handler(
    State(app_state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserInternalRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserData>>), (StatusCode, Json<ApiResponse<()>>)> {
    match UserService::create(app_state.db, payload).await {
        Ok(data) => Ok((StatusCode::CREATED, Json(ApiResponse::ok("User created successfully".to_string(), data)))),
        Err(app_error) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&app_error)))),
    }
}

#[axum::debug_handler]
pub async fn update_user_internal_handler(
    State(app_state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<UpdateUserInternalRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserProfile>>), (StatusCode, Json<ApiResponse<()>>)> {
    match UserService::update(app_state.db, payload).await {
        Ok(profile) => Ok((StatusCode::OK, Json(ApiResponse::ok("User updated successfully".to_string(), profile)))),
        Err(app_error) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&app_error)))),
    }
}