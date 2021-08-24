use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::actual_status::model::{
    ActualStatus,
    ActualStatusTranslateList,
    IptActualStatusTranslateListData,
    InsertableActualStatusTranslateList,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_actual_status(
    new_actual_status_data: IptActualStatusTranslateListData,
    conn: &PgConnection
) -> ServiceResult<ActualStatusTranslateList> {
    use crate::schema::actual_status_translate_list::dsl as actual_status_translate_list;

    let flag_found_actual_status = actual_status_translate_list::actual_status_translate_list
        .filter(actual_status_translate_list::id_lang.eq(&new_actual_status_data.id_lang))
        .filter(actual_status_translate_list::name.eq(&new_actual_status_data.name))
        .select(actual_status_translate_list::id_actual_status)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_actual_status START SEARCH ={:?}", flag_found_actual_status);

    match flag_found_actual_status {
        0 => {
            let new_id_actual_status = {
                use crate::schema::actual_status_ref::dsl as actual_status_ref;

                let new_actual_status: ActualStatus = diesel::insert_into(actual_status_ref::actual_status_ref)
                    .default_values()
                    .get_result(conn)?;

                new_actual_status.id
            };

            let new_actual_status_data = InsertableActualStatusTranslateList {
                id_actual_status: new_id_actual_status,
                id_lang: new_actual_status_data.id_lang,
                name: new_actual_status_data.name,
            };
            let inserted_actual_status_data: ActualStatusTranslateList = diesel::insert_into(actual_status_translate_list::actual_status_translate_list)
                .values(&new_actual_status_data)
                .get_result(conn)?;
            Ok(inserted_actual_status_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This actual_status name is already there. Id: {}", flag_found_actual_status))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
