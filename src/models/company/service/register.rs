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
    logged_uuid_user: Uuid,
    data: IptCompanyData,
    conn: &PgConnection
) -> ServiceResult<SlimCompany> {
    use crate::schema::company_ref::dsl::company_ref;

    let target_uuid_image_file = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

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
        uuid_user: (logged_uuid_user),
        uuid_image_file: (target_uuid_image_file),
        id_region: (data.id_region),
        id_company_type: (data.id_company_type),
    };

    let company: InsertableCompany = company_data.into();
    let inserted_company: Company = diesel::insert_into(company_ref).values(&company).get_result(conn)?;
    Ok(inserted_company.into())
}
