use crate::errors::ServiceResult;
use crate::graphql::file::ShowFileRelatedData;
use crate::models::relate_ref::file::access::check_file_owner_err;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::search::order::Paginate;
use diesel::PgConnection;
use uuid::Uuid;

/// Возвращает предварительно подписанный URL-адрес для загрузки файла из хранилища.
/// Работает только в том случае, если файл поддерживает управление версиями, связан с:
/// компонентом, модификацией компонента, набором файлов или стандартом.
pub(crate) fn get_url_by_file_uuid(
    logged_user_uuid: &Uuid,
    target_file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<DownloadFile> {
    // check ownership file
    check_file_owner_err(logged_user_uuid, target_file_uuid, conn)?;

    DownloadFile::get_by_file_uuid(target_file_uuid, conn)
}

/// Возвращает информацию обо всех редакциях (версиях) файла.
pub(crate) fn get_revisions_by_file_uuid(
    file_uuid: &Uuid,
    logged_user_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowFileRelatedData>> {
    // check ownership file
    check_file_owner_err(logged_user_uuid, file_uuid, conn)?;

    ShowFileRelatedData::get_revisions_by_uuid(file_uuid, paginate, conn)
}
