use std::net::IpAddr;
use std::str::FromStr;

use crate::errors::AppError;
use crate::models::db::{User, UserUuid};
use crate::models::domain::{AuthUser, AuthUserPublic, TokenPair, UserIdentifier, UserPublic, UserRegistration};
use crate::models::dto::{LoginRequest, RegisterRequest};
use crate::repositories::refresh_tokens::{store_refresh_token, use_refresh_token};
use crate::repositories::user_security_logs::{create_user_security_log, get_user_security_log_from_uuid, update_user_security_log};
use crate::util::crypto::{hash_password, verify_password};
use crate::util::tokens::{generate_tokens, verify_refresh_token};
use crate::repositories::users::{create_user, get_password_hash_from_uuid, get_user_from_uuid, get_uuid_from_identity};

use sqlx::PgPool;
use axum::http::{HeaderMap};

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: PgPool, _headers: HeaderMap, payload: RegisterRequest) -> Result<AuthUserPublic, AppError> {
        let mut tx = pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
        
        let password = payload.password.clone();
        let password_hash = hash_password(&password)
            .await?;

        let user_registration = UserRegistration {
            email: payload.email,
            username: payload.username,
            password_hash: password_hash,
        };

        let user: User = create_user(&mut *tx, user_registration)
            .await?;

        create_user_security_log(&mut *tx, &user)
            .await?;

        let token_pair = generate_tokens(user.uuid)
            .await?;

        store_refresh_token(&mut *tx, token_pair.refresh_token.clone())
            .await?;

        let auth_user_public: AuthUserPublic = AuthUser {
            user,
            token_pair
        }.into();

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
        Ok(auth_user_public)
    }

    pub async fn login(pool: PgPool, headers: HeaderMap, payload: LoginRequest) -> Result<AuthUserPublic, AppError> {
        let mut tx = pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        let user_identity = UserIdentifier (payload.identifier.clone());

        let user_uuid = get_uuid_from_identity(&mut *tx, &user_identity)
            .await?;

        let mut user_security_log = get_user_security_log_from_uuid(&mut *tx, &user_uuid)
            .await?;

        if user_security_log.lockout_until.is_some() && user_security_log.lockout_until.unwrap() > chrono::Utc::now() {
            user_security_log.total_failed_login_attempts += 1;
            user_security_log.recent_failed_login_attempts += 1;
            user_security_log.lockout_until = Some(chrono::Utc::now() + chrono::Duration::minutes(15));
            user_security_log.total_failed_login_attempts += 1;
            user_security_log.last_failed_login_at = Some(chrono::Utc::now());
            user_security_log.last_failed_login_ip = IpAddr::from_str(&headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| "0.0.0.0".into())).ok();
            user_security_log.last_failed_login_user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
            
            update_user_security_log(&mut *tx, &user_security_log)
                .await?;
            tx.commit().await
                .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
            return Err(AppError::Unauthorized("auth.lockout", None));
        }

        if user_security_log.last_failed_login_at.is_some() && user_security_log.last_failed_login_at.unwrap() + chrono::Duration::hours(1) < chrono::Utc::now() {
            user_security_log.recent_failed_login_attempts = 0;
            user_security_log.lockout_until = None;
            update_user_security_log(&mut *tx, &user_security_log)
                .await?;
        }

        let password = payload.password.clone();
        let existing_password_hash = get_password_hash_from_uuid(&mut *tx, &user_uuid)
            .await?;

        let correct = verify_password(&password, &existing_password_hash.0)
            .await?;

        if !correct {
            user_security_log.total_failed_login_attempts += 1;
            user_security_log.recent_failed_login_attempts += 1;
            user_security_log.total_failed_login_attempts += 1;
            user_security_log.last_failed_login_at = Some(chrono::Utc::now());
            user_security_log.last_failed_login_ip = IpAddr::from_str(&headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| "0.0.0.0".into())).ok();
            user_security_log.last_failed_login_user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());

            if user_security_log.recent_failed_login_attempts >= 5 {
                user_security_log.lockout_until = Some(chrono::Utc::now() + chrono::Duration::minutes(15));
            }

            update_user_security_log(&mut *tx, &user_security_log)
                .await?;
            tx.commit().await
                .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
            return Err(AppError::Unauthorized("auth.invalid_credentials", None));
        }

        user_security_log.total_successful_login_attempts += 1;
        user_security_log.recent_failed_login_attempts = 0;
        user_security_log.lockout_until = None;
        user_security_log.last_successful_login_at = Some(chrono::Utc::now());
        user_security_log.last_successful_login_ip = IpAddr::from_str(&headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()).map(|s| s.to_string()).unwrap_or_else(|| "0.0.0.0".into())).ok();
        user_security_log.last_successful_login_user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
        
        update_user_security_log(&mut *tx, &user_security_log)
            .await?;

        let token_pair = generate_tokens(user_uuid)
        .await?;

        store_refresh_token(&mut *tx, token_pair.refresh_token.clone())
            .await?;

        let user: User = get_user_from_uuid(&mut *tx, &user_uuid)
            .await?;

        let auth_user_public: AuthUserPublic = AuthUser {
            user,
            token_pair
        }.into();

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
        Ok(auth_user_public)
    }

    pub async fn refresh(pool: PgPool, headers: HeaderMap) -> Result<TokenPair, AppError> {
        let mut tx = pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        let refresh_token = headers.get("cookie")
            .and_then(|cookie_header| cookie_header.to_str().ok())
            .and_then(|cookie_str| cookie_str.split(';')
                .find_map(|cookie| {
                    let cookie = cookie.trim();
                    if cookie.starts_with("refresh_token=") {
                        Some(cookie.trim_start_matches("refresh_token=").to_string())
                    } else {
                        None
                    }
                })
            )
            .ok_or_else(|| AppError::Unauthorized("auth.token_missing", None))?;

        let claims = verify_refresh_token(&refresh_token)?;
        use_refresh_token(&mut *tx, &claims)
        .await?;
    
        let user_uuid = claims.sub;

        let token_pair = generate_tokens(user_uuid)
            .await?;

        store_refresh_token(&mut *tx, token_pair.refresh_token.clone())
            .await?;

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        
        Ok(token_pair)
    }

    pub async fn me(pool: PgPool, _headers: HeaderMap, user_uuid: UserUuid) -> Result<UserPublic, AppError> {
        let mut tx = pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        let user: User = get_user_from_uuid(&mut *tx, &user_uuid)
            .await?;

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        Ok(user.into())
    }
}