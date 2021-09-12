use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::company::spec::model::{
    SpecCompany,
    IptSpecCompanyData,
    InsertableSpecCompany
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_company_spec(
    data: IptSpecCompanyData,
    conn: &PgConnection
) -> ServiceResult<SpecCompany> {
    use crate::schema::spec_to_company::dsl::*;

    let new_company_spec: InsertableSpecCompany = data.into();

    let flag_found_spec = spec_to_company
        .filter(company_uuid.eq(&new_company_spec.company_uuid)
        .and(spec_id.eq(&new_company_spec.spec_id)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_spec START SEARCH ={:?}", flag_found_spec);

    match flag_found_spec as i32 {
        0 => {
            let inserted_company_spec: SpecCompany = diesel::insert_into(spec_to_company)
                .values(&new_company_spec)
                .get_result(conn)?;
            Ok(inserted_company_spec)
        },
        _ => Err(ServiceError::BadRequest("This spec name is already with the company.".to_string())),
    }
}
