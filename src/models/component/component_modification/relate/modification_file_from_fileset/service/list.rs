use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    component_modification::{
        modification_file_from_fileset::model::FileOfFilesetArg,
        fileset_for_program::util::get_component_by_fileset,
    },
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::model::{ShowFileRelatedData, DownloadFile};
use crate::schema::modification_file_from_fileset::dsl as modification_file_from_fileset;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_files_of_fileset(
    logged_user_uuid: &Uuid,
    arguments: &FileOfFilesetArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    let collect_file_uuids = get_file_uuids(logged_user_uuid, arguments, conn)?;

    ShowFileRelatedData::get_file_by_uuids(&collect_file_uuids, conn)
        .map_err(|err| {
            debug!("Error get files of fileset: {:?}", err);
            ServiceError::InternalServerError
        })
}

pub(crate) fn get_fileset_files(
    logged_user_uuid: &Uuid,
    arguments: &FileOfFilesetArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let collect_file_uuids = get_file_uuids(logged_user_uuid, arguments, conn)?;

    DownloadFile::get_by_files_uuids(&collect_file_uuids, conn)
        .map_err(|err| {
            debug!("Error get files of fileset: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets uuids from target files for get files data or dowload urls
fn get_file_uuids(
    logged_user_uuid: &Uuid,
    arguments: &FileOfFilesetArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let FileOfFilesetArg {
        fileset_uuid,
        file_uuids,
        limit,
        offset,
    } = arguments;

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = modification_file_from_fileset::modification_file_from_fileset.into_boxed();

    match file_uuids.is_empty() {
        // get all files for fileset
        true => query = query
            .filter(modification_file_from_fileset::fileset_uuid.eq(fileset_uuid)),
        // add filter for target files
        false => query = query
            .filter(modification_file_from_fileset::fileset_uuid.eq(fileset_uuid)
            .and(modification_file_from_fileset::file_uuid.eq_any(file_uuids))),
    }

    query.select(modification_file_from_fileset::file_uuid)
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Error get files of fileset: {:?}", err);
            ServiceError::InternalServerError
        })
}
