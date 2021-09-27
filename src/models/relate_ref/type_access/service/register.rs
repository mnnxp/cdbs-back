use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::relate_ref::type_access::model::{
    InsertableTypeAccessTranslateList,
    IptTypeAccessTranslateListData,
    TypeAccessTranslateList,
    TypeAccess
};
use diesel::prelude::*;

pub(crate) fn create_type_access(
    data: IptTypeAccessTranslateListData,
    conn: &PgConnection
) -> ServiceResult<TypeAccessTranslateList> {
    use crate::schema::type_access_translate_list::dsl::*;

    let flag_found_type_access = type_access_translate_list
        .filter(lang_id.eq(&data.lang_id))
        .filter(name.eq(&data.name))
        .select(type_access_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_type_access START SEARCH ={:?}", flag_found_type_access);

    match flag_found_type_access {
        0 => {
            let new_type_access_id = {
                use crate::schema::type_access_ref::dsl::*;

                let new_type_access: TypeAccess = diesel::insert_into(type_access_ref)
                    .default_values()
                    .get_result(conn)?;

                new_type_access.id
            };

            let data = InsertableTypeAccessTranslateList {
                type_access_id: new_type_access_id,
                lang_id: data.lang_id,
                name: data.name,
            };
            let inserted_type_access_data: TypeAccessTranslateList = diesel::insert_into(type_access_translate_list)
                .values(&data)
                .get_result(conn)?;
            Ok(inserted_type_access_data)
        },
        1.. => Err(ServiceError::BadRequest(
            format!("This type_access name is already there. Id: {}", flag_found_type_access))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
