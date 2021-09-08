use crate::errors::{ ServiceError, ServiceResult };
use crate::models::standard::standard_status::model::{
    StandardStatus,
    StandardStatusTranslateList,
    IptStandardStatusTranslateListData,
    InsertableStandardStatusTranslateList,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_standard_status(
    new_standard_status_data: IptStandardStatusTranslateListData,
    conn: &PgConnection
) -> ServiceResult<StandardStatusTranslateList> {
    use crate::schema::standard_status_translate_list::dsl as standard_status_translate_list;

    let flag_found_standard_status = standard_status_translate_list::standard_status_translate_list
        .filter(standard_status_translate_list::id_lang.eq(&new_standard_status_data.id_lang))
        .filter(standard_status_translate_list::name.eq(&new_standard_status_data.name))
        .select(standard_status_translate_list::id_standard_status)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_standard_status START SEARCH ={:?}", flag_found_standard_status);

    match flag_found_standard_status {
        0 => {
            let new_id_standard_status = {
                use crate::schema::standard_status_ref::dsl as standard_status_ref;

                let new_standard_status: StandardStatus = diesel::insert_into(standard_status_ref::standard_status_ref)
                    .default_values()
                    .get_result(conn)?;

                new_standard_status.id
            };

            let new_standard_status_data = InsertableStandardStatusTranslateList {
                id_standard_status: new_id_standard_status,
                id_lang: new_standard_status_data.id_lang,
                name: new_standard_status_data.name,
                shortname: new_standard_status_data.shortname,
            };
            let inserted_standard_status_data: StandardStatusTranslateList = diesel::insert_into(standard_status_translate_list::standard_status_translate_list)
                .values(&new_standard_status_data)
                .get_result(conn)?;
            Ok(inserted_standard_status_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This standard type name is already there. Id: {}", flag_found_standard_status))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
