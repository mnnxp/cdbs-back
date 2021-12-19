use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::license::model::{InsertableLicense, License, LicenseData};
use crate::schema::license_ref::dsl as license_ref;
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_license(
    new_license_data: &LicenseData,
    conn: &PgConnection
) -> ServiceResult<License> {
    let flag_found = license_ref::license_ref
        .filter(license_ref::keyword.eq(&new_license_data.keyword))
        .select(license_ref::id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Failed check license: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match flag_found.first() {
        Some(x) => {
            Err(ServiceError::BadRequest(format!("This license name is already there. Id: {}", x)))
        },
        None => {
            let new_license_data: InsertableLicense = new_license_data.into();
            diesel::insert_into(license_ref::license_ref)
                .values(&new_license_data)
                .get_result::<License>(conn)
                .map_err(|err| {
                    debug!("Failed insert license: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
