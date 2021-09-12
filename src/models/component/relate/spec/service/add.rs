use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::component::spec::model::{
    SpecComponent,
    IptSpecComponentData,
    InsertableSpecComponent
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_component_spec(
    data: IptSpecComponentData,
    conn: &PgConnection
) -> ServiceResult<SpecComponent> {
    use crate::schema::spec_to_component::dsl::*;

    let new_component_spec: InsertableSpecComponent = data.into();

    let flag_found_spec = spec_to_component
        .filter(component_uuid.eq(&new_component_spec.component_uuid)
        .and(spec_id.eq(&new_component_spec.spec_id)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_spec START SEARCH ={:?}", flag_found_spec);

    match flag_found_spec as i32 {
        0 => {
            let inserted_component_spec: SpecComponent = diesel::insert_into(spec_to_component)
                .values(&new_component_spec)
                .get_result(conn)?;
            Ok(inserted_component_spec)
        },
        _ => Err(ServiceError::BadRequest("This spec name is already with the component.".to_string())),
    }
}
