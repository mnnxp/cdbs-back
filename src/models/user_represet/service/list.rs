use crate::database::PooledConnection;
use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::model::Context;
use crate::models::user_represet::model::ShowUserRepreset;
use diesel::prelude::*;
// use std::any::Any;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowUserRepreset>> {
    match uuid_user_search {
        uuid_user_search if uuid_user_search == Uuid::nil() => find_all_user_represets(context, limit, offset),
        uuid_user_search if uuid_user_search > Uuid::nil() => find_uuid_user_represets(context, uuid_user_search, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_user_represets(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowUserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    use crate::schema::representation_type_ref::dsl::*;
    use crate::schema::region_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .inner_join(representation_type_ref)
        .inner_join(region_ref)
        .select((
            uuid, uuid_user, id_region, region, id_representation_type,
            representation_type, name, address, phone
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowUserRepreset>(conn)?)
}

fn find_uuid_user_represets(
    context: &Context,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowUserRepreset>> {
    use crate::schema::user_represet_ref::dsl::*;
    use crate::schema::representation_type_ref::dsl::*;
    use crate::schema::region_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(user_represet_ref
        .inner_join(representation_type_ref)
        .inner_join(region_ref)
        .filter(uuid_user.eq(uuid_user_search))
        .select((
            uuid, uuid_user, id_region, region, id_representation_type,
            representation_type, name, address, phone
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowUserRepreset>(conn)?)
}
