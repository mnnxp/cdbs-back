use crate::errors::ServiceResult;
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::{
    model::ModificationFilesArg,
    file::repository::get_file_uuids_by_modification_uuid,
    util::get_component_by_modification,
};
use crate::models::search::order::Paginate;
use crate::models::relate_ref::file::model::{DownloadFile, ShowFileRelatedData};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает предварительно подписанные URL-адреса и другую информацию для загрузки файлов модификации компонента.
pub(crate) fn get_component_modification_files(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let target_file_uuids = get_file_uuids_by_modification_uuid(
        &args.modification_uuid,
        &args.file_uuids,
        conn
    )?;

    DownloadFile::get_by_file_uuids(
        &target_file_uuids,
        &Paginate::parsing(args.limit, args.offset),
        conn
    )
}

/// Возвращает информацию о файлах модификации компонента.
pub(crate) fn get_component_modification_files_list(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let target_file_uuids = get_file_uuids_by_modification_uuid(
        &args.modification_uuid,
        &args.file_uuids,
        conn
    )?;

    ShowFileRelatedData::get_file_by_uuids(
        &target_file_uuids,
        &Paginate::parsing(args.limit, args.offset),
        conn
    )
}
