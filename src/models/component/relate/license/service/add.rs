use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::license::model::{
    ComponentLicense,
    IptComponentLicenseData,
    InsertableComponentLicense,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_component_license(
    logged_user_uuid: &Uuid,
    data: &IptComponentLicenseData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::license_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    if data.license_id < 0 {
        return Err(ServiceError::BadRequest("Error incorrect id".to_string()))
    }

    let data: InsertableComponentLicense = data.into();

    let flag_found_license = license_to_component
        .filter(component_uuid.eq(&data.component_uuid)
        .and(license_id.eq(&data.license_id)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_license START SEARCH ={:?}", flag_found_license);

    if flag_found_license != 0 {
        return Err(ServiceError::BadRequest("This license for the component is already".to_string()))
    }

    match diesel::insert_into(license_to_component)
        .values(&data)
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
