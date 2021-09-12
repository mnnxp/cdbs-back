use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::license::model::{
    LicenseComponent,
    IptLicenseComponentData,
    InsertableLicenseComponent,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_license_component(
    new_license_data: IptLicenseComponentData,
    // user_uuid: Uuid, todo!(access manage for owner component)
    conn: &PgConnection
) -> ServiceResult<LicenseComponent> {
    use crate::schema::license_to_component::dsl::*;

    let new_license_data: InsertableLicenseComponent = new_license_data.into();

    let flag_found_license = license_to_component
        .filter(component_uuid.eq(&new_license_data.component_uuid))
        .filter(license_id.eq(&new_license_data.license_id))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_license START SEARCH ={:?}", flag_found_license);

    match flag_found_license as i32 {
        0 => {
            let inserted_license_data: LicenseComponent = diesel::insert_into(license_to_component)
                .values(&new_license_data)
                .get_result(conn)?;
            Ok(inserted_license_data)
        },
        _ => Err(ServiceError::BadRequest("This license name is already with the component.".to_string())),
    }
}
