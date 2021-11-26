use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_ref::dsl::*;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_keywords(
    target_keyword_id: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    match target_keyword_id.is_empty() {
        true => find_all_keywords(limit, offset, conn),
        false => find_keyword_ids(target_keyword_id, limit, offset, conn)
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
    target_keyword_id: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    keyword_ref
        .filter(id.eq_any(target_keyword_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Keyword>(conn)
        .map_err(|err| {
            debug!("Failed get keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
