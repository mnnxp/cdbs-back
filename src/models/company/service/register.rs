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
    logged_user_uuid: &Uuid,
    data: &IptCompanyData,
    conn: &PgConnection
) -> ServiceResult<SlimCompany> {
    use crate::schema::company_ref::dsl::company_ref;

    let target_image_file_uuid = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    let company_data = CompanyData {
        orgname: data.orgname.to_string(),
        shortname: data.shortname.to_string(),
        inn: data.inn.to_string(),
        phone: data.phone.to_string(),
        email: data.email.to_string(),
        description: data.description.to_string(),
        address: data.address.to_string(),
        site_url: data.site_url.to_string(),
        time_zone: data.time_zone.to_string(),
        user_uuid: *logged_user_uuid,
        image_file_uuid: target_image_file_uuid,
        region_id: data.region_id,
        company_type_id: data.company_type_id,
    };

    let company: InsertableCompany = company_data.into();
    let inserted_company: Company = diesel::insert_into(company_ref).values(&company).get_result(conn)?;
    Ok(inserted_company.into())
}
