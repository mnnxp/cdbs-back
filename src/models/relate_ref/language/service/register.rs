// use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::models::relate_ref::language::model::{
    InsertableLanguage,
    LanguageData,
    Language
};
use diesel::prelude::*;

pub(crate) fn create_language(
    new_language_data: LanguageData,
    conn: &PgConnection
) -> ServiceResult<Language> {
    use crate::schema::language_ref::dsl::*;

    let new_language_data: InsertableLanguage = new_language_data.into();

    let inserted_language_data: Language = diesel::insert_into(language_ref)
        .values(&new_language_data)
        .get_result(conn)?;
    Ok(inserted_language_data)
}
