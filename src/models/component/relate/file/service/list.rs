use crate::errors::ServiceResult;
use crate::models::component::file::repository::get_file_uuids_by_component_uuid;
use crate::models::component::{
    model::ComponentFilesArg,
    access::util::check_access_component_for_user,
};
use crate::models::relate_ref::file::model::{DownloadFile, ShowFileRelatedData};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает предварительно подписанные URL-адрес и другую информацию для загрузки файлов компонента.
pub(crate) fn get_component_files(
    logged_user_uuid: &Uuid,
    args: &ComponentFilesArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &args.component_uuid,
        &need_access_level,
        conn
    )?;

    let target_file_uuids = get_file_uuids_by_component_uuid(&args.component_uuid, &args.file_uuids, conn)?;

    DownloadFile::get_by_file_uuids(
        &target_file_uuids,
        args.limit,
        args.offset,
        conn
    )
}

/// Возвращает информацию о файлах компонента.
pub(crate) fn get_component_files_list(
    logged_user_uuid: &Uuid,
    args: &ComponentFilesArg,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &args.component_uuid,
        &need_access_level,
        conn
    )?;

    let target_file_uuids = get_file_uuids_by_component_uuid(&args.component_uuid, &args.file_uuids, conn)?;

    ShowFileRelatedData::get_file_by_uuids(
        &target_file_uuids,
        args.limit,
        args.offset,
        conn
    )
}
