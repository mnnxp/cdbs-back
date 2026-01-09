use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::search::order::Paginate;
use crate::models::standard::{
    access::util::check_access_standard_for_user,
    file::repository::get_file_uuids_by_standard_uuid, model::StandardFilesArg,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает предварительно подписанные URL-адрес и другую информацию для загрузки файлов стандарта.
pub(crate) fn get_standard_files(
    logged_user_uuid: &Uuid,
    args: &StandardFilesArg,
    paginate: &Paginate,
    domain: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        &args.standard_uuid,
        need_access_level,
        conn,
    )?;

    let target_file_uuids =
        get_file_uuids_by_standard_uuid(&args.standard_uuid, &args.file_uuids, conn)?;

    DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, domain, conn)
}
