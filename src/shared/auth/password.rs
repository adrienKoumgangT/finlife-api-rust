use argon2::{
    password_hash::{self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use rand::rngs::OsRng;
use rand::seq::SliceRandom;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("password hashing failed")]
    HashFailed,
    #[error("invalid password hash format")]
    InvalidHashFormat,
    #[error("password verification error")]
    VerifyError,
}

/// Generates a secure password with Special Characters.
///
/// Includes: A-Z, a-z, 0-9, and symbols (!@#$%^&*...)
/// Recommended length: 32+ for API keys, 12+ for user passwords.
pub fn generate_password(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789\
                            !@#$%^&*()_+-=[]{}|;:,.<>?";

    // Create a random number generator
    let mut rng = OsRng;

    // 1. Create an iterator of length `len`
    // 2. For every iteration, pick a random byte from CHARSET
    // 3. Convert to char and collect into String
    (0..len)
        .map(|_| *CHARSET.choose(&mut rng).expect("charset not empty"))
        .map(|b| b as char)
        .collect()
}

/// Centralized Argon2 configuration.
fn argon2() -> Argon2<'static> {
    // memory: 19 MiB, time: 2, lanes: 1
    let params = Params::new(19_456, 2, 1, None).expect("invalid argon2 params");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hash a plain password into a PHC string.
///
/// Accepts `impl AsRef<[u8]>` so you can pass `Zeroizing<Vec<u8>>` or `SecretString`
/// without forcing a conversion to a primitive `&str`.
pub fn hash_password(plain: impl AsRef<[u8]>) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    let a2 = argon2();

    a2.hash_password(plain.as_ref(), &salt)
        .map(|phc| phc.to_string())
        .map_err(|_| PasswordError::HashFailed)
}

/// Verify a password against a stored PHC string.
pub fn verify_password(plain: impl AsRef<[u8]>, stored_phc: &str) -> Result<bool, PasswordError> {
    let parsed = PasswordHash::new(stored_phc).map_err(|_| PasswordError::InvalidHashFormat)?;
    let a2 = argon2();

    match a2.verify_password(plain.as_ref(), &parsed) {
        Ok(_) => Ok(true),
        // This specific error means "password mismatch". We return Ok(false).
        Err(password_hash::Error::Password) => Ok(false),
        // Any other error (e.g. AlgorithmMismatch, version issues) is a system failure.
        Err(_) => Err(PasswordError::VerifyError),
    }
}
