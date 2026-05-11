use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::model::{CompaniesArg, CompanyAndRelatedData, ShowCompanyShort};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::Paginate;
use crate::schema::company_ref::dsl as company_ref;
use crate::schema::company_fav::dsl as company_fav;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

/// Returns aggregated company data with filters by UUID, favorites, and suppliers.
/// Supports search by orgname, shortname, or INN, and exclusion of specific UUIDs.
pub(crate) fn get_companies(
    arguments: &CompaniesArg,
    paginate: &Paginate,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowCompanyShort>> {
    let CompaniesArg {
        filter_companies_uuids,
        user_uuid,
        favorite,
        supplier,
        search,
        exclude_uuids,
    } = arguments;

    let mut select_uuids = match (user_uuid, favorite) {
        // Companies owned by specific user
        (Some(ur_uuid), false) => get_companies_by_user(filter_companies_uuids, ur_uuid, conn)?,
        // Companies followed by specific user
        (Some(ur_uuid), true) => get_companies_followed_by_user(filter_companies_uuids, ur_uuid, conn)?,
        // Companies followed by current user
        (None, true) => get_companies_followed_by_user(filter_companies_uuids, &options.logged_user_uuid, conn)?,
        // Public companies (with search and exclude support)
        (None, false) => filter_companies_uuids.to_vec(),
    };

    // Apply search filter if present and select_uuids is not empty
    if let Some(search_term) = search {
        select_uuids = filter_companies_by_search(&select_uuids, search_term, conn)?;
    }

    // Apply exclude UUIDs
    if let Some(exclude) = exclude_uuids {
        select_uuids.retain(|uuid| !exclude.contains(uuid));
    }

    match select_uuids.is_empty() {
        true if *favorite || user_uuid.is_some() || search.is_some() => Ok(Vec::new()),
        true => ShowCompanyShort::get_all_public(supplier, paginate, conn),
        false => ShowCompanyShort::get_list_by_uuids(&select_uuids, supplier, options, paginate, conn),
    }
}

/// Filters company UUIDs by search term in orgname, shortname, or INN
fn filter_companies_by_search(
    company_uuids: &[Uuid],
    search_term: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    if search_term.is_empty() {
        return Ok(Vec::new())
    }
    let mut query = company_ref::company_ref.into_boxed();
    if !company_uuids.is_empty() {
        query = query.filter(company_ref::uuid.eq_any(company_uuids))
    }
    let search_pattern = format!("%{}%", search_term);
    query.filter(company_ref::orgname.ilike(&search_pattern)
        .or(company_ref::shortname.ilike(&search_pattern))
        .or(company_ref::inn.ilike(&search_pattern)))
        .select(company_ref::uuid)
        .load(conn)
        .map_err(|err| {
            debug!("Failed filter companies by search: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets company UUIDs by owner user
fn get_companies_by_user(
    filter_companies_uuids: &[Uuid],
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = company_ref::company_ref.into_boxed();
    query = query.filter(company_ref::user_uuid.eq(user_uuid));

    if !filter_companies_uuids.is_empty() {
        query = query.filter(company_ref::uuid.eq_any(filter_companies_uuids));
    }

    query.select(company_ref::uuid).load(conn).map_err(|err| {
        debug!("Failed get companies by user: {:?}", err);
        ServiceError::InternalServerError
    })
}

/// Gets company UUIDs followed by user
fn get_companies_followed_by_user(
    filter_companies_uuids: &[Uuid],
    user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = company_fav::company_fav.into_boxed();
    query = query.filter(company_fav::user_uuid.eq(user_uuid));
    query = query.filter(company_fav::is_enabled.eq(true));

    if !filter_companies_uuids.is_empty() {
        query = query.filter(company_fav::company_uuid.eq_any(filter_companies_uuids));
    }

    query.select(company_fav::company_uuid).load(conn).map_err(|err| {
        debug!("Failed get companies followed by user: {:?}", err);
        ServiceError::InternalServerError
    })
}

/// Returns company data by UUID with access checks.
pub(crate) fn find_by_uuid(
    target_company_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {
    CompanyAndRelatedData::get_by_uuid(target_company_uuid, options, conn)
        .map_err(|err| {
            debug!("Error loading company and collect related data: {:?}", err);
            get_err_msg(ErrorMessage::AccessDenied)
        })
}

/// Returns public supplier company data by UUID without permission checks.
pub(crate) fn get_supplier_by_uuid(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {
    CompanyAndRelatedData::get_supplier_by_uuid(target_company_uuid, conn)
        .map_err(|err| {
            debug!("Error loading supplier company and collect related data: {:?}", err);
            ServiceError::InternalServerError
        })
}