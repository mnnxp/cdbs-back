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
    data: Vec<IptSpecComponentData>,
    conn: &PgConnection
) -> ServiceResult<Vec<SpecComponent>> {
    use crate::schema::spec_to_component::dsl::*;

    let checking_duplicates: Vec<InsertableSpecComponent> = data.clone().into_iter()
        .map(|f| f.into()).collect();

    let flag_found_spec = spec_to_component
        .filter(uuid_component.eq_any(checking_duplicates.iter().map(|x| x.uuid_component))
        .and(id_spec.eq_any(checking_duplicates.iter().map(|x| x.id_spec))))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_spec START SEARCH ={:?}", flag_found_spec);

    match flag_found_spec as i32 {
        0 => {
            let new_component_spec: Vec<InsertableSpecComponent> = data.into_iter()
                .map(|f| f.into()).collect();
        
            let inserted_component_spec: Vec<SpecComponent> = diesel::insert_into(spec_to_component)
                .values(&new_component_spec)
                .load(conn)?;
            Ok(inserted_component_spec)
        },
        _ => Err(ServiceError::BadRequest("This spec name is already with the component.".to_string())),
    }
}
