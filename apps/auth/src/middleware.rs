use axum::{
    Json, body::Body, extract::State, http::{Request, Response, StatusCode}, middleware::Next, response::IntoResponse,
};
use axum_extra::extract::{CookieJar};
use redis::AsyncCommands;

use crate::{AppState, errors::AppError, models::{domain::{ClaimsPair, TokenPairPublic}, dto::ApiResponse}, services::auth::AuthService, util::{cookies::{create_cookies, remove_cookies}, tokens::{verify_access_token, verify_refresh_token}}};


const PUBLIC_ROUTES: &[&str] = &["/", "/health", "/register", "/verify", "/login", "/refresh"];

pub async fn auth_middleware(
    State(app_state): State<AppState>,
    jar: CookieJar,
    mut request: Request<Body>,
    next: Next
) -> Result<Response<Body>, (StatusCode, Json<ApiResponse<()>>)> {
    let path = request.uri().path();

    if PUBLIC_ROUTES.iter().any(|r| *r == path) {
        return Ok(next.run(request).await);
    }

    let access_token = jar
        .get("access_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    let refresh_token = jar
        .get("refresh_token")
        .map(|c| c.value().to_string())
        .unwrap_or_default();

    let access_token_claims = verify_access_token(&access_token).ok();
    let refresh_token_claims = verify_refresh_token(&refresh_token).ok();

    if access_token_claims.is_none() && refresh_token_claims.is_none() {
        return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.token_missing", None)))));
    } else if path == "/logout" {
        let updated_jar = remove_cookies(jar).await;

        request.extensions_mut().insert(ClaimsPair {
            access_token_claims,
            refresh_token_claims,
        });

        let response = next.run(request).await;

        return Ok((updated_jar, response).into_response())
    } else if access_token_claims.is_some() {
        let mut connection = app_state.cache.get().await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&AppError::Internal("general.internal", Some(e.to_string()))))))?;

        let exists = connection.exists::<_, bool>(format!("invalid_token:{}", access_token_claims.as_ref().unwrap().jti)).await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&AppError::Internal("general.internal", Some(e.to_string()))))))?;
        
        if exists {
            return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.token_invalid", None)))));
        }

        request.extensions_mut().insert(access_token_claims.as_ref().unwrap().sub);

        let response = next.run(request).await;
        
        return Ok((response).into_response())
    } else {
        let token_pair =  AuthService::refresh(app_state.db, request.headers().clone())
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.invalid_credentials", None)))))?;

        let updated_jar = create_cookies(jar, TokenPairPublic{..token_pair.clone().into()}).await;

        request.extensions_mut().insert(token_pair.refresh_token.subject);

        let response = next.run(request).await;
        
        return Ok((updated_jar, response).into_response())
    }
}
