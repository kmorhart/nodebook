use axum::extract::State;
use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use axum_extra::extract::CookieJar;
use deadpool_redis::redis::cmd;
use uuid::Uuid;
use crate::AppState;
use crate::errors::AppError;
use crate::models::db::UserUuid;
use crate::models::dto::{ ApiResponse, LoginRequest, RegisterRequest };
use crate::models::domain::{ UserPublic };
use crate::services::auth::AuthService;
use crate::util::cookies::{create_cookies, create_refresh_cookie, remove_cookies};

pub async fn root() -> &'static str {
    "Auth service is running, please specify a route to access the service."
}

pub async fn register_handler(
    jar: CookieJar,
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<UserPublic>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::register(app_state.db, headers, payload).await {
        Ok(data) => {
            let updated_jar = create_cookies(jar, &data.token_pair).await;
            Ok((updated_jar, (StatusCode::CREATED, Json(ApiResponse::ok("User registered successfully".to_string(), data.user)))))
        },
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn login_handler(
    jar: CookieJar,
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<UserPublic>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::login(app_state.db, headers, payload).await {
        Ok(data) => {
            let updated_jar = create_cookies(jar, &data.token_pair).await;

            Ok((updated_jar, (StatusCode::CREATED, Json(ApiResponse::ok("User logged in successfully".to_string(), data.user)))))
        },
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn logout_handler(
    jar: CookieJar,
    State(app_state): State<AppState>,
    _headers: HeaderMap,
    Extension(jti): Extension<Uuid>
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<()>>)), (StatusCode, Json<ApiResponse<()>>)> {
    let updated_jar = remove_cookies(jar).await;

    let mut connection = app_state.cache.get().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&AppError::Internal("general.internal", Some(e.to_string()))))))?;

    cmd("SET")
        .arg(&[format!("access_token/{}", jti), "".to_string()])
        .query_async::<()>(&mut connection)
        .await.unwrap();

    Ok((updated_jar, (StatusCode::OK, Json(ApiResponse::ok("User logged out successfully".to_string(), ())))))
}

pub async fn refresh_handler(
    jar: CookieJar,
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<()>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::refresh(app_state.db, headers).await {
        Ok(data) => {
            let updated_jar = create_refresh_cookie(jar, &data.refresh_token).await;

            Ok((updated_jar, (StatusCode::CREATED, Json(ApiResponse::ok("Token refreshed successfully".to_string(), ())))))
        },
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn me_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Extension(user_uuid): Extension<UserUuid>
) -> Result<(StatusCode, Json<ApiResponse<UserPublic>>), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::me(app_state.db, headers, user_uuid).await {
        Ok(user) => Ok((StatusCode::OK, Json(ApiResponse::ok("User retrieved successfully".to_string(), user)))),
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}
