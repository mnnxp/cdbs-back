use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::component::component_modification::set_of_files_program::model::SetOfFilesProgram;
use diesel::prelude::*;
use uuid::Uuid;


pub(crate) fn get_set_files_modification(
    context: &Context<'_>,
    target_uuid_modification: Uuid,
    target_id_program: i32,
    // target_id_set: i32,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SetOfFilesProgram>> {
    let mut variant_selection: u8 = 0;
    if target_id_program > 0 {
        variant_selection += 1;
    }
    if target_uuid_modification > Uuid::nil() {
        variant_selection += 10;
    }
    // if target_id_set > 0 {
    //     variant_selection += 100;
    // }

    match variant_selection {
        0 => ServiceResult::Err(ServiceError::BadRequest("Not correct query data.".to_string())),
        // 0 => find_all_set_files(context, limit, offset),
        // 1 => find_id_set_file(context, target_id_set, limit, offset),
        10 => find_set_for_uuid_modification(context, target_uuid_modification, limit, offset),
        // 11 => find_id_set_with_show_files(context, target_uuid_modification, target_id_set, limit, offset),
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_set_for_uuid_modification(
    context: &Context<'_>,
    target_uuid_modification: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<SetOfFilesProgram>> {
    use crate::schema::set_files_for_program::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(set_files_for_program
        .filter(uuid_modification.eq(target_uuid_modification))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<SetOfFilesProgram>(conn)?)
}

// fn find_id_set_with_show_files(
//     context: &Context<'_>,
//     target_uuid_modification: Uuid,
//     target_id_set: i32,
//     limit: i32,
//     offset: i32,
// ) -> ServiceResult<Vec<SetOfFilesProgram>> {
//     use crate::schema::set_files_for_program::dsl::*;
//     let conn: &PooledConnection = &get_conn(context)?;
//
//     Ok(set_files_for_program
//         .filter(uuid_modification.eq(target_uuid_modification))
//         .filter(id.eq(target_id_set))
//         .limit(limit as i64)
//         .offset(offset as i64)
//         .load::<SetOfFilesProgram>(conn)?)
// }
