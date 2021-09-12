use crate::errors::{ ServiceError, ServiceResult };
use crate::models::company::company_type::model::{
    CompanyType,
    CompanyTypeTranslateList,
    IptCompanyTypeTranslateListData,
    InsertableCompanyTypeTranslateList,
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_company_type(
    new_company_type_data: IptCompanyTypeTranslateListData,
    conn: &PgConnection
) -> ServiceResult<CompanyTypeTranslateList> {
    use crate::schema::company_type_translate_list::dsl as company_type_translate_list;

    let flag_found_company_type = company_type_translate_list::company_type_translate_list
        .filter(company_type_translate_list::lang_id.eq(&new_company_type_data.lang_id))
        .filter(company_type_translate_list::name.eq(&new_company_type_data.name))
        .select(company_type_translate_list::company_type_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_company_type START SEARCH ={:?}", flag_found_company_type);

    match flag_found_company_type {
        0 => {
            let new_company_type_id = {
                use crate::schema::company_type_ref::dsl as company_type_ref;

                let new_company_type: CompanyType = diesel::insert_into(company_type_ref::company_type_ref)
                    .default_values()
                    .get_result(conn)?;

                new_company_type.id
            };

            let new_company_type_data = InsertableCompanyTypeTranslateList {
                company_type_id: new_company_type_id,
                lang_id: new_company_type_data.lang_id,
                name: new_company_type_data.name,
                shortname: new_company_type_data.shortname,
            };
            let inserted_company_type_data: CompanyTypeTranslateList = diesel::insert_into(company_type_translate_list::company_type_translate_list)
                .values(&new_company_type_data)
                .get_result(conn)?;
            Ok(inserted_company_type_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This company type name is already there. Id: {}", flag_found_company_type))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
