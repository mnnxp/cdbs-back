use crate::database::PooledConnection;
use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::graphql::model::Context;
use crate::models::component_modification::model::ShowComponentModification;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn show(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    match uuid_component_search {
        uuid_component_search if uuid_component_search == Uuid::nil() => find_all_component_modification(context, limit, offset),
        uuid_component_search if uuid_component_search > Uuid::nil() => find_uuid_component_modification(context, uuid_component_search, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_component_modification(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    use crate::schema::component_modification_list::dsl::*;
    use crate::schema::name_cad_ref::dsl::*;
    use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_modification_list
        .inner_join(name_cad_ref)
        .inner_join(actual_status_ref)
        .select((
            uuid, uuid_component, modification_name, created_at, id_name_cad, name_cad,
            comment, uuid_modification_parent, commentchange, id_actual_status,
            actualstatus, is_delete
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponentModification>(conn)?)
}

fn find_uuid_component_modification(
    context: &Context,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowComponentModification>> {
    use crate::schema::component_modification_list::dsl::*;
    use crate::schema::name_cad_ref::dsl::*;
    use crate::schema::actual_status_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(component_modification_list
        .inner_join(name_cad_ref)
        .inner_join(actual_status_ref)
        .select((
            uuid, uuid_component, modification_name, created_at, id_name_cad, name_cad,
            comment, uuid_modification_parent, commentchange, id_actual_status,
            actualstatus, is_delete
        ))
        .filter(uuid_component.eq(uuid_component_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowComponentModification>(conn)?)
}
