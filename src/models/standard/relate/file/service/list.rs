use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::file as file;
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile) to get files associated with standards
pub(crate) fn get_standard_files(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        target_standard_uuid,
        &need_access_level,
        conn,
    )?;

    let target_file_uuids: Vec<Uuid> = file_to_standard::file_to_standard
        .filter(file_to_standard::standard_uuid.eq(target_standard_uuid))
        .select(file_to_standard::file_uuid)
        .load::<Uuid>(conn)?;

    file::service::list::get_urls_by_files_uuids(
        &target_file_uuids,
        conn
    )
}
