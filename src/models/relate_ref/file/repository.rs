use super::util::get_default_image;
use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, ShowFile,
    ShowFileRelatedData, DownloadFile, SlimFile,
};
use crate::models::relate_ref::program::model::Program;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::{download_presigned_url, save_presign_url};
use crate::schema::file_ref::dsl as file_ref;
use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    fn get_by_uuids(
        target_file_uuids: &[Uuid],
        limit: i32,
        offset: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        file_ref::file_ref.select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::revision,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                // file_ref::path_file,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .filter(file_ref::uuid.eq_any(target_file_uuids)
                .and(file_ref::is_hidden.eq(false)
                .and(file_ref::is_delete.eq(false))))
            .order(file_ref::filename.asc())
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<ShowFile>(conn)
            .map_err(|err| {
                debug!("Failed get files: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    fn get_hide_by_uuids(
        target_file_uuids: &[Uuid],
        limit: i32,
        offset: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        file_ref::file_ref.select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::revision,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                // file_ref::path_file,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .filter(file_ref::uuid.eq_any(target_file_uuids)
                // .and(file_ref::is_hidden.eq(true)
                .and(file_ref::is_delete.eq(false)))
            .order(file_ref::revision.asc())
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<ShowFile>(conn)
            .map_err(|err| {
                debug!("Failed get files: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowFileRelatedData {
    pub(crate) fn get_file_by_uuids(
        target_file_uuids: &[Uuid],
        limit: i32,
        offset: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let files_data = ShowFile::get_by_uuids(
            target_file_uuids,
            limit,
            offset,
            conn
        )?;
        let mut result: Vec<ShowFileRelatedData> = Vec::new();
        for fd in files_data {
            result.push(ShowFileRelatedData::data_enrichment(fd, conn)?)
        }
        Ok(result)
    }

    /// Adds information from other tables to the data (about the user, relevante program by extension)
    fn data_enrichment(
        file_data: ShowFile,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowFileRelatedData> {
        // collect data for user
        let owner_user: ShowUserShort = ShowUserShort::get_without_check_by_uuid(
            &file_data.user_uuid,
            conn
        ).expect("Error loading user");

        // get program by ext for file
        let program: Program = Program::get_program_for_ext(
            &file_data.id_ext,
            conn
        ).expect("Error loading user");

        Ok(ShowFileRelatedData {
            uuid: file_data.uuid,
            filename: file_data.filename,
            revision: file_data.revision,
            parent_file_uuid: file_data.parent_file_uuid,
            owner_user,
            content_type: file_data.content_type,
            filesize: file_data.filesize,
            program,
            created_at: file_data.created_at,
            updated_at: file_data.updated_at,
        })
    }

    pub(crate) fn get_revisions_by_uuid(
        file_uuid: &Uuid,
        limit: &i32,
        offset: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let mut revision_uuids = Vec::new();
        prev_revision_uuids_by_uuid(&mut revision_uuids, file_uuid, conn)?;
        next_revision_uuids_by_uuid(&mut revision_uuids, file_uuid, conn)?;

        let files_data = ShowFile::get_hide_by_uuids(
            &revision_uuids,
            *limit,
            *offset,
            conn
        )?;

        let mut result: Vec<ShowFileRelatedData> = Vec::new();
        for fd in files_data {
            result.push(ShowFileRelatedData::data_enrichment(fd, conn)?)
        }
        Ok(result)
    }
}

impl SlimFile {
    /// Get SlimFile data by target file uuid
    pub(crate) fn get_file_by_uuid(
        target_file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<SlimFile> {
        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(file_ref::uuid.eq(target_file_uuid)
                .and(file_ref::is_hidden.eq(false)
                .and(file_ref::is_delete.eq(false))))
            .first::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Collects SlimFiles data by target files uuids
    pub(crate) fn get_by_file_uuids(
        target_file_uuids: &[Uuid],
        limit: i32,
        offset: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(file_ref::uuid.eq_any(target_file_uuids)
                .and(file_ref::is_hidden.eq(false)
                .and(file_ref::is_delete.eq(false))))
            .order(file_ref::filename.asc())
            .limit(limit as i64)
            .offset(offset as i64)
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Collects array of SlimFile used to collect files with not upload confirmation
    pub(crate) fn get_not_checked_by_uuids(
        target_file_uuids: &[Uuid],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(file_ref::uuid.eq_any(target_file_uuids)
                .and(file_ref::is_checked.eq(false)
                .and(file_ref::is_hidden.eq(true)
                .and(file_ref::is_delete.eq(false)))))
            // .order(file_ref::filename.asc())
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl PreliminaryFileData {
    /// Creating data for write information about the file before upload to storage
    pub(crate) fn from_ipt_file_data(
        user_uuid: Uuid,
        object: ListObject,
        filename: &str,
        conn: &mut PgConnection,
    ) -> PreliminaryFileData {
        // set default parent
        let parent_file_uuid = get_default_image();
        // getting rid of dangerous names
        let filename = sanitize_filename::sanitize(filename);
        // get id for extension
        let id_ext = super::util::find_id_ext(&filename, conn);

        Self {
            parent_file_uuid,
            revision: 1,
            object,
            user_uuid,
            filename,
            id_ext,
            content_type: "application/text".to_string(), // <-- todo!(add parse of filename)
        }
    }
}

impl DownloadFile {
    /// Get DownloadFile with generated presigned_url from SlimFile data
    pub(crate) fn get_by_slim_file(
        slim_file: &SlimFile,
        conn: &mut PgConnection,
    ) -> ServiceResult<DownloadFile> {
        let naive_local_now = chrono::Local::now().naive_local();

        let get_url_from_db = presigned_url_ref::presigned_url_ref
            .select(presigned_url_ref::presigned_url)
            .filter(presigned_url_ref::file_uuid.eq(&slim_file.uuid)
                .and(presigned_url_ref::expiration_at.gt(naive_local_now)))
            .limit(1)
            .load::<String>(conn)
            .map_err(|err| {
                debug!("Failed get presigned_url: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let download_url = match get_url_from_db.first() {
            Some(url) => url.clone(),
            None => {
                debug!("Failed get presigned_url");
                // creates and saves (updates) download presigned url for a file in the database
                let presigned_url = download_presigned_url(
                    &StorageAccess::from_env(),
                    slim_file,
                )?;
                // save presigned url to database
                save_presign_url(&slim_file.uuid, &presigned_url, conn)?;
                presigned_url
            },
        };

        Ok(DownloadFile{
            uuid: slim_file.uuid,
            hash: hex::encode(&slim_file.hash),
            filename: slim_file.filename.clone(),
            filesize: slim_file.filesize,
            download_url,
        })
    }

    /// Get DownloadFile by file UUID, data will be received for SlimFile
    /// and then build DownloadFile with generated presigned_url
    pub(crate) fn get_by_file_uuid(
        target_file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<DownloadFile> {
        let file = SlimFile::get_file_by_uuid(target_file_uuid, conn)?;

        DownloadFile::get_by_slim_file(&file, conn)
    }

    /// Get structures of DownloadFile by files uuidsget_by_file_uuids
    pub(crate) fn get_by_file_uuids (
        target_file_uuids: &[Uuid],
        limit: i32,
        offset: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let mut collect_res: Vec<DownloadFile> = Vec::new();

        if target_file_uuids.is_empty() {
            return Ok(collect_res);
        }

        let slim_files = SlimFile::get_by_file_uuids(
            target_file_uuids,
            limit,
            offset,
            conn
        )?;

        for sf in &slim_files {
            collect_res.push(DownloadFile::get_by_slim_file(sf, conn)?);
        }

        Ok(collect_res)
    }
}

/// Collects UUIDs all next revision of a file
fn next_revision_uuids_by_uuid(
    revision_uuids: &mut Vec<Uuid>,
    file_uuid: &Uuid,
    // filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<()> {
    let mut get_parent = Some(*file_uuid);
    while let Some(parent_uuid) = get_parent {
        // revision.push(parent_uuid);
        revision_uuids.push(parent_uuid);
        get_parent = file_ref::file_ref
            .select(file_ref::uuid)
            .filter(file_ref::uuid.ne(parent_uuid)
                .and(file_ref::parent_file_uuid.eq(parent_uuid)))
            .order(file_ref::created_at.desc())
            .first::<Uuid>(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })?;
        debug!("Result found a parent: {:?}", parent_uuid);
    }
    debug!("Result found all revisions: {:?}", revision_uuids);
    Ok(())
}

/// Collects UUIDs all prev revision of a file
fn prev_revision_uuids_by_uuid(
    revision_uuids: &mut Vec<Uuid>,
    file_uuid: &Uuid,
    // filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<()> {
    let mut get_parent = Some(*file_uuid);
    while let Some(parent_uuid) = get_parent {
        // revision.push(parent_uuid);
        revision_uuids.push(parent_uuid);
        get_parent = file_ref::file_ref
            .select(file_ref::parent_file_uuid)
            .filter(file_ref::uuid.eq(parent_uuid)
                .and(file_ref::parent_file_uuid.ne(parent_uuid)))
            .order(file_ref::created_at.desc())
            .first::<Uuid>(conn)
            .optional()
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })?;
        debug!("Result found a parent: {:?}", parent_uuid);
    }
    debug!("Result found all revisions: {:?}", revision_uuids);
    Ok(())
}

/// Gets a filename for hidden file by UUID
pub(crate) fn get_filename_hidden_rev_by_uuid(
    target_file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<String> {
    file_ref::file_ref
        .select(file_ref::filename)
        .filter(file_ref::uuid.eq(target_file_uuid)
            .and(file_ref::is_hidden.eq(true)
            .and(file_ref::is_delete.eq(false))))
        // .order(file_ref::filename.asc())
        .first::<String>(conn)
        .map_err(|err| {
            debug!("Failed get file: {:?}", err);
            ServiceError::InternalServerError
        })
}