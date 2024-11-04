use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the catalogs associated with the component
pub(crate) fn get_component_specs(
    logged_user_uuid: &Uuid,
    component_uuid: &Uuid,
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection
) -> ServiceResult<Vec<SpecTranslateList>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        component_uuid,
        &need_access_level,
        conn
    )?;

    SpecTranslateList::for_component_by_uuid(
        component_uuid,
        set_lang_id,
        paginate,
        conn
    )
}
