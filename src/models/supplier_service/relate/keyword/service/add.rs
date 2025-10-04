use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, KeywordId},
    service::register::create_keyword,
};
use crate::models::supplier_service::service::update::change_service_updated_at;
use crate::models::supplier_service::{
    access::util::check_access_service_for_user,
    keyword::model::{InsertableServiceKeyword, IptServiceKeywordsData, IptServiceKeywordsNames},
};
use crate::schema::keyword_to_service::dsl as keyword_to_service;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет ключевые слова к компоненту по идентификаторам.
pub(crate) fn add_service_keywords(
    data: &IptServiceKeywordsData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_service_for_user(
        logged_user_uuid,
        &data.service_uuid,
        &need_access_level,
        conn,
    )?;

    // creating structures for inserting records into a table
    let mut keywords: Vec<InsertableServiceKeyword> = data.into();

    clear_duplicates(&mut keywords);

    match keywords.is_empty() {
        true => Err(get_err_msg(ErrorMessage::NotFoundKeywords)),
        false => {
            keywords.retain(|k| check_keyword_for_service(k, conn));
            change_service_updated_at(
                &data.service_uuid,
                logged_user_uuid,
                format!("Added new keywords: {:?})", &keywords),
                conn,
            )?;
            insert_rows_service_keywords(&keywords, conn)
        }
    }
}

/// Check already keyword for service (duplicate)
fn check_keyword_for_service(keyword: &InsertableServiceKeyword, conn: &mut PgConnection) -> bool {
    let check = keyword_to_service::keyword_to_service
        .filter(
            keyword_to_service::service_uuid
                .eq(&keyword.service_uuid)
                .and(keyword_to_service::keyword_id.eq(&keyword.keyword_id)),
        )
        .limit(1)
        .execute(conn);
    debug!("Check: {:?}", check);
    matches!(check, Ok(x) if x == 0)
}

fn insert_rows_service_keywords(
    insert_data: &[InsertableServiceKeyword],
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    diesel::insert_into(keyword_to_service::keyword_to_service)
        .values(insert_data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Добавляет ключевые слова к компоненту по словам.
pub(crate) fn add_keywords_by_names(
    data: &IptServiceKeywordsNames,
    logged_user_uuid: &Uuid,
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

    add_service_keywords(
        &IptServiceKeywordsData {
            keyword_ids,
            service_uuid: data.service_uuid,
        },
        logged_user_uuid,
        conn,
    )
}

/// Clear duplicates keywords
fn clear_duplicates(keywords: &mut Vec<InsertableServiceKeyword>) {
    let mut already_seen = Vec::new();
    keywords.retain(|item| match already_seen.contains(&item.keyword_id) {
        true => false,
        _ => {
            already_seen.push(item.keyword_id);
            true
        }
    })
}
