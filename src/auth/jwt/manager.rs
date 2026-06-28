use super::model::Claims;
use crate::errors::ServiceError;
use crate::models::user::model::SlimUser;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::fs;

// Initialize JWT keys once on first access.
// Keys are loaded from paths defined in environment variables or CLI arguments.
lazy_static::lazy_static! {
    static ref JWT_KEYS: JwtKeys = {
        let opt = {
            use structopt::StructOpt;
            crate::cli_args::Opt::from_args()
        };
        JwtKeys::new(&opt.jwt_private_key, &opt.jwt_public_key)
    };
}

/// JWT signing and verification keys loaded from PEM files
pub(crate) struct JwtKeys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl JwtKeys {
    /// Creates JwtKeys by reading RSA key pair from PEM files
    fn new(private_path: &str, public_path: &str) -> Self {
        let private_key = fs::read(private_path)
            .unwrap_or_else(|_| panic!("Failed to read private key from {}", private_path));
        let public_key = fs::read(public_path)
            .unwrap_or_else(|_| panic!("Failed to read public key from {}", public_path));

        Self {
            encoding: EncodingKey::from_rsa_pem(&private_key).expect("Invalid RSA private key"),
            decoding: DecodingKey::from_rsa_pem(&public_key).expect("Invalid RSA public key"),
        }
    }
}

/// Creates a new JWT token for the user using global static keys
pub(crate) fn create_token(
    user: &SlimUser,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, String::from("CADBase"), auth_duration_in_hour);
    encode(&Header::new(Algorithm::RS256), &claims, &JWT_KEYS.encoding)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

/// Decodes and validates a JWT token using global static keys
pub(crate) fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        &JWT_KEYS.decoding,
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
