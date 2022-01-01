use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::{
    model::ModificationFilesArg,
    util::get_component_by_modification,
};
use crate::models::relate_ref::file::model::{DownloadFile, ShowFileRelatedData};
use crate::schema::file_to_modification::dsl as file_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile)
/// to get files associated with component_modifications
pub(crate) fn get_component_modification_files(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = file_to_modification::file_to_modification.into_boxed();

    query = match args.files_uuids.is_empty() {
        true => query.filter(file_to_modification::modification_uuid.eq(&args.modification_uuid)),
        false => query.filter(file_to_modification::modification_uuid.eq(&args.modification_uuid)
            .and(file_to_modification::file_uuid.eq_any(&args.files_uuids))),
    };

    let target_file_uuids: Vec<Uuid> = query
        .select(file_to_modification::file_uuid)
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get file: {:?}", err);
            ServiceError::InternalServerError
        })?;

    DownloadFile::get_by_files_uuids(&target_file_uuids, conn)
}

/// Get files list of component modification
pub(crate) fn get_component_modification_files_list(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = file_to_modification::file_to_modification.into_boxed();

    query = match args.files_uuids.is_empty() {
        true => query.filter(file_to_modification::modification_uuid.eq(&args.modification_uuid)),
        false => query.filter(file_to_modification::modification_uuid.eq(&args.modification_uuid)
            .and(file_to_modification::file_uuid.eq_any(&args.files_uuids))),
    };

    let target_file_uuids: Vec<Uuid> = query
        .select(file_to_modification::file_uuid)
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files: {:?}", err);
            ServiceError::InternalServerError
        })?;

    ShowFileRelatedData::get_file_by_uuids(&target_file_uuids, conn)
}
