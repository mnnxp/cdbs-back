use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::standard::model::{
    StandardToComponent,
    IptStandardToComponentData,
    InsertableStandardToComponent
};
use diesel::prelude::*;
// use uuid::Uuid;

/// Add related standard for component
/// insert row in standard_to_component table
pub(crate) fn add_standard_to_component(
    data: &IptStandardToComponentData,
    conn: &PgConnection
) -> ServiceResult<bool> {
    use crate::schema::standard_to_component::dsl::*;

    let found_standard = standard_to_component
        .filter(component_uuid.eq(data.component_uuid)
        .and(standard_uuid.eq(data.standard_uuid)))
        .execute(conn);

    match found_standard {
        Ok(found) => {
            if found > 0 {
                return Err(ServiceError::BadRequest(
                    "This standard is already associated with the component".to_string()
                ))
            }

            let new_component_standard: InsertableStandardToComponent = data.into();

            match diesel::insert_into(standard_to_component)
                .values(&new_component_standard)
                .get_result::<StandardToComponent>(conn) {
                Ok(_) => Ok(true),
                Err(err) => {
                    debug!("Failed add standard component: {:?}", err);
                    Err(ServiceError::BadRequest("Failed add standard component".to_string()))
                },
            }
        },
        Err(err) => {
            debug!("Failed check standards component: {:?}", err);
            Err(ServiceError::BadRequest("Failed check standards component".to_string()))
        },
    }
}
