use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::ServiceResult;
use crate::graphql::file::ShowFileRelatedData;
use crate::models::component::component_modification::{
    file::repository::get_file_uuids_by_modification_uuid, model::ModificationFilesArg,
    util::get_component_by_modification,
};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::search::order::{Paginate, Sort};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает предварительно подписанные URL-адреса и другую информацию для загрузки файлов модификации компонента.
pub(crate) fn get_component_modification_files(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        AccessOperation::Read,
        conn,
    )?;

    let target_file_uuids =
        get_file_uuids_by_modification_uuid(&args.modification_uuid, &args.file_uuids, conn)?;
    DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, domain, conn)
}

/// Возвращает информацию о файлах модификации компонента.
pub(crate) fn get_component_modification_files_list(
    logged_user_uuid: &Uuid,
    args: &ModificationFilesArg,
    sort: &Sort,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    require_permission(
        logged_user_uuid,
        AccessEntity::Component,
        &get_component_by_modification(&args.modification_uuid, conn)?,
        AccessOperation::Read,
        conn,
    )?;

    ShowFileRelatedData::get_component_modification_files_offsec(
        &args.modification_uuid,
        &args.file_uuids,
        sort,
        paginate,
        domain,
        conn,
    )
}

impl ShowFileRelatedData {
    /// Returns file information from a component modification. Without access verification.
    pub(crate) fn get_component_modification_files_offsec(
        modification_uuid: &Uuid,
        file_uuids: &[Uuid],
        sort: &Sort,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let object_uuids =
            get_file_uuids_by_modification_uuid(modification_uuid, file_uuids, conn)?;
        ShowFileRelatedData::get_file_by_uuids(&object_uuids, sort, paginate, domain, conn)
    }
}
