use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::model::DelCompanyMemberData;
use crate::models::company::access::util::check_company_access;
use crate::schema::company_member_list::dsl as company_member_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет члена компании.
/// После удаления пользователь не будет иметь доступа к закрытым объектам компании.
pub(crate) fn del_company_member(
    logged_user_uuid: &Uuid,
    data: &DelCompanyMemberData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {

    // need top level access for change component main data
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    // debug!("fn target_company_uuid = {}", &target_company_uuid);
    // debug!("fn target_user_uuid = {}", &target_user_uuid);

    // get member for delete
    let found_member_id = company_member_list::company_member_list
        .filter(company_member_list::company_uuid.eq(&data.company_uuid)
        .and(company_member_list::user_uuid.eq(&data.user_uuid)))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed get company member id: {:?}", err);
            ServiceError::InternalServerError
        })?;

    match found_member_id {
        1.. => {
            // delete member and save delete data for send response
            let result = diesel::delete(company_member_list::company_member_list
                .filter(company_member_list::company_uuid.eq(&data.company_uuid)
                .and(company_member_list::user_uuid.eq(&data.user_uuid))))
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed delete company member id: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            Ok(result == 1)
        },
        _ => Err(ServiceError::BadRequest(
            "The user not found in the company".to_string(),
        )),
    }
}
