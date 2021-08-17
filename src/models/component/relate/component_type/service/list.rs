use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    // ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::component_type::model::ComponentType;
use diesel::prelude::*;


pub(crate) fn get_component_type(
    context: &Context<'_>,
    target_id_component_type: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentType>> {
    match target_id_component_type {
        target_id_component_type if target_id_component_type.is_empty() => find_all_component_type(context, limit, offset),
        target_id_component_type => find_id_component_type(context, target_id_component_type, limit, offset)
        // _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_component_type(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentType>> {
    use crate::schema::component_type_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;
    // let target_id_lang: IdLanguage = context.into();

    Ok(component_type_ref
        // .filter(id_lang.eq_any(target_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentType>(conn)?)
}

fn find_id_component_type(
    context: &Context<'_>,
    target_id_component_type: Vec<i32>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ComponentType>> {
    use crate::schema::component_type_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(component_type_ref
        // .filter(id_lang.eq_any(target_id_lang))
        .filter(id.eq_any(target_id_component_type))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ComponentType>(conn)?)
}
