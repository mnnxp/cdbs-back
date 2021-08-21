use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::actual_status::model::{
    InsertableActualStatus, ActualStatus, IptActualStatusData
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_actual_status(
    new_actual_status_data: IptActualStatusData,
    conn: &PgConnection
) -> ServiceResult<ActualStatus> {
    use crate::schema::actual_status_ref::dsl::*;
    // use crate::schema::actual_status_to_component::dsl::uuid as uuid_component;
    // use crate::schema::actual_status_to_modification::dsl::uuid as uuid_modification;
    // use diesel::dsl::count;

    let flag_found_actual_status = actual_status_ref
        .filter(id_lang.eq(&new_actual_status_data.id_lang))
        .filter(name.eq(&new_actual_status_data.name))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_actual_status START SEARCH ={:?}", flag_found_actual_status);

    match flag_found_actual_status {
        0 => {
            let new_actual_status_data: InsertableActualStatus = new_actual_status_data.into();
            let inserted_actual_status_data: ActualStatus = diesel::insert_into(actual_status_ref)
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
