use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::relate_ref::keyword::model::{
    Keyword, KeywordId, IptKeywordData, InsertableKeyword
};
use diesel::prelude::*;

/// Добавляет новое ключевое слово.
/// Возвращает ошибку с идентификатором ключевого слова, если оно уже существует.
pub(crate) fn create_keyword(
    new_keyword: &IptKeywordData,
    conn: &mut PgConnection
) -> ServiceResult<Keyword> {
    use crate::schema::keyword_ref::dsl as keyword_ref;

    if new_keyword.keyword.len() > 30 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(30)));
    }

    let check_keyword = KeywordId::get_by_name(&new_keyword.keyword, conn);

    match check_keyword {
        Ok(x) => Err(get_err_msg(ErrorMessage::NameAlreadyThereX("keyword".to_string(), x))),
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
