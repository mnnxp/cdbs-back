use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::graphql::component_model::ComponentModificationAndRelatedData;
use crate::models::component::component_modification::model::ComponentModificationArg;
use crate::models::search::model::ExtraOptions;
use diesel::prelude::*;

/// Возвращает список модификаций компонента по UUID компонента.
pub(crate) fn get_component_modifications(
    args: &ComponentModificationArg,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
    require_permission(
        &options.logged_user_uuid,
        AccessEntity::Component,
        &args.component_uuid,
        AccessOperation::Read,
        conn,
    )?;
    ComponentModificationAndRelatedData::by_args(args, options.set_lang_id, conn)
}
