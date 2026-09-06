use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::{model::ExtraOptions, order::Paginate};
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the catalogs associated with the component
pub(crate) fn get_component_specs(
    component_uuid: &Uuid,
    options: &ExtraOptions,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    require_permission(
        &options.logged_user_uuid,
        AccessEntity::Component,
        component_uuid,
        AccessOperation::Read,
        conn,
    )?;

    SpecTranslateList::for_component_by_uuid(component_uuid, options.set_lang_id, paginate, conn)
}
