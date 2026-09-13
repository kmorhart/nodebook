use crate::{errors::AppError, models::{db::{UserUuid}, domain::{Token, TokenPair}}};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::env;



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub jti: Uuid,
    pub sub: UserUuid,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

pub async fn generate_tokens(user_uuid: UserUuid) -> Result<TokenPair, AppError> {
    let access_token = generate_access_token(&user_uuid)?;

    let refresh_token = generate_refresh_token(&user_uuid)?;
    Ok(TokenPair { access_token, refresh_token })
}

pub fn generate_access_token(user_uuid: &UserUuid) -> Result<Token, AppError> {
    let secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");
    let now = Utc::now();
    let expiry = now + Duration::minutes(30);

    let claims = Claims {
        jti: Uuid::new_v4(),
        sub: user_uuid.clone(),
        exp: expiry.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(Token {
        uuid: Uuid::new_v4(),
        subject: user_uuid.clone(),
        token: token,
        issued_at: now,
        expires_at: expiry,
    })
}

pub fn generate_refresh_token(user_uuid: &UserUuid) -> Result<Token, AppError> {
    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    let now = Utc::now();
    let expiry = now + Duration::days(14);

    let claims = Claims {
        jti: Uuid::new_v4(),
        sub: user_uuid.clone(),
        exp: expiry.timestamp() as usize,
        iat: now.timestamp() as usize,
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
    .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(Token {
        uuid: claims.jti,
        subject: user_uuid.clone(),
        token: token,
        issued_at: now,
        expires_at: expiry,
    })
}

pub fn verify_access_token(token: &str) -> Result<Claims, AppError> {
    let secret = env::var("JWT_ACCESS_SECRET").expect("JWT_ACCESS_SECRET must be set");

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e|  AppError::Unauthorized("auth.token_invalid", Some(e.to_string())))?;
    Ok(data.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<Claims, AppError> {
    let secret = env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ).map_err(|e| AppError::Unauthorized("auth.token_invalid", Some(e.to_string())))?;
    Ok(data.claims)
}