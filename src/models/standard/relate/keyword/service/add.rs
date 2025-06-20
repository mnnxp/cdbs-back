use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, KeywordId},
    service::register::create_keyword,
};
use crate::models::standard::{
    access::util::check_access_standard_for_user,
    keyword::model::{
        InsertableStandardKeyword, IptStandardKeywordsData, IptStandardKeywordsNames,
    },
};
use crate::schema::keyword_to_standard::dsl as keyword_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет ключевые слова к стандарту по идентификаторам.
pub(crate) fn add_standard_keywords(
    logged_user_uuid: &Uuid,
    data: &IptStandardKeywordsData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn,
    )?;

    // creating structures for inserting records into a table
    let mut keywords: Vec<InsertableStandardKeyword> = data.into();

    clear_duplicates(&mut keywords);

    match keywords.is_empty() {
        true => Err(get_err_msg(ErrorMessage::NotFoundKeywords)),
        false => {
            keywords.retain(|k| check_keyword_for_standard(k, conn));
            insert_rows_standard_keywords(&keywords, conn)
        }
    }
}

/// Check already keyword for standard (duplicate)
fn check_keyword_for_standard(
    keyword: &InsertableStandardKeyword,
    conn: &mut PgConnection,
) -> bool {
    let check = keyword_to_standard::keyword_to_standard
        .filter(
            keyword_to_standard::standard_uuid
                .eq(&keyword.standard_uuid)
                .and(keyword_to_standard::keyword_id.eq(&keyword.keyword_id)),
        )
        .limit(1)
        .execute(conn);
    debug!("Check: {:?}", check);
    matches!(check, Ok(x) if x == 0)
}

fn insert_rows_standard_keywords(
    insert_data: &[InsertableStandardKeyword],
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    diesel::insert_into(keyword_to_standard::keyword_to_standard)
        .values(insert_data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Добавляет ключевые слова к стандарту по словам.
pub(crate) fn add_keywords_by_names(
    logged_user_uuid: &Uuid,
    data: &IptStandardKeywordsNames,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let mut keyword_ids: Vec<i32> = Vec::new();

    for kw in &data.keywords {
        match KeywordId::get_by_name(kw, conn) {
            Ok(x) => keyword_ids.push(x),
            Err(err) => {
                debug!("Error ({:?}) for keyword: {:?}", err, kw);
                let keyword = create_keyword(
                    &IptKeywordData {
                        keyword: kw.clone(),
                    },
                    conn,
                )?;
                keyword_ids.push(keyword.id);
            }
        }
    }

    add_standard_keywords(
        logged_user_uuid,
        &IptStandardKeywordsData {
            keyword_ids,
            standard_uuid: data.standard_uuid,
        },
        conn,
    )
}

/// Clear duplicates keywords
fn clear_duplicates(keywords: &mut Vec<InsertableStandardKeyword>) {
    let mut already_seen = Vec::new();
    keywords.retain(|item| match already_seen.contains(&item.keyword_id) {
        true => false,
        _ => {
            already_seen.push(item.keyword_id);
            true
        }
    })
}
