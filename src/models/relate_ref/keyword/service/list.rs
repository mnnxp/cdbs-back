use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::search::order::Paginate;
use crate::schema::keyword_ref::dsl::*;
use diesel::{PgConnection, prelude::*};

/// Returns keywords by IDs. If no keyword filter is specified, all existing ones are aggregated.
/// Keywords can be used for components and standards as well as companies.
pub(crate) fn get_keywords(
    keyword_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    match keyword_ids.is_empty() {
        true => find_all_keywords(paginate, conn),
        false => find_keyword_ids(keyword_ids, paginate, conn)
    }
}

fn find_all_keywords(
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    keyword_ref
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Keyword>(conn)
        .map_err(|err| {
            debug!("Failed get keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_keyword_ids(
    keyword_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Keyword>> {
    keyword_ref
        .filter(id.eq_any(keyword_ids))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Keyword>(conn)
        .map_err(|err| {
            debug!("Failed get keyword: {:?}", err);
            ServiceError::InternalServerError
        })
}
