use crate::errors::AppError;
use crate::models::db::{ UserData, UserProfile };


pub async fn create_user_data<'a, E>(executor: E, state: UserProfile) -> Result<UserData, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let data = sqlx::query_as::<_, UserData>(
        r#"
        INSERT INTO user_data (ref_uuid, user_uuid)
        VALUES ($1, $2)
        RETURNING ref_uuid, user_uuid, settings, workspaces
        "#,
    )
        .bind(state.ref_uuid)
        .bind(state.user_uuid)
        .fetch_one(executor)
        .await
        .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(data)
}