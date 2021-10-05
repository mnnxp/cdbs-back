use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::standard::model::DelStandardToComponentData;
use diesel::prelude::*;
use uuid::Uuid;

/// Remove related standards from component
/// delete rows in standard_to_component table
pub(crate) fn del_standards_component(
    logged_user_uuid: &Uuid,
    data: &DelStandardToComponentData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::standard_to_component::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    crate::models::component::access::util::check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    let del_count = diesel::delete(standard_to_component
        .filter(component_uuid.eq(&data.component_uuid)
        .and(standard_uuid.eq_any(&data.standards_uuids))))
        .execute(conn);

    match del_count {
        Ok(count) => Ok(count as i32),
        Err(err) => {
            debug!("Failed delete related standards to component: {:?}", err);
            Err(ServiceError::BadRequest("Failed delete related standards to component".to_string()))
        },
    }
}
