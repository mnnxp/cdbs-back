use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::schema::company_type_translate_list::dsl as company_type_translate_list;
use diesel::prelude::*;

impl CompanyTypeTranslateList {
    /// Get company type by id
    pub(crate) fn get_company_type_by_id(
        target_company_type_id: i32,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<CompanyTypeTranslateList> {
        company_type_translate_list::company_type_translate_list
            .filter(
                company_type_translate_list::company_type_id
                    .eq(target_company_type_id)
                    .and(company_type_translate_list::lang_id.eq(set_lang_id)),
            )
            .first::<CompanyTypeTranslateList>(conn)
            .map_err(|err| {
                // if not found data for set lang
                debug!("Not found set lang for company type: {:?}", err);
                company_type_translate_list::company_type_translate_list
                    .filter(
                        company_type_translate_list::company_type_id
                            .eq(target_company_type_id)
                            .and(company_type_translate_list::lang_id.eq(1)),
                    )
                    .first::<CompanyTypeTranslateList>(conn)
            })
            .map_err(|err| {
                debug!("Failed get company type: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Get all company types with translate
    pub(crate) fn get_company_types(
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        company_type_translate_list::company_type_translate_list
            .filter(company_type_translate_list::lang_id.eq(set_lang_id))
            .order(company_type_translate_list::name.asc())
            .load::<CompanyTypeTranslateList>(conn)
            .map_err(|err| {
                // if not found data for set lang
                debug!("Not found set lang for company types: {:?}", err);
                company_type_translate_list::company_type_translate_list
                    .filter(company_type_translate_list::lang_id.eq(1))
                    .load::<CompanyTypeTranslateList>(conn)
            })
            .map_err(|err| {
                debug!("Failed get company types: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
