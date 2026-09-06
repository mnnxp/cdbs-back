use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns an array of directory sections associated with the standard
pub(crate) fn get_standard_specs(
    logged_user_uuid: &Uuid,
    standard_uuid: &Uuid,
    set_lang_id: i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Standard,
        standard_uuid,
        AccessOperation::Read,
        conn,
    )?;

    SpecTranslateList::for_standard_by_uuid(standard_uuid, set_lang_id, paginate, conn)
}
