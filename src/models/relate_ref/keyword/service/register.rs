use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::{
    Keyword, IptKeywordData, InsertableKeyword
};
use diesel::prelude::*;

pub(crate) fn create_keyword(
    new_keyword_data: &IptKeywordData,
    conn: &PgConnection
) -> ServiceResult<Keyword> {
    use crate::schema::keyword_ref::dsl::*;

    let new_keyword_data: InsertableKeyword = new_keyword_data.into();

    let flag_found_keyword = keyword_ref
        .filter(keyword.eq(&new_keyword_data.keyword))
        .select(id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_keyword START SEARCH ={:?}", flag_found_keyword);

    match flag_found_keyword {
        0 => {
            diesel::insert_into(keyword_ref)
                .values(&new_keyword_data)
                .get_result::<Keyword>(conn)
                .map_err(|err| {
                    debug!("Failed insert keyword: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This keyword name is already there. Id: {}", flag_found_keyword))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
