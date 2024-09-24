use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::fileset_for_program::file::repository::get_file_uuids_by_fileset_uuid;
use crate::models::component::{
    component_modification::fileset_for_program::file::model::FileOfFilesetArg,
    component_modification::fileset_for_program::util::get_component_by_fileset,
    access::util::check_access_component_for_user,
};
use crate::models::search::order::Paginate;
use crate::models::relate_ref::file::model::{ShowFileRelatedData, DownloadFile};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает информацию о файлах из набора файлов модификации компонента.
pub(crate) fn get_files_of_fileset(
    logged_user_uuid: &Uuid,
    args: &FileOfFilesetArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    // todo!(временное решение: убрать ограничение доступа файлам из набора модификации компонента)
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&args.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    // Gets uuids from target files for get files data or dowload urls
    let collect_file_uuids = get_file_uuids_by_fileset_uuid(
        &args.fileset_uuid,
        &args.file_uuids,
        conn
    )?;

    ShowFileRelatedData::get_file_by_uuids(
        &collect_file_uuids,
        &Paginate::parsing(args.limit, args.offset),
        conn
    ).map_err(|err| {
        debug!("Error get files of fileset: {:?}", err);
        ServiceError::InternalServerError
    })
}

/// Возвращает предварительно подписанные URL-адреса и другую информацию для загрузки файлов набора файлов модификации компонента.
pub(crate) fn get_fileset_files(
    logged_user_uuid: &Uuid,
    args: &FileOfFilesetArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    // todo!(временное решение: убрать ограничение доступа файлам из набора модификации компонента)
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(&args.fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let collect_file_uuids = get_file_uuids_by_fileset_uuid(
        &args.fileset_uuid,
        &args.file_uuids,
        conn
    )?;

    DownloadFile::get_by_file_uuids(
        &collect_file_uuids,
        &Paginate::parsing(args.limit, args.offset),
        conn
    ).map_err(|err| {
        debug!("Error get files of fileset: {:?}", err);
        ServiceError::InternalServerError
    })
}
