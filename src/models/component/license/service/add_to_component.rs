use crate::errors::{ ServiceError, ServiceResult };
use crate::models::component::license::model::{
    LicenseToComponent,
    LicenseToComponentData,
    InsertableLicenseToComponent,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_license_component(
    new_license_data: LicenseToComponentData,
    // user_uuid: Uuid, todo!(access manage for owner component)
    conn: &PgConnection
) -> ServiceResult<LicenseToComponent> {
    use crate::schema::license_to_component::dsl::*;

    let flag_found_license = license_to_component
        .filter(uuid_component.eq(&new_license_data.uuid_component))
        .filter(id_license.eq(&new_license_data.id_license))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_license START SEARCH ={:?}", flag_found_license);

    match flag_found_license {
        0 => {
            let new_license_data: InsertableLicenseToComponent = new_license_data.into();
            let inserted_license_data: LicenseToComponent = diesel::insert_into(license_to_component)
                .values(&new_license_data)
                .get_result(conn)?;
            Ok(inserted_license_data)
        },
        _ => Err(ServiceError::BadRequest("This license name is already with the component.".to_string())),
    }
}
