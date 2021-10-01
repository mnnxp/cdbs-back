use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::model::{
    CompanyMember, SlimCompanyMember, DelCompanyMemberData
};
use crate::models::company::access::util::check_company_access;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_company_member(
    logged_user_uuid: &Uuid,
    data: &DelCompanyMemberData,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyMember> {
    use crate::schema::company_member_list::dsl::*;

    // need top level access for change component main data
    let need_access_level = 1; // todo!(create enum for manage access level)

    if !check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )? {
        // return error if user not have access level
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // debug!("fn target_company_uuid = {}", &target_company_uuid);
    // debug!("fn target_user_uuid = {}", &target_user_uuid);

    // find member and check privileges for delete
    let find_member = company_member_list
        .filter(company_uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .execute(conn)
        .unwrap_or(0);

    match find_member as i32 {
        1.. => {
            // delete member and save delete data for send response
            let delete_company_member: CompanyMember =
                diesel::delete(company_member_list
                    .filter(company_uuid.eq(&data.company_uuid)
                    .and(user_uuid.eq(&data.user_uuid))))
                    .get_result(conn)?;
            // debug!("fn delete_company_member ={:?}", &delete_company_member);
            Ok(delete_company_member.into())
        },
        _ => Err(ServiceError::BadRequest(
            "The user not found in the company".to_string(),
        )),
    }
}
