use crate::errors::ServiceResult;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::schema::company_type_translate_list::dsl as company_type_translate_list;
use diesel::prelude::*;

impl CompanyTypeTranslateList {
    /// Get company typeanization by id and set lang
    pub fn get_company_type_by_id(
        target_company_type_id: &i32,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<CompanyTypeTranslateList> {
        let company_type = company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::company_type_id.eq(target_company_type_id)
            .and(company_type_translate_list::lang_id.eq(set_lang_id)))
            .first::<CompanyTypeTranslateList>(conn);

        // if not found data for set lang
        match company_type {
            Ok(cy_type) => Ok(cy_type),
            Err(err) => {
                debug!("Not found set lang for company type: {:?}", err);
                Ok(company_type_translate_list::company_type_translate_list
                    .filter(company_type_translate_list::company_type_id.eq(target_company_type_id))
                    .first::<CompanyTypeTranslateList>(conn)?)
            },
        }
    }

    /// Get list company typeanization by vec id and set lang
    pub fn get_company_type_by_vec_id(
        target_vec_company_type_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        let companies_types = company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::company_type_id.eq_any(target_vec_company_type_id)
            .and(company_type_translate_list::lang_id.eq(set_lang_id)))
            .load::<CompanyTypeTranslateList>(conn);

        // if not found data for set lang
        match companies_types {
            Ok(cs_types) => Ok(cs_types),
            Err(err) => {
                debug!("Not found set lang for companies types: {:?}", err);
                Ok(company_type_translate_list::company_type_translate_list
                    .filter(company_type_translate_list::company_type_id.eq_any(target_vec_company_type_id))
                    .load::<CompanyTypeTranslateList>(conn)?)
            },
        }
    }
}
