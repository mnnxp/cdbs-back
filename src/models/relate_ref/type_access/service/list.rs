use crate::errors::ServiceResult;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_type_access(
    type_access_ids: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    match type_access_ids.is_empty() {
        true => find_all_type_access(
            limit,
            offset,
            set_lang_id,
            conn,
        ),
        false => find_type_access_ids(
            type_access_ids,
            limit,
            offset,
            set_lang_id,
            conn,
        ),
    }
}

fn find_all_type_access(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    use crate::schema::type_access_translate_list::dsl::*;

    Ok(type_access_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<TypeAccessTranslateList>(conn)?)
}

fn find_type_access_ids(
    type_access_ids: &[i32],
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    use crate::schema::type_access_translate_list::dsl::*;

    Ok(type_access_translate_list
        .filter(type_access_id.eq_any(type_access_ids)
        .and(lang_id.eq(set_lang_id)))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<TypeAccessTranslateList>(conn)?)
}
