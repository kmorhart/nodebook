use crate::errors::AppError;
use crate::models::db::{ UserProfile };
use crate::models::domain::{ UserRegistration };


pub async fn create_user_profile<'a, E>(executor: E, state: UserRegistration) -> Result<UserProfile, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let user = sqlx::query_as::<_, UserProfile>(
        r#"
        INSERT INTO user_profile (user_uuid, username)
        VALUES ($1, $2)
        RETURNING ref_uuid, user_uuid, username, firstname, lastname, bio
        "#,
    )
        .bind(state.uuid)
        .bind(state.username)
        .fetch_one(executor)
        .await
        .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(user)
}

pub async fn update_user_profile<'a, E>(executor: E, state: UserRegistration) -> Result<UserProfile, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let user = sqlx::query_as::<_, UserProfile>(
        r#"
        UPDATE users_profile
        SET username = $2
        WHERE user_uuid = $1
        RETURNING ref_uuid, user_uuid, username, firstname, lastname, bio
        "#,
    )
        .bind(state.uuid)
        .bind(state.username)
        .fetch_one(executor)
        .await
        .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(user)
}