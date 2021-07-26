use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::company::model::ShowCompany;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_companies(
    context: &Context<'_>,
    uuid_company_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompany>> {
    let mut variant_selection: u8 = 0;
    if uuid_company_search > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_companys(context, limit, offset),
        1 => find_uuid_company(context, uuid_company_search, limit, offset),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_companys(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompany>> {
    use crate::schema::company_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(company_ref
        .select((
            uuid, orgname, shortname, inn, phone, email, description, address, site_url,
            time_zone, uuid_user, uuid_image_file, id_region, id_type_org, is_supplier,
            is_email_verified, is_enabled, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowCompany>(conn)?)
}

fn find_uuid_company(
    context: &Context<'_>,
    uuid_company_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompany>> {
    use crate::schema::company_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(company_ref
        .filter(uuid.eq(uuid_company_search))
        .select((
            uuid, orgname, shortname, inn, phone, email, description, address, site_url,
            time_zone, uuid_user, uuid_image_file, id_region, id_type_org, is_supplier,
            is_email_verified, is_enabled, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowCompany>(conn)?)
}
