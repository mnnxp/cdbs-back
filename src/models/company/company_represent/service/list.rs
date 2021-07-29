use crate::database::{get_conn, PooledConnection};
use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::company::company_represent::model::ShowCompanyRepresent;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;

pub(crate) fn get_company_represents(
    context: &Context<'_>,
    uuid_company_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompanyRepresent>> {
    match uuid_company_search {
        uuid_company_search if uuid_company_search == Uuid::nil() => {
            find_all_company_represents(context, limit, offset)
        }
        uuid_company_search if uuid_company_search > Uuid::nil() => {
            find_uuid_company_represents(context, uuid_company_search, limit, offset)
        }
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string())),
    }
}

fn find_all_company_represents(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompanyRepresent>> {
    use crate::schema::company_represent_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(company_represent_ref
        .select((
            uuid,
            uuid_company,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowCompanyRepresent>(conn)?)
}

fn find_uuid_company_represents(
    context: &Context<'_>,
    uuid_company_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowCompanyRepresent>> {
    use crate::schema::company_represent_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(company_represent_ref
        .filter(uuid_company.eq(uuid_company_search))
        .select((
            uuid,
            uuid_company,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowCompanyRepresent>(conn)?)
}
