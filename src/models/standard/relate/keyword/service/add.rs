use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::keyword::model::{
    StandardKeyword,
    IptStandardKeywordsData,
    InsertableStandardKeyword
};
use crate::models::standard::access::util::check_access_standard_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_standard_keywords(
    logged_user_uuid: &Uuid,
    data: &IptStandardKeywordsData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::keyword_to_standard::dsl::*;

    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &data.standard_uuid,
        &need_access_level,
        conn
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_kw_has: Vec<i32> = Vec::new(); // <-- for accumulated keyword duplicates

    // creating structures for inserting records into a table
    let new_standard_keywords: Vec<InsertableStandardKeyword> = data.into();

    if new_standard_keywords.is_empty() {
        // return error if not found correct keywords
        return Err(ServiceError::BadRequest("Not found keywords".to_string()))
    }

    let mut insert_data: Vec<InsertableStandardKeyword> = Vec::new();

    for standard_kw in new_standard_keywords {
        // check new row on non duplicate
        let flag_found_keyword = keyword_to_standard
            .filter(standard_uuid.eq(&standard_kw.standard_uuid)
            .and(keyword_id.eq(&standard_kw.keyword_id)))
            .execute(conn);

        match flag_found_keyword {
            Ok(x) if x == 0 => {
                debug!("Inserted standard keyword: {:?}", &standard_kw.keyword_id);

                insert_data.push(standard_kw);

                count_insert_rows += 1;
            },
            Ok(x) => {
                debug!("Found standard keyword in database: {:?}", x);

                error_kw_has.push(standard_kw.keyword_id);
            },
            Err(err) => {
                debug!("Failed check keyword for standard: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed check keyword for standard".to_string()
                ))
            },
        }
    }

    if insert_data.is_empty() {
        // return error if all keyword duplicate
        return Err(ServiceError::BadRequest(
            format!("This ids {:?} already has", error_kw_has)
        ))
    }

    match diesel::insert_into(keyword_to_standard)
        .values(&insert_data)
        .get_result::<StandardKeyword>(conn) {
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
