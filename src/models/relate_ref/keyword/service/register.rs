use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::{
    Keyword, KeywordId, IptKeywordData, InsertableKeyword
};
use diesel::prelude::*;

pub(crate) fn create_keyword(
    new_keyword: &IptKeywordData,
    conn: &mut PgConnection
) -> ServiceResult<Keyword> {
    use crate::schema::keyword_ref::dsl as keyword_ref;

    if new_keyword.keyword.len() > 10 {
        return Err(ServiceError::BadRequest(
            "Keywords must be less than 10 symbols".to_string()
        ));
    }

    let check_keyword = KeywordId::get_by_name(&new_keyword.keyword, conn);

    match check_keyword {
        Ok(x) => Err(ServiceError::BadRequest(
            format!("This keyword name is already there. Id: {}", x))
        ),
        Err(err) => {
            debug!("Keyword not found: {:?}", err);
            let new_keyword: InsertableKeyword = new_keyword.into();

            diesel::insert_into(keyword_ref::keyword_ref)
                .values(&new_keyword)
                .get_result::<Keyword>(conn)
                .map_err(|err| {
                    debug!("Failed insert keyword: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
