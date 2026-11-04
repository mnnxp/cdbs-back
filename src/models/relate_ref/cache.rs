//! In-memory cache for static reference tables (dictionaries).
//!
//! Reference tables change rarely (essentially never in production), so we
//! load all of them **once at startup** into a single `StaticDictionaries`
//! struct and read lock-free afterwards (`&'static` via `OnceLock`).
//!
//! Adding a new language or row requires a restart — by design.

use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::actual_status_translate_list::dsl as astl;
use crate::schema::component_type_translate_list::dsl as cttl;
use crate::schema::type_access_translate_list::dsl as tatl;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Default language used when a requested language has no row.
const DEFAULT_LANG_ID: i32 = 1;

/// All static reference tables, keyed by `(lang_id, id)`.
///
/// Loaded once in `StaticDictionaries::init` (called from `main.rs` before
/// the server starts). After that, all lookups are lock-free reads.
#[derive(Clone)]
pub struct StaticDictionaries {
    pub type_access: HashMap<(i32, i32), TypeAccessTranslateList>,
    pub component_type: HashMap<(i32, i32), ComponentTypeTranslateList>,
    pub actual_status: HashMap<(i32, i32), ActualStatusTranslateList>,
}

static DICTIONARIES: OnceLock<StaticDictionaries> = OnceLock::new();

impl StaticDictionaries {
    /// Loads all reference tables into memory.
    ///
    /// **Call once from `main.rs`** before the Actix server starts. All 3
    /// tables are fetched with 3 full-table `SELECT`s (no `WHERE lang_id`),
    /// so every language is available without lazy loading.
    pub fn init(conn: &mut PgConnection) -> ServiceResult<()> {
        let type_access = tatl::type_access_translate_list
            .load::<TypeAccessTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load type_access_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.type_access_id), r))
            .collect();

        let component_type = cttl::component_type_translate_list
            .load::<ComponentTypeTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load component_type_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.component_type_id), r))
            .collect();

        let actual_status = astl::actual_status_translate_list
            .load::<ActualStatusTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load actual_status_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.actual_status_id), r))
            .collect();

        DICTIONARIES
            .set(StaticDictionaries {
                type_access,
                component_type,
                actual_status,
            })
            .map_err(|_| {
                error!("StaticDictionaries was already initialized");
                ServiceError::InternalServerError
            })?;

        debug!(
            "Static dictionaries loaded: type_access={}, component_type={}, actual_status={}",
            DICTIONARIES.get().map(|d| d.type_access.len()).unwrap_or(0),
            DICTIONARIES
                .get()
                .map(|d| d.component_type.len())
                .unwrap_or(0),
            DICTIONARIES
                .get()
                .map(|d| d.actual_status.len())
                .unwrap_or(0),
        );
        Ok(())
    }

    /// Returns the global dictionaries or an error if `init` was not called.
    fn get_ref() -> ServiceResult<&'static Self> {
        DICTIONARIES.get().ok_or_else(|| {
            error!("StaticDictionaries was not initialized at startup");
            ServiceError::InternalServerError
        })
    }
}

// Public lookups (lock-free, no DB connection)

/// Cached equivalent of `ActualStatusTranslateList::get_by_id`.
pub(crate) fn actual_status_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<ActualStatusTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .actual_status
        .get(&(lang_id, target_id))
        .or_else(|| dicts.actual_status.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Cached equivalent of `TypeAccessTranslateList::get_type_access_by_id`.
/// Falls back to `DEFAULT_LANG_ID` when `lang_id` has no row.
pub(crate) fn type_access_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<TypeAccessTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .type_access
        .get(&(lang_id, target_id))
        .or_else(|| dicts.type_access.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Cached equivalent of `ComponentTypeTranslateList::get_by_id`.
pub(crate) fn component_type_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<ComponentTypeTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .component_type
        .get(&(lang_id, target_id))
        .or_else(|| dicts.component_type.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}
