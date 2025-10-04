use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::{
    InsertableTypeAccessTranslateList, IptTypeAccessTranslateListData, TypeAccessTranslateList,
};
use crate::schema::type_access_ref::dsl as type_access_ref;
use crate::schema::type_access_translate_list::dsl as type_access_translate_list;
use diesel::prelude::*;

/// Добавляет тип доступа.
/// Возвращает ошибку с идентификатором типа доступа, если он уже существует.
pub(crate) fn create_type_access(
    data: &IptTypeAccessTranslateListData,
    conn: &mut PgConnection,
) -> ServiceResult<TypeAccessTranslateList> {
    let flag_found = type_access_translate_list::type_access_translate_list
        .filter(
            type_access_translate_list::lang_id
                .eq(&data.lang_id)
                .and(type_access_translate_list::name.eq(&data.name)),
        )
        .select(type_access_translate_list::type_access_id)
        .limit(1)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!("Not found data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // debug!("fn create_type_access START SEARCH ={:?}", flag_found);

    match flag_found.first() {
        Some(x) => Err(get_err_msg(ErrorMessage::NameAlreadyThereX(
            "type_access".to_string(),
            *x,
        ))),
        None => {
            let new_type_access_id = diesel::insert_into(type_access_ref::type_access_ref)
                .default_values()
                .returning(type_access_ref::id)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert type_access_ref data: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            let data = InsertableTypeAccessTranslateList {
                type_access_id: new_type_access_id,
                lang_id: data.lang_id,
                name: data.name.to_string(),
            };

            diesel::insert_into(type_access_translate_list::type_access_translate_list)
                .values(&data)
                .get_result::<TypeAccessTranslateList>(conn)
                .map_err(|err| {
                    debug!("Failed insert type_access_translate_list data: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }
}
