use crate::errors::AppError;
use crate::models::db::{ PasswordHash, User, UserUuid };
use crate::models::domain::{UserIdentifier, UserRegistration};


pub async fn create_user<'a, E>(executor: E, state: UserRegistration) -> Result<User, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING uuid, username, email, password_hash, role, is_active, is_verified, created_at, updated_at
        "#,
    )
        .bind(state.email)
        .bind(state.username)
        .bind(state.password_hash)
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e {
                match db_err.constraint() {
                    Some("users_email_key") => return AppError::Conflict("user.email_taken", Some(e.to_string())),
                    Some("users_username_key") => return AppError::Conflict("user.username_taken", Some(e.to_string())),
                    _ => {}
                }
            }
            AppError::Internal("general.internal", Some(e.to_string()))
        })?;

    match user {
        Some(user) => Ok(user),
        None => Err(AppError::NotFound("general.internal", None)),
    }
}

pub async fn get_uuid_from_identity<'a, E>(executor: E, state: &UserIdentifier) -> Result<UserUuid, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let uuid = sqlx::query_scalar::<_, UserUuid>(
        r#"
        SELECT uuid
        FROM users
        WHERE email = $1 OR username = $1
        "#,
    )
        .bind(state)
        .fetch_optional(executor)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                return AppError::NotFound("user.not_found", Some(e.to_string()));
            }
            AppError::Internal("general.internal", Some(e.to_string()))
        })?;

    match uuid {
        Some(uuid) => Ok(uuid),
        None => Err(AppError::NotFound("user.not_found", None)),
    }
}

pub async fn get_password_hash_from_uuid<'a, E>(executor: E, state: &UserUuid) -> Result<PasswordHash, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let password_hash = sqlx::query_scalar::<_, PasswordHash>(
        r#"
        SELECT password_hash
        FROM users
        WHERE uuid = $1
        "#,
    )
        .bind(state)
        .fetch_one(executor)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                return AppError::NotFound("user.not_found", Some(e.to_string()));
            }
            AppError::Internal("general.internal", Some(e.to_string()))
        })?;

    Ok(password_hash)
}

pub async fn get_user_from_uuid<'a, E>(executor: E, state: &UserUuid) -> Result<User, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT uuid, username, email, password_hash, role, is_active, is_verified, created_at, updated_at
        FROM users
        WHERE uuid = $1
        "#,
    )
        .bind(state)
        .fetch_one(executor)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                return AppError::NotFound("user.not_found", Some(e.to_string()));
            }
            AppError::Internal("general.internal", Some(e.to_string()))
        })?;

    Ok(user)
}