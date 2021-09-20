use crate::errors::{
    ServiceResult,
    ServiceError,
};
use crate::models::component::keyword::model::{
    ComponentKeyword,
    IptComponentKeywordData,
    InsertableComponentKeyword
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_component_keywords(
    data: IptComponentKeywordData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::keyword_to_component::dsl::*;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_kw_has: Vec<i32> = Vec::new(); // <-- for accumulated keyword duplicates

    // creating structures for inserting records into a table
    let new_component_keywords: Vec<InsertableComponentKeyword> = data.into();

    if new_component_keywords.is_empty() {
        // return error if not found correct keywords
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    let mut insert_data: Vec<InsertableComponentKeyword> = Vec::new();

    for component_kw in new_component_keywords {
        // check new row on non duplicate
        let flag_found_keyword = keyword_to_component
            .filter(component_uuid.eq(&component_kw.component_uuid)
            .and(keyword_id.eq(&component_kw.keyword_id)))
            .execute(conn).unwrap_or(0);

        if flag_found_keyword == 0 {
            debug!("Inserted component keyword: {:?}", &component_kw.keyword_id);

            insert_data.push(component_kw);

            count_insert_rows += 1;
        } else {
            error_kw_has.push(component_kw.keyword_id);
        }
    }

    if insert_data.is_empty() {
        // return error if all keyword duplicate
        return Err(ServiceError::BadRequest(
            format!("This ids {:?} already has", error_kw_has)
        ))
    }

    match diesel::insert_into(keyword_to_component)
        .values(&insert_data)
        .get_result::<ComponentKeyword>(conn) {
        Ok(_) => {
            debug!("Completed, add {:?} keywords", count_insert_rows);

            Ok(count_insert_rows)
        },
        Err(err) => {
            debug!("Fail inserted keyword: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
