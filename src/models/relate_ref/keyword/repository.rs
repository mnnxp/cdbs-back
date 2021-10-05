use crate::errors::ServiceResult;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_ref::dsl as keyword_ref;
use diesel::prelude::*;

impl Keyword {
    pub fn get_keyword_by_id(
        target_keyword_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Keyword> {
        Ok(keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq(target_keyword_id))
            .first::<Keyword>(conn)?)
    }

    pub fn get_keyword_by_vec_id(
        target_keyword_ids: &[i32],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        Ok(keyword_ref::keyword_ref
            .filter(keyword_ref::id.eq_any(target_keyword_ids))
            .load::<Keyword>(conn)?)
    }
}
