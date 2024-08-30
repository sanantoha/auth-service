use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::error::Error;

// Hash a password
pub fn hash_password(password: &str) -> Result<String, Error> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Use the default Argon2 algorithm
    let argon2 = Argon2::default();
    
    // Hash the password
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
                .map_err(|e| Error::PasswordHash(format!("error hash password {}", e)))?;

    // Return the hash as a string
    Ok(password_hash.to_string())
}

// Verify a password
pub fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| Error::PasswordHash(format!("error verify password {}", e)))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}