use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::standard::model::ShowStandard;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_standards(
    context: &Context<'_>,
    uuid_standard_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowStandard>> {
    let mut variant_selection: u8 = 0;
    if uuid_standard_search > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_standards(context, limit, offset),
        1 => find_uuid_standard(context, uuid_standard_search, limit, offset),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_standards(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowStandard>> {
    use crate::schema::standard_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(standard_ref
        .select((
            uuid, uuid_standard_parent, classifier, name, description,
            specified_tolerance, technical_committee, publication_at,
            uuid_image_file, uuid_user, uuid_company, id_type_access,
            id_standard_status, id_region, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowStandard>(conn)?)
}

fn find_uuid_standard(
    context: &Context<'_>,
    uuid_standard_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowStandard>> {
    use crate::schema::standard_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(&context)?;

    Ok(standard_ref
        .filter(uuid.eq(uuid_standard_search))
        .select((
            uuid, uuid_standard_parent, classifier, name, description,
            specified_tolerance, technical_committee, publication_at,
            uuid_image_file, uuid_user, uuid_company, id_type_access,
            id_standard_status, id_region, is_delete, created_at, updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowStandard>(conn)?)
}
