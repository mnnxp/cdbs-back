use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::search::order::Paginate;
use crate::schema::type_access_translate_list::dsl::*;
use diesel::{prelude::*, PgConnection};

/// Returns access types by ID.
/// If you do not specify a filter for access types, all existing access types are aggregated.
pub(crate) fn get_type_access(
    type_access_ids: &[i32],
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    match type_access_ids.is_empty() {
        true => find_all_type_access(set_lang_id, paginate, conn),
        false => find_type_access_ids(type_access_ids, set_lang_id, paginate, conn),
    }
}

fn find_all_type_access(
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    type_access_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<TypeAccessTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get type access: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_type_access_ids(
    type_access_ids: &[i32],
    set_lang_id: &i32,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    type_access_translate_list
        .filter(
            type_access_id
                .eq_any(type_access_ids)
                .and(lang_id.eq(set_lang_id)),
        )
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<TypeAccessTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get type access: {:?}", err);
            ServiceError::InternalServerError
        })
}
