use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component::supplier::model::SupplierComponent;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_all_component_suppliers(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SupplierComponent>> {
    use crate::schema::supplier_to_component::dsl as supplier_to_component;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(supplier_to_component::supplier_to_component
        .filter(supplier_to_component::uuid_component.eq(target_uuid_component))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SupplierComponent>(conn)?)
}
