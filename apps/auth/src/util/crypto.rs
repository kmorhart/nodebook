use argon2::{ Argon2, PasswordHasher, PasswordVerifier, Params, Algorithm, Version };
use argon2::password_hash::{ SaltString };
use rand_core::OsRng;

use crate::errors::AppError;
use crate::models::db::PasswordHash;

pub async fn hash_password(password: &str) -> Result<PasswordHash, AppError> {
    let password_owned = password.to_string();
    
    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(19456, 2, 1, None).unwrap(),
            );
        let hash = argon2
            .hash_password(password_owned.as_bytes(), &salt)
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))
            .unwrap()
            .to_string();

        tx.send(hash).unwrap();
    });
    let hash = rx.await
        .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;

    Ok(PasswordHash(hash))
}

pub async fn verify_password(password: &str, existing_password_hash: &str) -> Result<bool, AppError> {    
    let password_owned = password.to_string();
    let existing_password_hash_owned = existing_password_hash.to_string();

    let (tx, rx) = oneshot::channel();

    rayon::spawn(move || {
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(19456, 2, 1, None).unwrap(),
        );

        let parsed_hash = argon2::PasswordHash::new(&existing_password_hash_owned)
            .map_err(|e| AppError::Internal("general.internal", Some(e.to_string()))).unwrap();

        let verification = argon2
            .verify_password(password_owned.as_bytes(), &parsed_hash)
            .is_ok();

        tx.send(verification).unwrap();
    });

    let correct = rx.await
        .map_err(|e| AppError::Internal("general.internal", Some(e.to_string())))?;
    
    Ok(correct)
}