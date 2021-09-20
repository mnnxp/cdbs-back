use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::license::model::{
    ComponentLicense,
    IptComponentLicenseData,
    InsertableComponentLicense,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_component_license(
    new_license_data: IptComponentLicenseData,
    // user_uuid: Uuid, todo!(access manage for owner component)
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::license_to_component::dsl::*;

    if new_license_data.license_id < 0 {
        return Err(ServiceError::BadRequest("Error incorrect id".to_string()))
    }

    let new_license_data: InsertableComponentLicense = new_license_data.into();

    let flag_found_license = license_to_component
        .filter(component_uuid.eq(&new_license_data.component_uuid)
        .and(license_id.eq(&new_license_data.license_id)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_license START SEARCH ={:?}", flag_found_license);

    if flag_found_license != 0 {
        return Err(ServiceError::BadRequest("This license for the component is already".to_string()))
    }

    match diesel::insert_into(license_to_component)
        .values(&new_license_data)
        .get_result::<ComponentLicense>(conn) {
        Ok(row) => {
            debug!("Completed, add license for component: {:?}", row);
            Ok(true)
        },
        Err(err) => {
            debug!("Fail add license for component: {:?}", err);
            Err(ServiceError::InternalServerError)
        },
    }
}
