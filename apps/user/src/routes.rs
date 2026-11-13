// use axum::extract::{Path, State};
// use axum::{Extension, Json};
// use axum::http::{HeaderMap, StatusCode};
// use axum_extra::extract::CookieJar;
// use crate::AppState;
// use crate::errors::AppError;
// use crate::models::db::{UserUuid};
// use crate::models::domain::UserProfilePublic;
// use crate::models::dto::{ ApiResponse, ValidatedJson };

// pub async fn root() -> &'static str {
//     "User service is running, please specify a route to access the service."
// }

// pub async fn health_handler() -> &'static str {
//     "running"
// }

// // pub async fn get_handler(
// //     State(app_state): State<AppState>,
// //     jar: CookieJar,
// //     headers: HeaderMap,
// //     ValidatedJson(payload): ValidatedJson<GetRequest>,
// // ) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<UserProfilePublic>>)), (StatusCode, Json<ApiResponse<()>>)> {
    
// // }

// pub async fn me_handler(
//     State(app_state): State<AppState>,
//     headers: HeaderMap,
//     Extension(user_uuid): Extension<UserUuid>
// ) -> Result<(StatusCode, Json<ApiResponse<UserProfilePublic>>), (StatusCode, Json<ApiResponse<()>>)> {
    
// }