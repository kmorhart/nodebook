use axum::{
    Json, body::Body, extract::State, http::{Request, Response, StatusCode}, middleware::Next, response::IntoResponse,
};
use axum_extra::extract::{CookieJar, cookie::{Cookie, SameSite}};
use deadpool_redis::redis::cmd;

use crate::{AppState, errors::AppError, models::dto::ApiResponse, services::auth::AuthService, util::tokens::verify_access_token};


const PUBLIC_ROUTES: &[&str] = &["/", "/register", "/login", "/refresh"];

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

            let access_cookie = Cookie::build(("access_token", token_pair.access_token.token.clone()))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/")
                .build();

            let refresh_cookie = Cookie::build(("refresh_token", token_pair.refresh_token.token.clone()))
                .http_only(true)
                .secure(true)
                .same_site(SameSite::Strict)
                .path("/refresh")
                .build();

            if path == "/logout" {
                let updated_jar = jar.clone().remove(Cookie::build(("access_token", "")).build()).remove(Cookie::build(("refresh_token", "")).build());
                request.extensions_mut().insert(token_pair.refresh_token.uuid);
                let response = next.run(request).await;
                return Ok((updated_jar, response).into_response());
            }

            request.extensions_mut().insert(token_pair.refresh_token.subject);
            request.extensions_mut().insert(token_pair.refresh_token.uuid);
            
            let updated_jar = jar.clone().add(access_cookie).add(refresh_cookie);
            let response = next.run(request).await;
            
            Ok((updated_jar, response).into_response())
        }
    }
}
