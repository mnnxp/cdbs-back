use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::graphql::file::ShowFileRelatedData;
use crate::models::component::file::repository::get_file_uuids_by_component_uuid;
use crate::models::component::model::ComponentFilesArg;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::search::order::{Paginate, Sort};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает предварительно подписанные URL-адрес и другую информацию для загрузки файлов компонента.
pub(crate) fn get_component_files(
    logged_user_uuid: &Uuid,
    args: &ComponentFilesArg,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &args.component_uuid,
        AccessOperation::Read,
        conn,
    )?;

    let target_file_uuids =
        get_file_uuids_by_component_uuid(&args.component_uuid, &args.file_uuids, conn)?;
    DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, domain, conn)
}

/// Возвращает информацию о файлах компонента.
pub(crate) fn get_component_files_list(
    logged_user_uuid: &Uuid,
    args: &ComponentFilesArg,
    sort: &Sort,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &args.component_uuid,
        AccessOperation::Read,
        conn,
    )?;

    let object_uuids =
        get_file_uuids_by_component_uuid(&args.component_uuid, &args.file_uuids, conn)?;
    ShowFileRelatedData::get_file_by_uuids(&object_uuids, sort, paginate, domain, conn)
}
