use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::config;
use diesel::prelude::*;
use uuid::Uuid;

/// Retund default component modification uuid
pub(crate) fn get_root_modification_uuid() -> Uuid {
    config::root_modification_uuid()
}

/// Get component uuid from modification by uuid
pub(crate) fn get_component_by_modification(
    target_modification_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    use crate::schema::component_modification_list::dsl::*;

    component_modification_list
        .filter(uuid.eq(target_modification_uuid))
        .select(component_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get_component_by_modification data: {:?}", err);
            get_err_msg(ErrorMessage::NotFoundModificationData)
        })
}
