use chrono::DateTime;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::domain::{Claims, ClaimsPair, Token};


pub async fn store_refresh_token<'a, E>(executor: E, state: Token) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let result = sqlx::query(
        r#"
        INSERT INTO refresh_tokens (uuid, subject, token, issued_at, expires_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
        .bind(state.uuid)
        .bind(state.subject)
        .bind(state.token)
        .bind(state.issued_at)
        .bind(state.expires_at)
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("auth.token_invalid", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("auth.token_invalid", None));
    }

    Ok(())
}

pub async fn use_refresh_token<'a, E>(executor: E, state: &Claims) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let result = sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET used = true, used_at = NOW()
        WHERE uuid = $1 AND used = false AND revoked = false AND expires_at > NOW()
        "#,
    )
        .bind(state.jti)
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("auth.token_invalid", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("auth.token_invalid", None));
    }

    Ok(())
}

pub async fn revoke_refresh_token<'a, E>(executor: E, state: Uuid) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let result = sqlx::query(
        r#"
        UPDATE refresh_tokens
        SET revoked = true, revoked_at = NOW()
        WHERE uuid = $1 AND revoked = false AND expires_at > NOW()
        "#,
    )
        .bind(state)
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("auth.token_invalid", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("auth.token_invalid", None));
    }

    Ok(())
}

pub async fn revoke_refresh_token_special<'a, E>(executor: E, state: ClaimsPair) -> Result<(), AppError>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>
{
    let result = sqlx::query(
        r#"
        UPDATE refresh_tokens 
        SET revoked = true, revoked_at = NOW() 
        WHERE subject = $1 
        AND revoked = false 
        AND expires_at > NOW() 
        AND ABS(EXTRACT(EPOCH FROM (issued_at - $2))) <= 1;
        "#
    )
        .bind(state.access_token_claims.as_ref().unwrap().sub)
        .bind(DateTime::from_timestamp(state.access_token_claims.as_ref().unwrap().iat as i64, 0).unwrap())
        .execute(executor)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => AppError::NotFound("auth.token_invalid", Some(e.to_string())),
            _ => AppError::Internal("general.internal", Some(e.to_string())),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("auth.token_invalid", None));
    }

    Ok(())
}