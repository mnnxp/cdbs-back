use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::member::model::CompanyMemberAndRelatedData;
use crate::models::company::access::util::check_company_access;
use diesel::PgConnection;
use uuid::Uuid;

/// Возвращает агрегированные данные об участниках компании (сообщества).
pub(crate) fn get_by_company_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    if !check_company_access(
        logged_user_uuid,
        target_company_uuid,
        &need_access_level,
        conn,
    )? {
        // return error if user not have access level
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    let result: Vec<CompanyMemberAndRelatedData> = CompanyMemberAndRelatedData::get_list_members_by_company_uuid(
        target_company_uuid,
        set_lang_id,
        conn
    ).expect("Error loading list companies and collect short data");

    debug!("Components data: {:#?}", result);

    Ok(result)
}

// /// Search company members by company uuid
// /// Optional filter by user uuid
// pub(crate) fn get_member_by_uuids(
//     logged_user_uuid: &Uuid,
//     target_company_uuid: &Uuid,
//     members_uuids: &[Uuid],
//     set_lang_id: &i32,
//     conn: &mut PgConnection,
// ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
//
//     let need_access_level = 3; // todo!(create enum for manage access level)
//
//     if !check_company_access(
//         logged_user_uuid,
//         target_company_uuid,
//         &need_access_level,
//         conn,
//     )? {
//         // return error if user not have access level
//         return Err(ServiceError::BadRequest("Access denied".to_string()))
//     }
//
//     // collect data for member
//     let result: Vec<CompanyMemberAndRelatedData> =
//         CompanyMemberAndRelatedData::_get_company_members_by_uuid(
//             target_company_uuid,
//             members_uuids,
//             set_lang_id,
//             conn
//         ).expect("Error loading company and collect related data");
//
//     debug!("Component data: {:#?}", result);
//
//     Ok(result)
// }
