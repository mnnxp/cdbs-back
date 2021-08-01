use crate::database::{get_conn, PooledConnection};
use crate::errors::{
    ServiceError,
    ServiceResult
};
// use crate::graphql::model::Context;
use async_graphql::Context;
use crate::models::file::model::ShowFile;
use diesel::prelude::*;

use uuid::Uuid;

pub(crate) fn get_files(
    context: &Context<'_>,
    uuid_user_search: Uuid,
    uuid_component_search: Uuid,
    uuid_component_modification_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowFile>> {
    let mut variant_selection: u8 = 0;
    if uuid_user_search > Uuid::nil() {
        variant_selection += 1;
    }
    if uuid_component_search > Uuid::nil() {
        variant_selection += 10;
    }
    if uuid_component_modification_search > Uuid::nil() {
        variant_selection += 100;
    }

    match variant_selection {
        0 => find_all_files(context, limit, offset),
        1 => find_uuid_user_file(context, uuid_user_search, limit, offset),
        10 => find_uuid_component_file(context, uuid_component_search, limit, offset),
        // 11
        100 => find_uuid_component_modification_file(
            context, uuid_component_modification_search, limit, offset
        ),
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string()))
    }
}

fn find_all_files(
    context: &Context<'_>,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowFile>> {
    use crate::schema::file_ref::dsl::*;
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(file_ref
        .inner_join(extension_ref)
        .select((
            uuid,
            uuid_file_parent,
            uuid_user,
            filename,
            content_type,
            id_ext,
            extension,
            filesize,
            path_file,
            created_at,
            updated_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowFile>(conn)?)
}

fn find_uuid_user_file(
    context: &Context<'_>,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowFile>> {
    use crate::schema::file_ref::dsl::*;
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    Ok(file_ref
        .inner_join(extension_ref)
        .select((
            uuid,
            uuid_file_parent,
            uuid_user,
            filename,
            content_type,
            id_ext,
            extension,
            filesize,
            path_file,
            created_at,
            updated_at
        ))
        .filter(uuid_user.eq(uuid_user_search))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowFile>(conn)?)
}

fn find_uuid_component_file(
    context: &Context<'_>,
    uuid_component_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowFile>> {
    use crate::schema::file_ref::dsl::*;
    use crate::schema::file_to_component::dsl::*;
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let uuid_for_select_file: Vec<Uuid> = file_to_component
        .filter(uuid_component.eq(uuid_component_search))
        .select(uuid_file)
        .limit(limit as i64)
        .offset(offset as i64)
        .load(conn)?;
    // debug!("fn find_uuid_component_file = {:?}", &uuid_for_select_file);

    match uuid_for_select_file {
        uuid_for_select_file if uuid_for_select_file.is_empty()
            => ServiceResult::Err(ServiceError::BadRequest("File not found.".to_string())),
        uuid_for_select_file => {
                // debug!("uuid_for_select_file = {:?}", &uuid_for_select_file);
                Ok(file_ref
                .inner_join(extension_ref)
                .select((
                    uuid,
                    uuid_file_parent,
                    uuid_user,
                    filename,
                    content_type,
                    id_ext,
                    extension,
                    filesize,
                    path_file,
                    created_at,
                    updated_at
                ))
                .filter(uuid.eq_any(uuid_for_select_file))
                .limit(limit as i64)
                .offset(offset as i64)
                .load::<ShowFile>(conn)?)
            }
    }
}

fn find_uuid_component_modification_file(
    context: &Context<'_>,
    uuid_component_modification_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowFile>> {
    use crate::schema::file_ref::dsl::*;
    use crate::schema::file_to_modification::dsl::*;
    use crate::schema::extension_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context)?;

    let uuid_for_select_file: Vec<Uuid> = file_to_modification
        .filter(uuid_modification.eq(uuid_component_modification_search))
        .select(uuid_file)
        .limit(limit as i64)
        .offset(offset as i64)
        .load(conn)?;

    match uuid_for_select_file {
        uuid_for_select_file if uuid_for_select_file.is_empty()
            => ServiceResult::Err(ServiceError::BadRequest("File not found.".to_string())),
        uuid_for_select_file
            => Ok(file_ref
                .inner_join(extension_ref)
                .select((
                    uuid,
                    uuid_file_parent,
                    uuid_user,
                    filename,
                    content_type,
                    id_ext,
                    extension,
                    filesize,
                    path_file,
                    created_at,
                    updated_at
                ))
                .filter(uuid.eq_any(uuid_for_select_file))
                .limit(limit as i64)
                .offset(offset as i64)
                .load::<ShowFile>(conn)?)
    }
}
