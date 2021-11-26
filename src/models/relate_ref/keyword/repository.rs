use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_ref::dsl as keyword_ref;
use diesel::prelude::*;

impl Keyword {
    pub(crate) fn get_by_ids(
        target_keyword_ids: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq_any(target_keyword_ids))
            .load::<Keyword>(conn)
            .map_err(|err| {
                debug!("Failed get keyword: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}
