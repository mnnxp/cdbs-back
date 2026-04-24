use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::component::standard::model::DelStandardToComponentData;
use diesel::prelude::*;
use uuid::Uuid;

/// Открепляет стандарт от компонента.
pub(crate) fn del_standards_component(
    logged_user_uuid: &Uuid,
    data: &DelStandardToComponentData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    use crate::schema::standard_to_component::dsl::*;
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &data.component_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    let del_count = diesel::delete(
        standard_to_component.filter(
            component_uuid
                .eq(&data.component_uuid)
                .and(standard_uuid.eq_any(&data.standards_uuids)),
        ),
    )
    .execute(conn);

    match del_count {
        Ok(count) => Ok(count),
        Err(err) => {
            debug!("Failed delete related standards to component: {:?}", err);
            Err(get_err_msg(
                ErrorMessage::FailedDeleteRelatedStandardsComponent,
            ))
        }
    }
}
