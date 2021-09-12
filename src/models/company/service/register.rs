use crate::errors::ServiceResult;
use crate::models::company::model::{
    IptCompanyData,
    InsertableCompany,
    SlimCompany,
    Company,
    CompanyData,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_company(
    logged_user_uuid: Uuid,
    data: IptCompanyData,
    conn: &PgConnection
) -> ServiceResult<SlimCompany> {
    use crate::schema::company_ref::dsl::company_ref;

    let target_image_file_uuid = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    let company_data = CompanyData {
        orgname: (data.orgname),
        shortname: (data.shortname),
        inn: (data.inn),
        phone: (data.phone),
        email: (data.email),
        description: (data.description),
        address: (data.address),
        site_url: (data.site_url),
        time_zone: (data.time_zone),
        user_uuid: (logged_user_uuid),
        image_file_uuid: (target_image_file_uuid),
        region_id: (data.region_id),
        company_type_id: (data.company_type_id),
    };

    let company: InsertableCompany = company_data.into();
    let inserted_company: Company = diesel::insert_into(company_ref).values(&company).get_result(conn)?;
    Ok(inserted_company.into())
}
