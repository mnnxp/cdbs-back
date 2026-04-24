use super::model::Claims;
use crate::errors::ServiceError;
use crate::models::user::model::SlimUser;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

lazy_static::lazy_static! {
    static ref ENCODE_KEY : EncodingKey = EncodingKey::from_rsa_pem(
        include_bytes!("../../../keys/rs256-4096-private.pem")
    ).expect("Encode private key failed!");
}

pub(crate) fn create_token(
    user: &SlimUser,
    auth_duration_in_hour: u16,
) -> Result<String, ServiceError> {
    let claims: Claims = Claims::new(user, String::from("CADBase"), auth_duration_in_hour);
    encode(&Header::new(Algorithm::RS256), &claims, &ENCODE_KEY)
        .map_err(|e| ServiceError::BadRequest(e.to_string()))
}

pub(crate) fn decode_token(token: &str) -> Result<Claims, ServiceError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_rsa_pem(include_bytes!("../../../keys/rs256-4096-public.pem")).unwrap(),
        &Validation::new(Algorithm::RS256),
    )
    .map(|data| data.claims)
    .map_err(|e| ServiceError::BadRequest(e.to_string()))
}
