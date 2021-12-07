use crate::errors::ServiceResult;
use crate::models::standard::spec::model::StandardSpecsArg;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::prelude::*;
use uuid::Uuid;

/// Get all specs for standard
pub(crate) fn get_standard_specs(
    logged_user_uuid: &Uuid,
    arg: &StandardSpecsArg,
    set_lang_id: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<SpecTranslateList>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &arg.standard_uuid,
        &need_access_level,
        conn
    )?;

    SpecTranslateList::get_by_uuid(
        arg,
        set_lang_id,
        conn
    )
}
