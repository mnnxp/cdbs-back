use crate::errors::ServiceResult;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::schema::company_type_translate_list::dsl as company_type_translate_list;
use diesel::prelude::*;

impl CompanyTypeTranslateList {
    /// Get company typeanization by id and set lang
    pub fn get_company_type_by_id(
        target_id_company_type: &i32,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<CompanyTypeTranslateList> {
        Ok(company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::id_company_type.eq(target_id_company_type)
            .and(company_type_translate_list::id_lang.eq(set_id_lang)))
            .first::<CompanyTypeTranslateList>(conn)?)
    }

    /// Get list company typeanization by vec id and set lang
    pub fn get_company_type_by_vec_id(
        target_vec_id_company_type: &[i32],
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        Ok(company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::id_company_type.eq_any(target_vec_id_company_type)
            .and(company_type_translate_list::id_lang.eq(set_id_lang)))
            .load::<CompanyTypeTranslateList>(conn)?)
    }
}
