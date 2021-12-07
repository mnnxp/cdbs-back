use crate::errors::ServiceResult;
use crate::models::company::{
    spec::model::CompanySpecsArg,
    access::util::check_company_access,
};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use diesel::prelude::*;
use uuid::Uuid;

/// Get all specs for company
pub(crate) fn get_company_specs(
    logged_user_uuid: &Uuid,
    arg: &CompanySpecsArg,
    set_lang_id: &i32,
    conn: &PgConnection
) -> ServiceResult<Vec<SpecTranslateList>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &arg.company_uuid,
        &need_access_level,
        conn
    )?;

    SpecTranslateList::for_company_by_uuid(
        arg,
        set_lang_id,
        conn
    )
}
