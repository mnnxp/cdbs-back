use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::user_api_key_ref::dsl as user_api_key_ref;
use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::result::Error;
use rand::distributions::{Alphanumeric, DistString};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::model::{ApiKey, IptUpdateApiKeyData};

/// Hashes an API key using SHA-256 for secure storage
fn hash_api_key(raw_key: &str) -> Vec<u8> {
    Sha256::digest(raw_key.as_bytes()).to_vec()
}

/// Generates a secure random API key
fn generate_raw_api_key() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 32)
}

/// Validates API key and returns user UUID
pub(crate) fn validate_api_key(raw_key: &str, conn: &mut PgConnection) -> ServiceResult<Uuid> {
    let key_hash = hash_api_key(raw_key);
    let now = Utc::now();

    let result = diesel::update(
        user_api_key_ref::user_api_key_ref
            .filter(user_api_key_ref::key_hash.eq(&key_hash))
            .filter(user_api_key_ref::is_active.eq(true))
            .filter(user_api_key_ref::expires_at.gt(now)),
    )
    .set(user_api_key_ref::last_used_at.eq(now))
    .returning(user_api_key_ref::user_uuid)
    .get_result::<Uuid>(conn)
    .optional()?;

    result.ok_or(ServiceError::Unauthorized)
}

/// Creates a new API key and returns the raw key string.
pub(crate) fn generate_api_key(
    user_uuid: &Uuid,
    name: &str,
    expires_at: Option<DateTime<Utc>>,
    conn: &mut PgConnection,
) -> ServiceResult<String> {
    if name.is_empty() {
        return Err(get_err_msg(ErrorMessage::DataNotFound));
    }
    if name.chars().count() > 100 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(100)));
    }

    let date_now = Utc::now();
    let set_expires_at = match expires_at {
        Some(dt) => {
            // Reject expiration dates that are in the past
            if dt < date_now {
                return Err(get_err_msg(ErrorMessage::FailedUpdateData));
            }
            dt
        }
        // Set default expiration period to 1 year if not provided
        None => date_now + Duration::days(365),
    };

    // Generate a random key and store only its hash
    let raw_key = generate_raw_api_key();
    let key_hash = hash_api_key(&raw_key);

    diesel::insert_into(user_api_key_ref::user_api_key_ref)
        .values((
            user_api_key_ref::user_uuid.eq(*user_uuid),
            user_api_key_ref::key_hash.eq(key_hash),
            user_api_key_ref::name.eq(name),
            user_api_key_ref::expires_at.eq(set_expires_at),
            user_api_key_ref::is_active.eq(true),
            user_api_key_ref::created_at.eq(Utc::now()),
        ))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed to create API key: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(raw_key)
}

/// Lists all API keys for a user
pub(crate) fn list_api_keys(
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ApiKey>> {
    user_api_key_ref::user_api_key_ref
        .filter(user_api_key_ref::user_uuid.eq(user_uuid))
        .order(user_api_key_ref::created_at.desc())
        .load::<ApiKey>(conn)
        .map_err(|err| {
            debug!("Failed to list API keys: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets a specific API key by ID (with ownership check)
pub(crate) fn get_api_key(
    key_id: i32,
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<ApiKey> {
    user_api_key_ref::user_api_key_ref
        .filter(user_api_key_ref::id.eq(key_id))
        .filter(user_api_key_ref::user_uuid.eq(user_uuid))
        .first::<ApiKey>(conn)
        .map_err(|err| match err {
            Error::NotFound => get_err_msg(ErrorMessage::DataNotFound),
            _ => ServiceError::InternalServerError,
        })
}

/// Updates API key metadata
pub(crate) fn change_api_key(
    key_id: i32,
    user_uuid: &Uuid,
    data: &IptUpdateApiKeyData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    if data.name.is_none() && data.expires_at.is_none() && data.is_active.is_none() {
        return Ok(0);
    }

    if let Some(name) = &data.name {
        if name.chars().count() > 100 {
            return Err(get_err_msg(ErrorMessage::TextMustLess(100)));
        }
    }

    diesel::update(
        user_api_key_ref::user_api_key_ref
            .filter(user_api_key_ref::id.eq(key_id))
            .filter(user_api_key_ref::user_uuid.eq(*user_uuid)),
    )
    .set(data)
    .execute(conn)
    .map_err(|err| {
        debug!("Failed to update API key: {:?}", err);
        ServiceError::InternalServerError
    })
}

/// Deletes (revokes) an API key
pub(crate) fn revoke_api_key(
    key_id: i32,
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let deleted = diesel::delete(
        user_api_key_ref::user_api_key_ref
            .filter(user_api_key_ref::id.eq(key_id))
            .filter(user_api_key_ref::user_uuid.eq(*user_uuid)),
    )
    .execute(conn)?;

    Ok(deleted > 0)
}

/// Regenerates an API key (deletes old, creates new with same metadata)
pub(crate) fn regenerate_api_key(
    key_id: i32,
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<String> {
    conn.transaction::<String, ServiceError, _>(|transaction_conn| {
        // Get old key data before deletion
        let old_key: ApiKey = user_api_key_ref::user_api_key_ref
            .filter(user_api_key_ref::id.eq(key_id))
            .filter(user_api_key_ref::user_uuid.eq(*user_uuid))
            .first(transaction_conn)
            .map_err(|err| match err {
                Error::NotFound => get_err_msg(ErrorMessage::DataNotFound),
                _ => ServiceError::InternalServerError,
            })?;

        // Delete old key
        diesel::delete(user_api_key_ref::user_api_key_ref.filter(user_api_key_ref::id.eq(key_id)))
            .execute(transaction_conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        // Create new key with same name and expiration
        generate_api_key(
            user_uuid,
            &old_key.name,
            Some(old_key.expires_at),
            transaction_conn,
        )
    })
}
