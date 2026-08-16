// src/auth/access/cache.rs

use diesel::PgConnection;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::auth::rbac::AccessEntity;
use crate::errors::{ServiceError, ServiceResult};

use super::AccessInfo;
use super::{get_company_access, get_component_access, get_service_access, get_standard_access};

/// Global thread-safe storage for access control caching
static ACCESS_CACHE: OnceLock<RwLock<HashMap<AccessCacheKey, CachedAccess>>> = OnceLock::new();

/// Returns a reference to the global access cache
fn get_cache() -> &'static RwLock<HashMap<AccessCacheKey, CachedAccess>> {
    ACCESS_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct AccessCacheKey {
    entity: AccessEntity,
    user_uuid: Uuid,
    object_uuid: Uuid,
}

struct CachedAccess {
    result: AccessInfo,
    cached_at: Instant,
}

const TTL_SECONDS: u64 = 300; // 5 minutes

pub(crate) fn check_component_access(
    user_uuid: &Uuid,
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    with_cache(
        AccessEntity::Component,
        user_uuid,
        component_uuid,
        conn,
        get_component_access,
    )
}

pub(crate) fn check_standard_access(
    user_uuid: &Uuid,
    standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    with_cache(
        AccessEntity::Standard,
        user_uuid,
        standard_uuid,
        conn,
        get_standard_access,
    )
}

pub(crate) fn check_service_access(
    user_uuid: &Uuid,
    service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    with_cache(
        AccessEntity::Service,
        user_uuid,
        service_uuid,
        conn,
        get_service_access,
    )
}

pub(crate) fn check_company_access(
    user_uuid: &Uuid,
    company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<AccessInfo> {
    with_cache(
        AccessEntity::Company,
        user_uuid,
        company_uuid,
        conn,
        get_company_access,
    )
}

fn with_cache<F>(
    entity: AccessEntity,
    user_uuid: &Uuid,
    object_uuid: &Uuid,
    conn: &mut PgConnection,
    checker: F,
) -> ServiceResult<AccessInfo>
where
    F: Fn(&Uuid, &Uuid, &mut PgConnection) -> ServiceResult<AccessInfo>,
{
    let key = AccessCacheKey {
        entity,
        user_uuid: *user_uuid,
        object_uuid: *object_uuid,
    };

    // Read from cache
    {
        let cache = match get_cache().read() {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to acquire read lock: {}", e);
                return Err(ServiceError::InternalServerError);
            }
        };
        if let Some(cached) = cache.get(&key) {
            if cached.cached_at.elapsed() < Duration::from_secs(TTL_SECONDS) {
                debug!("Access cache HIT: {:?}", key);
                return Ok(cached.result.clone());
            }
        }
    }

    debug!("Access cache MISS: {:?}", key);

    // Execute check
    let result = checker(user_uuid, object_uuid, conn);
    debug!(
        "Get access {:?} in object {:?}, result {:?}",
        user_uuid, object_uuid, result
    );
    let result = result?;
    // Store in cache
    {
        let mut cache = match get_cache().write() {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to acquire write lock: {}", e);
                return Err(ServiceError::InternalServerError);
            }
        };
        cache.insert(
            key,
            CachedAccess {
                result: result.clone(),
                cached_at: Instant::now(),
            },
        );
    }

    Ok(result)
}

// For invalidation when permissions change
pub(crate) fn invalidate_access(user_uuid: &Uuid, entity: AccessEntity, object_uuid: &Uuid) {
    invalidate_object_cache(entity, object_uuid);
    invalidate_user_cache(user_uuid);
}

pub(crate) fn invalidate_user_cache(user_uuid: &Uuid) {
    match get_cache().write() {
        Ok(mut cache) => {
            cache.retain(|key, _| &key.user_uuid != user_uuid);
            debug!("Cache invalidated for user: {:?}", user_uuid);
        }
        Err(e) => log::error!("Cache write lock failed for user: {}", e),
    }
}

pub(crate) fn invalidate_object_cache(entity: AccessEntity, object_uuid: &Uuid) {
    match get_cache().write() {
        Ok(mut cache) => {
            cache.retain(|key, _| key.entity != entity || &key.object_uuid != object_uuid);
            debug!("Cache invalidated for {:?}: {:?}", entity, object_uuid);
        }
        Err(e) => log::error!("Cache write lock failed for object: {}", e),
    }
}
