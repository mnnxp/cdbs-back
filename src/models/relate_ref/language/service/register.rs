use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::language::model::{
    InsertableLanguage,
    LanguageData,
    Language
};
use diesel::prelude::*;

pub(crate) fn create_language(
    new_language_data: &LanguageData,
    conn: &mut PgConnection
) -> ServiceResult<Language> {
    use crate::schema::language_ref::dsl::*;

    let new_language_data: InsertableLanguage = new_language_data.into();

    diesel::insert_into(language_ref)
        .values(&new_language_data)
        .get_result::<Language>(conn)
        .map_err(|err| {
            debug!("Failed insert language: {:?}", err);
            ServiceError::InternalServerError
        })
}
