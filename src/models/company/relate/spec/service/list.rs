use crate::errors::ServiceResult;
use crate::models::company::{
    spec::model::CompanySpecsArg,
    access::util::check_company_access,
};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список связанных с компанией каталогов.
pub(crate) fn get_company_specs(
    logged_user_uuid: &Uuid,
    arg: &CompanySpecsArg,
    set_lang_id: &i32,
    conn: &mut PgConnection
) -> ServiceResult<Vec<SpecTranslateList>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &arg.company_uuid,
        &need_access_level,
        conn
    )?;

    SpecTranslateList::for_company_by_uuid(
        &arg.company_uuid,
        set_lang_id,
        &Paginate::parsing(arg.limit, arg.offset),
        conn
    )
}
