//! In-memory cache for static reference tables (dictionaries).
//!
//! Reference tables change rarely (essentially never in production), so we
//! load all of them **once at startup** into a single `StaticDictionaries`
//! struct and read lock-free afterwards (`&'static` via `OnceLock`).
//!
//! Adding a new language or row requires a restart — by design.

use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::relate_ref::program::model::Program;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::models::supplier_service::service_status::model::ServiceStatusTranslateList;
use crate::schema::actual_status_translate_list::dsl as astl;
use crate::schema::company_type_translate_list::dsl as cttl;
use crate::schema::component_type_translate_list::dsl as cttp;
use crate::schema::program_ref::dsl as pr;
use crate::schema::region_translate_list::dsl as rtl;
use crate::schema::representation_type_translate_list::dsl as rttl;
use crate::schema::service_status_translate_list::dsl as sstl;
use crate::schema::standard_status_translate_list::dsl as sdtl;
use crate::schema::type_access_translate_list::dsl as tatl;
use diesel::prelude::*;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Default language used when a requested language has no row.
const DEFAULT_LANG_ID: i32 = 1;

/// All static reference tables.
///
/// Language-keyed dictionaries use `HashMap<(lang_id, id), T>`.
/// `program` has no `lang_id`, so it uses `HashMap<i32, Program>`.
///
/// Loaded once in `StaticDictionaries::init` (called from `main.rs` before
/// the server starts). After that, all lookups are lock-free reads.
#[derive(Clone)]
pub struct StaticDictionaries {
    pub type_access: HashMap<(i32, i32), TypeAccessTranslateList>,
    pub component_type: HashMap<(i32, i32), ComponentTypeTranslateList>,
    pub actual_status: HashMap<(i32, i32), ActualStatusTranslateList>,
    pub standard_status: HashMap<(i32, i32), StandardStatusTranslateList>,
    pub service_status: HashMap<(i32, i32), ServiceStatusTranslateList>,
    pub company_type: HashMap<(i32, i32), CompanyTypeTranslateList>,
    pub region: HashMap<(i32, i32), RegionTranslateList>,
    pub representation_type: HashMap<(i32, i32), RepresentationTypeTranslateList>,
    pub program: HashMap<i32, Program>,
}

static DICTIONARIES: OnceLock<StaticDictionaries> = OnceLock::new();

impl StaticDictionaries {
    /// Loads all reference tables into memory.
    ///
    /// **Call once from `main.rs`** before the Actix server starts. All tables
    /// are fetched with full-table `SELECT`s (no `WHERE lang_id`), so every
    /// language is available without lazy loading.
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

        let component_type = cttp::component_type_translate_list
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

        let standard_status = sdtl::standard_status_translate_list
            .load::<StandardStatusTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load standard_status_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.standard_status_id), r))
            .collect();

        let service_status = sstl::service_status_translate_list
            .load::<ServiceStatusTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load service_status_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.service_status_id), r))
            .collect();

        let company_type = cttl::company_type_translate_list
            .load::<CompanyTypeTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load company_type_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.company_type_id), r))
            .collect();

        let region = rtl::region_translate_list
            .load::<RegionTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load region_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.region_id), r))
            .collect();

        let representation_type = rttl::representation_type_translate_list
            .load::<RepresentationTypeTranslateList>(conn)
            .map_err(|e| {
                error!("Failed to load representation_type_translate_list: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| ((r.lang_id, r.representation_type_id), r))
            .collect();

        let program = pr::program_ref
            .load::<Program>(conn)
            .map_err(|e| {
                error!("Failed to load program_ref: {:?}", e);
                ServiceError::InternalServerError
            })?
            .into_iter()
            .map(|r| (r.id, r))
            .collect();

        DICTIONARIES
            .set(StaticDictionaries {
                type_access,
                component_type,
                actual_status,
                standard_status,
                service_status,
                company_type,
                region,
                representation_type,
                program,
            })
            .map_err(|_| {
                error!("StaticDictionaries was already initialized");
                ServiceError::InternalServerError
            })?;

        debug!("Static dictionaries loaded");
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

/// Access type name (Manage / Write / Read) for the requested language
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

/// Component type name for the requested language
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

/// Actual status name for the requested language
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

/// Standard status name for the requested language
pub(crate) fn standard_status_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<StandardStatusTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .standard_status
        .get(&(lang_id, target_id))
        .or_else(|| dicts.standard_status.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Service status name for the requested language
pub(crate) fn service_status_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<ServiceStatusTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .service_status
        .get(&(lang_id, target_id))
        .or_else(|| dicts.service_status.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Company type name and shortname for the requested language
pub(crate) fn company_type_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<CompanyTypeTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .company_type
        .get(&(lang_id, target_id))
        .or_else(|| dicts.company_type.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Region name for the requested language
pub(crate) fn region_by_id(target_id: i32, lang_id: i32) -> ServiceResult<RegionTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .region
        .get(&(lang_id, target_id))
        .or_else(|| dicts.region.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Representation type name for the requested language
pub(crate) fn representation_type_by_id(
    target_id: i32,
    lang_id: i32,
) -> ServiceResult<RepresentationTypeTranslateList> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .representation_type
        .get(&(lang_id, target_id))
        .or_else(|| dicts.representation_type.get(&(DEFAULT_LANG_ID, target_id)))
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}

/// Software/program by ID (no language)
pub(crate) fn program_by_id(target_id: i32) -> ServiceResult<Program> {
    let dicts = StaticDictionaries::get_ref()?;
    dicts
        .program
        .get(&target_id)
        .cloned()
        .ok_or(ServiceError::InternalServerError)
}
