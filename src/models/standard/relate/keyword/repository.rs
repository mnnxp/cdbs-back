use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::search::order::Paginate;
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl Keyword {
    /// Gets all keywords for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<Keyword>> {
        let keyword_ids: Vec<i32> = keyword_to_standard::keyword_to_standard
            .filter(keyword_to_standard::standard_uuid.eq(standard_uuid))
            .select(keyword_to_standard::keyword_id)
            .limit(1000)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get keyword_to_standard ids: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if keyword_ids.is_empty() {
            return Ok(Vec::new()); // not found keywords
        }
        Keyword::get_by_ids(&keyword_ids, paginate, conn)
    }
}
