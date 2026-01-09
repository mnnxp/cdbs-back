use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use crate::models::standard::access::util::check_access_standard_for_user;
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
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_standard_for_user(logged_user_uuid, standard_uuid, need_access_level, conn)?;

    SpecTranslateList::for_standard_by_uuid(standard_uuid, set_lang_id, paginate, conn)
}
