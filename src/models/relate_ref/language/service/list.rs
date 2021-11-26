use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::language::model::Language;
use crate::schema::language_ref::dsl::*;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_languages(
    target_lang_iduage: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Language>> {
    match target_lang_iduage.is_empty() {
        true => find_all_languages(limit, offset, conn),
        false => find_lang_iduage(target_lang_iduage, limit, offset, conn)
    }
}

fn find_all_languages(
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Language>> {
    language_ref
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Language>(conn)
        .map_err(|err| {
            debug!("Failed get lang: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_lang_iduage(
    target_lang_iduage: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Language>> {
    language_ref
        .filter(id.eq_any(target_lang_iduage))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Language>(conn)
        .map_err(|err| {
            debug!("Failed get lang: {:?}", err);
            ServiceError::InternalServerError
        })
}
