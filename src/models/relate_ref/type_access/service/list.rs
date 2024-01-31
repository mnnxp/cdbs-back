use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::type_access::model::{
    TypeAccessTranslateList, TypeAccessArg
};
use crate::schema::type_access_translate_list::dsl::*;
use diesel::{PgConnection, prelude::*};

/// Возвращает типы доступа по идентификаторам.
/// Если фильтр на типы доступа не указан, то агрегируются все существующие.
pub(crate) fn get_type_access(
    args: &TypeAccessArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    match args.type_access_ids.is_empty() {
        true => find_all_type_access(&args.limit, &args.offset, set_lang_id, conn),
        false => find_type_access_ids(args, set_lang_id, conn),
    }
}

fn find_all_type_access(
    limit: &i32,
    offset: &i32,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    type_access_translate_list
        .filter(lang_id.eq(set_lang_id))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<TypeAccessTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get type access: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_type_access_ids(
    args: &TypeAccessArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<TypeAccessTranslateList>> {
    type_access_translate_list
        .filter(type_access_id.eq_any(&args.type_access_ids)
        .and(lang_id.eq(set_lang_id)))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<TypeAccessTranslateList>(conn)
        .map_err(|err| {
            debug!("Failed get type access: {:?}", err);
            ServiceError::InternalServerError
        })
}
