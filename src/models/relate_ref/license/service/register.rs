use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::license::model::{InsertableLicense, License, LicenseData};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_license(
    new_license_data: &LicenseData,
    conn: &PgConnection
) -> ServiceResult<License> {
    use crate::schema::license_ref::dsl::*;

    let flag_found_license = license_ref
        .filter(keyword.eq(&new_license_data.keyword))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_license START SEARCH ={:?}", flag_found_license);

    match flag_found_license {
        0 => {
            let new_license_data: InsertableLicense = new_license_data.into();
            diesel::insert_into(license_ref)
                .values(&new_license_data)
                .get_result::<License>(conn)
                .map_err(|err| {
                    debug!("Failed insert license: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This license name is already there. Id: {}", flag_found_license))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
