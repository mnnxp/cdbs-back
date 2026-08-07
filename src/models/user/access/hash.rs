use argon2::{self, Config, Variant, Version};
use rand::RngCore;

/// Standard cryptographic salt length (256-bit)
const SALT_LEN: usize = 32;

/// Generates a cryptographically secure random salt
pub(crate) fn make_salt() -> [u8; SALT_LEN] {
    let mut psw_salt = [0_u8; SALT_LEN];
    rand::thread_rng().fill_bytes(&mut psw_salt);
    psw_salt
}

/// Hashes a password using Argon2id with production-ready, resource-efficient parameters
pub(crate) fn make_hash_salt(password: &[u8], psw_salt: &[u8]) -> Vec<u8> {
    let config = Config {
        variant: Variant::Argon2id, // Defends against both GPU brute-force and side-channel attacks
        version: Version::Version13,
        mem_cost: 16384, // 16 MB memory overhead (optimal)
        time_cost: 2,    // 2 passes: quick execution (~15-30ms) to prevent DoS vectors
        lanes: 1,        // 1 thread: prevents thread pool exhaustion under concurrent login load
        ..Default::default()
    };

    argon2::hash_encoded(password, psw_salt, &config)
        .unwrap()
        .into_bytes()
}

/// Verifies a password against an encoded Argon2id hash string
pub(crate) fn verify(psw_hash: &[u8], password: &[u8]) -> bool {
    let hash_str = match std::str::from_utf8(psw_hash) {
        Ok(s) => s,
        Err(_) => return false,
    };
    argon2::verify_encoded(hash_str, password).unwrap_or(false)
}
