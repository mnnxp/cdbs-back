use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::{Keyword, KeywordId};
use crate::models::search::order::Paginate;
use crate::schema::keyword_ref::dsl as keyword_ref;
use diesel::prelude::*;

impl Keyword {
    pub(crate) fn get_by_ids(
        target_keyword_ids: &[i32],
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq_any(target_keyword_ids))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .order(keyword_ref::id.asc())
            .load::<Keyword>(conn)
            .map_err(|err| {
                debug!("Failed get keyword: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl KeywordId {
    pub(crate) fn get_by_name(
        keyword: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        keyword_ref::keyword_ref
            .filter(keyword_ref::keyword.eq(keyword))
            .select(keyword_ref::id)
            .first::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get keyword: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
