use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::{
    keyword::model::{IptComponentKeywordsData, IptComponentKeywordsNames, InsertableComponentKeyword},
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::keyword::{
    model::{IptKeywordData, KeywordId},
    service::register::create_keyword,
};
use crate::schema::keyword_to_component::dsl as keyword_to_component;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет ключевые слова к компоненту по идентификаторам.
pub(crate) fn add_component_keywords(
    logged_user_uuid: &Uuid,
    data: &IptComponentKeywordsData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn
    )?;

    // creating structures for inserting records into a table
    let mut keywords: Vec<InsertableComponentKeyword> = data.into();

    clear_duplicates(&mut keywords);

    match keywords.is_empty() {
        true => Err(ServiceError::BadRequest("Not found keywords".to_string())),
        false => {
            keywords.retain(|k| check_keyword_for_component(k, conn));
            insert_rows_component_keywords(&keywords, conn)
        },
    }
}

/// Check already keyword for component (duplicate)
fn check_keyword_for_component(
    keyword: &InsertableComponentKeyword,
    conn: &mut PgConnection
) -> bool {
    let check = keyword_to_component::keyword_to_component
        .filter(keyword_to_component::component_uuid.eq(&keyword.component_uuid)
        .and(keyword_to_component::keyword_id.eq(&keyword.keyword_id)))
        .limit(1)
        .execute(conn);
    debug!("Check: {:?}", check);
    matches!(check, Ok(x) if x == 0)
}

fn insert_rows_component_keywords(
    insert_data: &[InsertableComponentKeyword],
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    diesel::insert_into(keyword_to_component::keyword_to_component)
        .values(insert_data)
        .execute(conn)
        .map_err(|err| {
            debug!("Fail inserted keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Добавляет ключевые слова к компоненту по словам.
pub(crate) fn add_keywords_by_names(
    logged_user_uuid: &Uuid,
    data: &IptComponentKeywordsNames,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let mut keyword_ids: Vec<i32> = Vec::new();

    for kw in &data.keywords {
        match KeywordId::get_by_name(kw, conn) {
            Ok(x) => keyword_ids.push(x),
            Err(err) => {
                debug!("Error ({:?}) for keyword: {:?}", err, kw);
                let keyword = create_keyword(&IptKeywordData{keyword: kw.clone()}, conn)?;
                keyword_ids.push(keyword.id);
            }
        }
    }

    add_component_keywords(
        logged_user_uuid,
        &IptComponentKeywordsData{
            keyword_ids,
            component_uuid: data.component_uuid,
        },
        conn
    )
}

/// Clear duplicates keywords
fn clear_duplicates(keywords: &mut Vec<InsertableComponentKeyword>)  {
    let mut already_seen = Vec::new();
    keywords.retain(|item| match already_seen.contains(&item.keyword_id) {
        true => false,
        _ => {
            already_seen.push(item.keyword_id);
            true
        }
    })
}
