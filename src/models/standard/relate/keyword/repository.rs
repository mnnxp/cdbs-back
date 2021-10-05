use crate::errors::ServiceResult;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl Keyword {
    /// Gets all keywords for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        let target_keyword_ids: Vec<i32> = keyword_to_standard::keyword_to_standard
            .filter(keyword_to_standard::standard_uuid.eq(standard_uuid))
            .select(keyword_to_standard::keyword_id)
            .load::<i32>(conn)?;

        Keyword::get_keyword_by_vec_id(&target_keyword_ids, conn)
    }
}
