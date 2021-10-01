use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::member::model::{
    CompanyMember, IptCompanyMemberData, InsertableCompanyMember, SlimCompanyMember,
};
use crate::models::company::access::util::check_company_access;
use crate::models::company::member::role::util::check_role_of_company;
use diesel::prelude::*;
use uuid::Uuid;

pub fn add_company_member(
    logged_user_uuid: &Uuid,
    data: &IptCompanyMemberData,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyMember> {
    use crate::schema::company_member_list::dsl::company_member_list;

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

    check_role_of_company(
        &data.company_uuid,
        &data.role_id,
        conn
    )?;

    let company_member: InsertableCompanyMember = data.into();
    let inserted_company_member: CompanyMember = diesel::insert_into(company_member_list)
        .values(&company_member)
        .get_result(conn)?;
    Ok(inserted_company_member.into())
}
