use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::language::model::Language;
use crate::models::search::order::Paginate;
use crate::schema::language_ref::dsl::*;
use diesel::{prelude::*, PgConnection};

/// Returns a list of available languages
pub(crate) fn get_languages(
    lang_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Language>> {
    match lang_ids.is_empty() {
        true => find_all_languages(paginate, conn),
        false => find_lang_iduage(lang_ids, paginate, conn),
    }
}

fn find_all_languages(
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Language>> {
    language_ref
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Language>(conn)
        .map_err(|err| {
            debug!("Failed get lang: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_lang_iduage(
    lang_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Language>> {
    language_ref
        .filter(id.eq_any(lang_ids))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Language>(conn)
        .map_err(|err| {
            debug!("Failed get lang: {:?}", err);
            ServiceError::InternalServerError
        })
}
