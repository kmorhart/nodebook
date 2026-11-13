use crate::errors::AppError;
use crate::models::db::{UserData, UserProfile};
use crate::models::domain::{UserRegistration};
use crate::models::dto::{CreateUserInternalRequest, UpdateUserInternalRequest};
use crate::repositories::user_data::create_user_data;
use crate::repositories::user_profile::{create_user_profile, update_user_profile};

use sqlx::PgPool;

pub struct UserService;

impl UserService {
    pub async fn create(postgres_pool: PgPool, payload: CreateUserInternalRequest) -> Result<UserData, AppError> {
        let mut tx = postgres_pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        let user_registration = UserRegistration {
            uuid: payload.uuid,
            username: payload.username,
        };

        let profile: UserProfile = create_user_profile(&mut *tx, user_registration)
            .await?;

        let data: UserData = create_user_data(&mut *tx, profile)
            .await?;

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
        Ok(data)
    }

    pub async fn update(postgres_pool: PgPool, payload: UpdateUserInternalRequest) -> Result<UserProfile, AppError> {
        let mut tx = postgres_pool.begin().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

        let user_registration = UserRegistration {
            uuid: payload.uuid,
            username: payload.username,
        };

        let profile: UserProfile = update_user_profile(&mut *tx, user_registration)
            .await?;

        tx.commit().await
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
        Ok(profile)
    }
}