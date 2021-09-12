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
        Ok(company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::company_type_id.eq(target_company_type_id)
            .and(company_type_translate_list::lang_id.eq(set_lang_id)))
            .first::<CompanyTypeTranslateList>(conn)?)
    }

    /// Get list company typeanization by vec id and set lang
    pub fn get_company_type_by_vec_id(
        target_vec_company_type_id: &[i32],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        Ok(company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::company_type_id.eq_any(target_vec_company_type_id)
            .and(company_type_translate_list::lang_id.eq(set_lang_id)))
            .load::<CompanyTypeTranslateList>(conn)?)
    }
}
