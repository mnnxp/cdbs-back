use crate::errors::ServiceResult;
use crate::graphql::component_model::ComponentModificationAndRelatedData;
use crate::models::component::{
    component_modification::model::ComponentModificationArg,
    access::util::check_access_component_for_user,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список модификаций компонента по UUID компонента.
pub(crate) fn get_component_modifications(
    logged_user_uuid: &Uuid,
    args: &ComponentModificationArg,
    set_lang_id: &i32,
    conn: &mut PgConnection
) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &args.component_uuid,
        &need_access_level,
        conn
    )?;

    ComponentModificationAndRelatedData::by_args(
        &args.component_uuid,
        &args.sort,
        &args.paginate,
        set_lang_id,
        conn
    )
}
