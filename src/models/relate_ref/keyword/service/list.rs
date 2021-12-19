use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::{
    Keyword, KeywordArg
};
use crate::schema::keyword_ref::dsl::*;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_keywords(
    args: &KeywordArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    match args.keyword_ids.is_empty() {
        true => find_all_keywords(&args.limit, &args.offset, conn),
        false => find_keyword_ids(args, conn)
    }
}

fn find_all_keywords(
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    keyword_ref
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Keyword>(conn)
        .map_err(|err| {
            debug!("Failed get keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_keyword_ids(
    args: &KeywordArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    keyword_ref
        .filter(id.eq_any(&args.keyword_ids))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Keyword>(conn)
        .map_err(|err| {
            debug!("Failed get keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
