use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::order::Paginate;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список связанных с компанией каталогов.
pub(crate) fn get_company_specs(
    logged_user_uuid: &Uuid,
    company_uuid: &Uuid,
    set_lang_id: i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        company_uuid,
        AccessOperation::Read,
        conn,
    )?;
    SpecTranslateList::for_company_by_uuid(company_uuid, set_lang_id, paginate, conn)
}
