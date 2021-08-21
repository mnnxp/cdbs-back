use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::license::model::LicenseComponent;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_licenses_component(
    context: &Context<'_>,
    id_license_search: i32,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<LicenseComponent>> {
    let mut variant_selection: u8 = 0;
    if id_license_search > 0 {
        variant_selection += 1;
    }
    if uuid_component_search > Uuid::nil() {
        variant_selection += 10;
    }

    match variant_selection {
        0 => find_all_licenses(context, limit, offset),
        1 => find_id_license(context, id_license_search, limit, offset),
        10 => find_uuid_component_license(context, uuid_component_search, limit, offset),
        11 => find_id_license_and_uuid_component(context, id_license_search, uuid_component_search, limit, offset),
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_licenses(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<LicenseComponent>> {
    use crate::schema::license_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(license_to_component
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<LicenseComponent>(conn)?)
}

fn find_id_license(
    context: &Context<'_>,
    id_license_search: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<LicenseComponent>> {
    use crate::schema::license_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(license_to_component
        .filter(id_license.eq(id_license_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<LicenseComponent>(conn)?)
}

fn find_uuid_component_license(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<LicenseComponent>> {
    use crate::schema::license_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(license_to_component
        .filter(uuid_component.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<LicenseComponent>(conn)?)
}

fn find_id_license_and_uuid_component(
    context: &Context<'_>,
    id_license_search: i32,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<LicenseComponent>> {
    use crate::schema::license_to_component::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(license_to_component
        .filter(uuid_component.eq(uuid_component_search))
        .filter(id_license.eq(id_license_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<LicenseComponent>(conn)?)
}
