use axum::{
    Json, body::Body, extract::State, http::{Request, Response, StatusCode}, middleware::Next, response::IntoResponse,
};
use axum_extra::extract::{CookieJar};
use deadpool_redis::redis::cmd;

use crate::{AppState, errors::AppError, models::{domain::TokenPairPublic, dto::ApiResponse}, services::auth::AuthService, util::{cookies::{create_cookies, remove_cookies}, tokens::verify_access_token}};


const PUBLIC_ROUTES: &[&str] = &["/", "/health", "/register", "/login", "/refresh"];

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
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.invalid_credentials", None)))))?;
    
    match verify_access_token(&access_token) {
        Ok(claims) => {
            let mut connection = app_state.cache.get().await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse::err(&AppError::Internal("general.internal", Some(e.to_string()))))))?;
            let invalid: u8 = cmd("EXISTS")
                .arg(format!("access_token/{}", claims.jti))
                .query_async(&mut connection)
                .await
                .unwrap();

            if invalid == 1 {
                return Err((StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.token_invalid", None)))));
            }

            request.extensions_mut().insert(claims.sub);
            request.extensions_mut().insert(claims.jti);
            Ok(next.run(request).await)
        },
        Err(_) => {
            let token_pair =  AuthService::refresh(app_state.db, request.headers().clone())
                .await
                .map_err(|_| (StatusCode::UNAUTHORIZED, Json(ApiResponse::err(&AppError::Unauthorized("auth.invalid_credentials", None)))))?;

            if path == "/logout" {
                let updated_jar = remove_cookies(jar).await;
                request.extensions_mut().insert(token_pair.refresh_token.uuid);
                let response = next.run(request).await;
                return Ok((updated_jar, response).into_response());
            }

            let updated_jar = create_cookies(jar, TokenPairPublic{..token_pair.clone().into()}).await;

            request.extensions_mut().insert(token_pair.refresh_token.subject);
            request.extensions_mut().insert(token_pair.refresh_token.uuid);

            let response = next.run(request).await;
            
            Ok((updated_jar, response).into_response())
        }
    }
}
