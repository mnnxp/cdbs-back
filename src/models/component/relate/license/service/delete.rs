use crate::errors::{
    ServiceResult,
    ServiceError,
};
use crate::models::component::license::model::IptComponentLicenseData;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_license(
    logged_user_uuid: &Uuid,
    data: &IptComponentLicenseData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::license_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        true, // ownership_check
        conn
    )?;

    if data.license_id < 0 {
        // return error if not correct license id
        return Err(ServiceError::BadRequest("Error incorrect id".to_string()))
    }

    match diesel::delete(license_to_component)
        .filter(component_uuid.eq(&data.component_uuid)
        .and(license_id.eq(&data.license_id)))
        .execute(conn) {
        Ok(count) => {
            debug!("Completed, delete {:?} license", count);

            Ok(count as i32)
        },
        Err(err) => {
            debug!("Fail inserted license: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
