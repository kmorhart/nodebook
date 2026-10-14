use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use axum_extra::extract::CookieJar;
use crate::AppState;
use crate::errors::AppError;
use crate::models::db::UserUuid;
use crate::models::dto::{ ApiResponse, LoginRequest, RegisterRequest, ValidatedJson };
use crate::models::domain::{ ClaimsPair, TokenPairPublic, UserPublic };
use crate::services::auth::AuthService;
use crate::services::mail::MailService;
use crate::util::cookies::{create_cookies};

pub async fn root() -> &'static str {
    "Auth service is running, please specify a route to access the service."
}

pub async fn health_handler() -> &'static str {
    "running"
}

pub async fn register_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    ValidatedJson(payload): ValidatedJson<RegisterRequest>,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<UserPublic>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::register(app_state.db, headers, payload).await {
        Ok(data) => {
            let verification_token = AuthService::generate_verify_token(app_state.cache, data.user.uuid).await
                .map_err(|e| (e.status(), Json(ApiResponse::err(&e))))?;

            let email_to_send = data.user.email.0.clone();
            let token_to_send = verification_token.token.to_string();

            tokio::spawn(async move {
                MailService::send_verification(&email_to_send, &token_to_send)
            });

            let updated_jar = create_cookies(jar, data.token_pair).await;
            
            Ok((updated_jar, (StatusCode::CREATED, Json(ApiResponse::ok("User registered successfully".to_string(), data.user)))))
        },
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn verify_handler(
    Path(token): Path<String>,
    State(app_state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<UserPublic>>), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::verify(app_state.db, app_state.cache, token).await {
        Ok(user) => Ok((StatusCode::OK, Json(ApiResponse::ok("User verified successfully".to_string(), user)))),
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn login_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<UserPublic>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::login(app_state.db, headers, payload).await {
        Ok(data) => {
            let updated_jar = create_cookies(jar, data.token_pair).await;
            
            Ok((updated_jar, (StatusCode::CREATED, Json(ApiResponse::ok("User logged in successfully".to_string(), data.user)))))
        },
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn logout_handler(
    State(app_state): State<AppState>,
    Extension(claims_pair): Extension<ClaimsPair>
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::logout(app_state.db, app_state.cache, claims_pair).await {
        Ok(true) => Ok((StatusCode::OK, Json(ApiResponse::ok("User logged out successfully".to_string(), ())))),
        Ok(false) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&AppError::Internal("general.internal", Some("Failed to invalidate access token".to_string())))))),
        Err(app_error) => Err((app_error.status(), Json(ApiResponse::err(&app_error)))),
    }
}

pub async fn refresh_handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
) -> Result<(CookieJar, (StatusCode, Json<ApiResponse<()>>)), (StatusCode, Json<ApiResponse<()>>)> {
    match AuthService::refresh(app_state.db, headers).await {
        Ok(data) => {
            let token_pair_public: TokenPairPublic = data.clone().into();
            let updated_jar = create_cookies(jar, token_pair_public).await;

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

pub async fn session_handler(
    Extension(user_uuid): Extension<UserUuid>
) -> Result<(StatusCode, Json<ApiResponse<UserUuid>>), (StatusCode, Json<ApiResponse<()>>)> {
    Ok((StatusCode::OK, Json(ApiResponse::ok("User session retrieved successfully".to_string(), user_uuid))))
}