// use crate::database::{db_connection, Pool};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::spec::model::{
    InsertableSpecTranslateList,
    IptSpecTranslateListData,
    SpecTranslateList,
    InsertableSpec,
    Spec
};
// use actix_web::web;
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_spec(
    new_spec_data: IptSpecTranslateListData,
    conn: &PgConnection
) -> ServiceResult<SpecTranslateList> {
    use crate::schema::spec_translate_list::dsl::*;

    let flag_found_spec = spec_translate_list
        .filter(lang_id.eq(&new_spec_data.lang_id)
        .and(spec.eq(&new_spec_data.spec)))
        .select(spec_id)
        .first::<i32>(conn).unwrap_or(0);

    // debug!("fn create_spec START SEARCH ={:?}", flag_found_spec);

    match flag_found_spec {
        0 => {
            let new_spec_id = {
                use crate::schema::spec_ref::dsl::*;

                let value_spec_data: InsertableSpec = InsertableSpec {
                    parent_spec_id: new_spec_data.parent_spec_id
                };
                let new_spec: Spec = diesel::insert_into(spec_ref)
                    .values(&value_spec_data)
                    .get_result(conn)?;

                new_spec.id
            };

            let new_spec_data = InsertableSpecTranslateList {
                spec_id: new_spec_id,
                lang_id: new_spec_data.lang_id,
                spec: new_spec_data.spec,
            };
            let inserted_spec_data: SpecTranslateList = diesel::insert_into(spec_translate_list)
                .values(&new_spec_data)
                .get_result(conn)?;
            Ok(inserted_spec_data)
        },
        1..=i32::MAX => Err(ServiceError::BadRequest(
            format!("This spec name is already there. Id: {}", flag_found_spec))
        ),
        _ => Err(ServiceError::BadRequest("What?".to_string())),
    }
}
