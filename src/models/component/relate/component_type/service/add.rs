use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::component_type::model::{
    ComponentType,
    ComponentTypeTranslateList,
    IptComponentTypeTranslateListData,
    InsertableComponentTypeTranslateList,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_component_type(
    new_component_type_data: IptComponentTypeTranslateListData,
    conn: &PgConnection
) -> ServiceResult<ComponentTypeTranslateList> {
    use crate::schema::component_type_translate_list::dsl as component_type_translate_list;

    let flag_found_component_type = component_type_translate_list::component_type_translate_list
        .filter(component_type_translate_list::lang_id.eq(&new_component_type_data.lang_id))
        .filter(component_type_translate_list::component_type.eq(&new_component_type_data.component_type))
        .select(component_type_translate_list::component_type_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_component_type START SEARCH ={:?}", flag_found_component_type);

    match flag_found_component_type {
        0 => {
            let new_component_type_id = {
                use crate::schema::component_type_ref::dsl as component_type_ref;

                let new_component_type: ComponentType = diesel::insert_into(component_type_ref::component_type_ref)
                    .default_values()
                    .get_result(conn)?;

                new_component_type.id
            };

            let new_component_type_data = InsertableComponentTypeTranslateList {
                component_type_id: new_component_type_id,
                lang_id: new_component_type_data.lang_id,
                component_type: new_component_type_data.component_type,
            };
            let inserted_component_type_data: ComponentTypeTranslateList = diesel::insert_into(component_type_translate_list::component_type_translate_list)
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
