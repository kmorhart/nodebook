use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::db::{ EmailAddress, PasswordHash, User, UserUuid }; 
use serde::{ Serialize, Deserialize };



// Internal structs for actions describing user intent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistration {
    pub email: String,
    pub username: String,
    pub password_hash: PasswordHash,
}

// pub struct UserLogin {
//     pub identifier: String,
//     pub password_hash: PasswordHash,
// }

pub struct UsernameAvailable {
    pub username: String,
}

#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize)]
#[sqlx(transparent)]
pub struct UserIdentifier (pub String);


// Internal structs for data representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub user: User,
    pub token_pair: TokenPair,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: Token,
    pub refresh_token: Token,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub uuid: Uuid,
    pub subject: UserUuid,
    pub token: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

// Public-facing structs for data representation, with fields omitted for API responses

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthUserPublic {
    pub user: UserPublic,
    pub token_pair: TokenPairPublic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub uuid: UserUuid,
    pub username: String,
    pub email: EmailAddress,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPairPublic {
    pub access_token: TokenPublic,
    pub refresh_token: TokenPublic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPublic {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub issued_at: DateTime<Utc>,
}

// Implementations for converting internal structs to public-facing structs

impl From<AuthUser> for AuthUserPublic {
    fn from(internal: AuthUser) -> Self {
        Self {
            user: internal.user.into(),
            token_pair: internal.token_pair.into(),
        }
    }
}

impl From<User> for UserPublic {
    fn from(internal: User) -> Self {
        Self {
            uuid: internal.uuid,
            username: internal.username,
            email: internal.email,
            is_active: internal.is_active,
            is_verified: internal.is_verified,
            created_at: internal.created_at,
        }
    }
}

impl From<TokenPair> for TokenPairPublic {
    fn from(internal: TokenPair) -> Self {
        Self {
            access_token: internal.access_token.into(),
            refresh_token: internal.refresh_token.into(),
        }
    }
}

impl From<Token> for TokenPublic {
    fn from(internal: Token) -> Self {
        Self {
            token: internal.token,
            expires_at: internal.expires_at,
            issued_at: internal.issued_at,
        }
    }
}
