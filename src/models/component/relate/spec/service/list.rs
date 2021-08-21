use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use async_graphql::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_all_component_specs(
    context: &Context<'_>,
    target_uuid_component: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SpecTranslateList>> {
    use crate::schema::spec_to_component::dsl as spec_to_component;
    use crate::schema::spec_translate_list::dsl as spec_translate_list;
    let conn: &PooledConnection = &get_conn(context)?;

    let set_id_lang = crate::models::user::get_set_language(context);

    let id_spec_for_uuid: Vec<i32> = spec_to_component::spec_to_component
        .filter(spec_to_component::uuid_component.eq(target_uuid_component))
        .select(spec_to_component::id_spec)
        .load::<i32>(conn)?;

    Ok(spec_translate_list::spec_translate_list
        .filter(spec_translate_list::id_spec.eq_any(id_spec_for_uuid))
        .filter(spec_translate_list::id_lang.eq(set_id_lang))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SpecTranslateList>(conn)?)
}
