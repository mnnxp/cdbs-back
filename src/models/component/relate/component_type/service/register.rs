use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::component_type::model::{
    InsertableComponentType, ComponentType, IptComponentTypeData
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_component_type(
    new_component_type_data: IptComponentTypeData,
    conn: &PgConnection
) -> ServiceResult<ComponentType> {
    use crate::schema::component_type_ref::dsl::*;

    let flag_found_component_type = component_type_ref
        .filter(id_lang.eq(&new_component_type_data.id_lang))
        .filter(name.eq(&new_component_type_data.name))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_component_type START SEARCH ={:?}", flag_found_component_type);

    match flag_found_component_type {
        0 => {
            let new_component_type_data: InsertableComponentType = new_component_type_data.into();
            let inserted_component_type_data: ComponentType = diesel::insert_into(component_type_ref)
                .values(&new_component_type_data)
                .get_result(conn)?;
            Ok(inserted_component_type_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This component_type name is already there. Id: {}", flag_found_component_type))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
