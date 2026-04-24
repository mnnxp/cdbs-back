use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::models::company::member::model::CompanyMemberAndRelatedData;
use diesel::PgConnection;
use uuid::Uuid;

/// Возвращает агрегированные данные об участниках компании (сообщества).
pub(crate) fn get_by_company_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    set_lang_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        target_company_uuid,
        AccessOperation::Read,
        conn,
    )?;

    let result: Vec<CompanyMemberAndRelatedData> =
        CompanyMemberAndRelatedData::get_list_members_by_company_uuid(
            target_company_uuid,
            set_lang_id,
            conn,
        )
        .expect("Error loading list companies and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}
