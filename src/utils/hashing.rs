use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Algorithm, Version, Params,
};
use crate::error::AppError;

// Optimized Argon2 configuration for web applications
// Tuned for fast response times while maintaining security
fn get_argon2() -> Argon2<'static> {
    // Recommended parameters per OWASP Password Storage Cheat Sheet (2025):
    // - m_cost (memory): 19 MiB (19456 KiB) - minimum for Argon2id
    // - t_cost (iterations): 2 - balanced security
    // - p_cost (parallelism): 1 - single thread
    // Expected: ~100-300ms per hash/verify (adjust based on hardware)
    let params = Params::new(
        19456,  // 19 MiB memory (OWASP minimum)
        2,      // 2 iterations
        1,      // 1 thread
        None    // output length (default)
    ).expect("Invalid Argon2 params");

    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let start = std::time::Instant::now();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = get_argon2();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?
        .to_string();

    tracing::debug!("Password hashing took {}ms", start.elapsed().as_millis());
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let start = std::time::Instant::now();
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::Internal(format!("Password hash parsing failed: {}", e)))?;

    let argon2 = get_argon2();
    
    let result = argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
    
    tracing::debug!("Password verification took {}ms", start.elapsed().as_millis());
    Ok(result)
}
