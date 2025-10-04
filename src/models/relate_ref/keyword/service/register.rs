use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::model::{
    InsertableKeyword, IptKeywordData, Keyword, KeywordId,
};
use diesel::prelude::*;

/// Добавляет новое ключевое слово.
/// Возвращает ошибку с идентификатором ключевого слова, если оно уже существует.
pub(crate) fn create_keyword(
    new_keyword: &IptKeywordData,
    conn: &mut PgConnection,
) -> ServiceResult<Keyword> {
    use crate::schema::keyword_ref::dsl as keyword_ref;

    if new_keyword.keyword.len() > 100 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(100)));
    }

    let check_keyword = KeywordId::get_by_name(&new_keyword.keyword, conn);

    match check_keyword {
        Ok(x) => Err(get_err_msg(ErrorMessage::NameAlreadyThereX(
            "keyword".to_string(),
            x,
        ))),
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
        }
    }
}
