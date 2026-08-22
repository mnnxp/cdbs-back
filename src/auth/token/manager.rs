use super::model::Claims;
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceError;
use crate::models::user::model::SlimUser;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::sync::OnceLock;

/// Global static storage for JWT keys
static JWT_KEYS: OnceLock<JwtKeys> = OnceLock::new();

pub(crate) fn init_jwt_keys(private_key: &str, public_key: &str) -> Result<(), Error> {
    let keys = JwtKeys::new(private_key, public_key)?;
    let _ = JWT_KEYS.set(keys);
    Ok(())
}

/// JWT signing and verification keys loaded from env
struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtKeys {
    /// Creates a new JwtKeys instance from RSA PEM strings
    fn new(private_key: &str, public_key: &str) -> Result<Self, Error> {
        let encoding = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
        let decoding = DecodingKey::from_rsa_pem(public_key.as_bytes())?;
        Ok(Self { encoding, decoding })
    }
}

/// Creates a new JWT token for the user using global static keys
pub(crate) fn create_token(
    user: &SlimUser,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, String::from("CADBase"), auth_duration_in_hour);
    let keys = JWT_KEYS.get().expect("JWT keys are not initialized.");
    encode(&Header::new(Algorithm::RS256), &claims, &keys.encoding)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

/// Decodes and validates a JWT token using global static keys
pub(crate) fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    let keys = JWT_KEYS.get().expect("JWT keys are not initialized.");
    decode::<Claims>(token, &keys.decoding, &Validation::new(Algorithm::RS256))
        .map(|data| data.claims)
        .map_err(|err| {
            debug!("Failed to decode token: {:?}", err);
            match err.kind() {
                ErrorKind::ExpiredSignature => ServiceError::Unauthorized,
                _ => get_err_msg(ErrorMessage::TokenIsInvalid),
            }
        })
}
