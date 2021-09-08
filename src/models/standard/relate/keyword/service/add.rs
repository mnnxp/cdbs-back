use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::standard::keyword::model::{
    KeywordStandard,
    IptKeywordStandardData,
    InsertableKeywordStandard
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_standard_keyword(
    data: IptKeywordStandardData,
    conn: &PgConnection
) -> ServiceResult<KeywordStandard> {
    use crate::schema::keyword_to_standard::dsl::*;

    let new_standard_keyword: InsertableKeywordStandard = data.into();

    let flag_found_keyword = keyword_to_standard
        .filter(uuid_standard.eq(&new_standard_keyword.uuid_standard)
        .and(id_keyword.eq(&new_standard_keyword.id_keyword)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_keyword START SEARCH ={:?}", flag_found_keyword);

    match flag_found_keyword as i32 {
        0 => {
            let inserted_standard_keyword: KeywordStandard = diesel::insert_into(keyword_to_standard)
                .values(&new_standard_keyword)
                .get_result(conn)?;
            Ok(inserted_standard_keyword)
        },
        _ => Err(ServiceError::BadRequest("This keyword name is already with the standard.".to_string())),
    }
}
