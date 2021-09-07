use crate::errors::{ ServiceError, ServiceResult };
use crate::models::company::representation_type::model::{
    RepresentationType,
    RepresentationTypeTranslateList,
    IptRepresentationTypeTranslateListData,
    InsertableRepresentationTypeTranslateList,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_representation_type(
    new_representation_type_data: IptRepresentationTypeTranslateListData,
    conn: &PgConnection
) -> ServiceResult<RepresentationTypeTranslateList> {
    use crate::schema::representation_type_translate_list::dsl as representation_type_translate_list;

    let flag_found_representation_type = representation_type_translate_list::representation_type_translate_list
        .filter(representation_type_translate_list::id_lang.eq(&new_representation_type_data.id_lang))
        .filter(representation_type_translate_list::name.eq(&new_representation_type_data.name))
        .select(representation_type_translate_list::id_representation_type)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_representation_type START SEARCH ={:?}", flag_found_representation_type);

    match flag_found_representation_type {
        0 => {
            let new_id_representation_type = {
                use crate::schema::representation_type_ref::dsl as representation_type_ref;

                let new_representation_type: RepresentationType = diesel::insert_into(representation_type_ref::representation_type_ref)
                    .default_values()
                    .get_result(conn)?;

                new_representation_type.id
            };

            let new_representation_type_data = InsertableRepresentationTypeTranslateList {
                id_representation_type: new_id_representation_type,
                id_lang: new_representation_type_data.id_lang,
                representation_type: new_representation_type_data.representation_type,
            };
            let inserted_representation_type_data: RepresentationTypeTranslateList = diesel::insert_into(representation_type_translate_list::representation_type_translate_list)
                .values(&new_representation_type_data)
                .get_result(conn)?;
            Ok(inserted_representation_type_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This company type name is already there. Id: {}", flag_found_representation_type))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
