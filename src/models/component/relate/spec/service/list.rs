use crate::errors::ServiceResult;
use crate::models::component::{
    spec::model::ComponentSpecsArg,
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает массив разделов каталога, связанных с компонентом.
pub(crate) fn get_component_specs(
    logged_user_uuid: &Uuid,
    arg: &ComponentSpecsArg,
    set_lang_id: &i32,
    conn: &mut PgConnection
) -> ServiceResult<Vec<SpecTranslateList>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &arg.component_uuid,
        &need_access_level,
        conn
    )?;

    SpecTranslateList::for_component_by_uuid(
        arg,
        set_lang_id,
        conn
    )
}
