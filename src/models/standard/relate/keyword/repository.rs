use crate::errors::ServiceResult;
use crate::models::standard::model::Standard;
use crate::models::standard::keyword::model::KeywordStandard;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;

impl Keyword {
    /// Get list keywords for standard
    pub fn get_by_standard(
        standard: &Standard,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        let target_vec_id_keyword: Vec<i32> = KeywordStandard::belonging_to(standard)
            .select(keyword_to_standard::id_keyword)
            .load::<i32>(conn)?;
        Keyword::get_keyword_by_vec_id(&target_vec_id_keyword, conn)
    }
}
