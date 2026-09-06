use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::{model::ExtraOptions, order::Paginate};
use diesel::prelude::*;
use uuid::Uuid;

/// Returns an array of directory sections associated with the service
pub(crate) fn get_service_specs(
    service_uuid: &Uuid,
    options: &ExtraOptions,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    require_permission(
        &options.logged_user_uuid,
        AccessEntity::Service,
        service_uuid,
        AccessOperation::Write,
        conn,
    )?;

    SpecTranslateList::for_service_by_uuid(service_uuid, options.set_lang_id, paginate, conn)
}
