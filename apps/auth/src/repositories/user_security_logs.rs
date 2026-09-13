use crate::{errors::AppError, models::db::{User, UserSecurityLog, UserUuid}};

pub async fn create_user_security_log<'a, E>(executor: E, state: &User) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> 
{
    sqlx::query(
        r#"
        INSERT INTO user_security_logs (user_uuid)
        VALUES ($1)
        "#,
    )
        .bind(state.uuid)
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("user.not_found", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    Ok(())
}

pub async fn get_user_security_log_from_uuid<'a, E>(executor: E, state: &UserUuid) -> Result<UserSecurityLog, AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let user_security_log = sqlx::query_as::<_, UserSecurityLog>(
        r#"
        SELECT *
        FROM user_security_logs
        WHERE user_uuid = $1
        "#,
    )
        .bind(state)
        .fetch_one(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("user.not_found", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    Ok(user_security_log)
}

pub async fn update_user_security_log<'a, E>(executor: E, state: &UserSecurityLog) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let result = sqlx::query(
        r#"
        UPDATE user_security_logs
        SET recent_failed_login_attempts = $1,
            total_successful_login_attempts = $2,
            total_failed_login_attempts = $3,
            lockout_until = $4,
            last_successful_login_at = $5,
            last_failed_login_at = $6,
            last_password_change_at = $7,
            email_verified_at = $8,
            last_successful_login_ip = $9,
            last_failed_login_ip = $10,
            last_successful_login_user_agent = $11,
            last_failed_login_user_agent = $12
        WHERE user_uuid = $13
        "#,
    )
        .bind(state.recent_failed_login_attempts)
        .bind(state.total_successful_login_attempts)
        .bind(state.total_failed_login_attempts)
        .bind(state.lockout_until)
        .bind(state.last_successful_login_at)
        .bind(state.last_failed_login_at)
        .bind(state.last_password_change_at)
        .bind(state.email_verified_at)
        .bind(state.last_successful_login_ip.clone())
        .bind(state.last_failed_login_ip.clone())
        .bind(state.last_successful_login_user_agent.clone())
        .bind(state.last_failed_login_user_agent.clone())
        .bind(state.user_uuid)
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("user.not_found", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("user.not_found", None));
    }

    Ok(())
}