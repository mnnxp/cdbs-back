use crate::errors::ServiceResult;
use crate::models::relate_ref::file::{
    model::DownloadFile,
    service::list::get_urls_by_files_uuids,
};
use crate::models::standard::{
    model::StandardFilesArg,
    access::util::check_access_standard_for_user
};
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns pre-signed URLs (in wrapper DownloadFile) to get files associated with standards
pub(crate) fn get_standard_files(
    logged_user_uuid: &Uuid,
    arguments: &StandardFilesArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<DownloadFile>> {
    let StandardFilesArg {
        standard_uuid,
        files_uuids,
    } = arguments;

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        standard_uuid,
        &need_access_level,
        conn,
    )?;

    let target_file_uuids: Vec<Uuid> = match files_uuids.is_empty() {
        true => file_to_standard::file_to_standard
            .filter(file_to_standard::standard_uuid.eq(standard_uuid))
            .select(file_to_standard::file_uuid)
            .load::<Uuid>(conn)?,
        false => file_to_standard::file_to_standard
            .filter(file_to_standard::standard_uuid.eq(standard_uuid)
            .and(file_to_standard::file_uuid.eq_any(files_uuids)))
            .select(file_to_standard::file_uuid)
            .load::<Uuid>(conn)?,
    };

    get_urls_by_files_uuids(
        &target_file_uuids,
        conn
    )
}
