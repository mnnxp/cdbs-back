use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::language::model::{
    Language, LanguageArg
};
use crate::schema::language_ref::dsl::*;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_languages(
    args: &LanguageArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Language>> {
    match args.lang_ids.is_empty() {
        true => find_all_languages(&args.limit, &args.offset, conn),
        false => find_lang_iduage(args, conn)
    }
}

fn find_all_languages(
    limit: &i32,
    offset: &i32,
    conn: &mut PgConnection,
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
    args: &LanguageArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Language>> {
    language_ref
        .filter(id.eq_any(&args.lang_ids))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Language>(conn)
        .map_err(|err| {
            debug!("Failed get lang: {:?}", err);
            ServiceError::InternalServerError
        })
}
