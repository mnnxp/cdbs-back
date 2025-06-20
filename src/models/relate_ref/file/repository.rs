use super::commit::Commit;
use super::util::{find_id_ext, get_default_image};
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::file::ShowFileRelatedData;
use crate::models::relate_ref::file::model::{
    DownloadFile, ListObject, PreliminaryFileData, ShowFile, SlimFile,
};
use crate::models::relate_ref::program::model::Program;
use crate::models::search::order::{objects_order, Paginate, Sort, TableName};
use crate::models::user::model::ShowUserShort;
use crate::schema::file_ref::dsl as file_ref;
use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::{download_presigned_url, save_presign_url};
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает Uuid найденных по списку из file_uuids файлов, исключая удалённые и скрытые файлы
fn get_by_uuids(file_uuids: &[Uuid], conn: &mut PgConnection) -> ServiceResult<Vec<Uuid>> {
    file_ref::file_ref
        .select(file_ref::uuid)
        .filter(
            file_ref::uuid.eq_any(file_uuids).and(
                file_ref::is_hidden
                    .eq(false)
                    .and(file_ref::is_delete.eq(false)),
            ),
        )
        .limit(1000)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Возвращает Uuid найденных по списку из file_uuids файлов, исключаются только удалённые файлы
fn get_hide_by_uuids(file_uuids: &[Uuid], conn: &mut PgConnection) -> ServiceResult<Vec<Uuid>> {
    file_ref::file_ref
        .select(file_ref::uuid)
        .filter(
            file_ref::uuid
                .eq_any(file_uuids)
                // .and(file_ref::is_hidden.eq(true)
                .and(file_ref::is_delete.eq(false)),
        )
        .order(file_ref::revision.asc())
        .limit(1000)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files: {:?}", err);
            ServiceError::InternalServerError
        })
}

impl ShowFile {
    fn get_by_uuid(file_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<ShowFile> {
        file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::commit_uuid,
                file_ref::revision,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .filter(file_ref::uuid.eq(&file_uuid))
            .first::<ShowFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowFileRelatedData {
    pub(crate) fn get_file_by_uuids(
        target_file_uuids: &[Uuid],
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let file_uuids = get_by_uuids(target_file_uuids, conn)?;
        ShowFileRelatedData::data_enrichment(&file_uuids, sort, paginate, conn)
    }

    /// Adds information from other tables to the data (about the user, relevante program by extension)
    fn data_enrichment(
        file_uuids: &[Uuid],
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let mut res = Vec::new();
        for file_uuid in &objects_order(file_uuids, sort, paginate, conn)? {
            let sf = ShowFile::get_by_uuid(file_uuid, conn)?;
            res.push(ShowFileRelatedData {
                uuid: sf.uuid,
                filename: sf.filename.clone(),
                revision: sf.revision,
                commit_msg: Commit::get_message(&sf.commit_uuid, conn)?,
                parent_file_uuid: sf.parent_file_uuid,
                owner_user: ShowUserShort::get_without_check_by_uuid(&sf.user_uuid, conn)?,
                content_type: sf.content_type.clone(),
                filesize: sf.filesize,
                program: Program::get_program_for_ext(&sf.id_ext, conn)?,
                created_at: sf.created_at,
                updated_at: sf.updated_at,
            })
        }
        Ok(res)
    }

    pub(crate) fn get_revisions_by_uuid(
        file_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let mut revision_uuids = Vec::new();
        prev_revision_uuids_by_uuid(&mut revision_uuids, file_uuid, conn)?;
        next_revision_uuids_by_uuid(&mut revision_uuids, file_uuid, conn)?;

        let file_uuids = get_hide_by_uuids(&revision_uuids, conn)?;
        ShowFileRelatedData::data_enrichment(
            &file_uuids,
            &Sort::parsing(TableName::FileRef, "revision", false),
            paginate,
            conn,
        )
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
            .filter(
                file_ref::uuid.eq(target_file_uuid).and(
                    file_ref::is_hidden
                        .eq(false)
                        .and(file_ref::is_delete.eq(false)),
                ),
            )
            .first::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Collects SlimFiles data by target files uuids
    pub(crate) fn get_by_file_uuids(
        target_file_uuids: &[Uuid],
        paginate: &Paginate,
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
            .filter(
                file_ref::uuid.eq_any(target_file_uuids).and(
                    file_ref::is_hidden
                        .eq(false)
                        .and(file_ref::is_delete.eq(false)),
                ),
            )
            .order(file_ref::filename.asc())
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Collects array of File used to collect files with not upload confirmation
    /// Checking if a file is owned and not checked or deleted
    pub(crate) fn get_not_checked_by_uuids(
        target_file_uuids: &[Uuid],
        user_uuid: &Uuid,
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
            .filter(
                file_ref::uuid.eq_any(target_file_uuids).and(
                    file_ref::user_uuid.eq(user_uuid).and(
                        file_ref::is_checked.eq(false).and(
                            file_ref::is_hidden
                                .eq(true)
                                .and(file_ref::is_delete.eq(false)),
                        ),
                    ),
                ),
            )
            .load::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Returns a string in which each byte of data is encoded using two hexadecimal digits
    pub(crate) fn encode_hash(file_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<String> {
        let hash = file_ref::file_ref
            .select(file_ref::hash)
            .filter(
                file_ref::uuid
                    .eq(file_uuid)
                    .and(file_ref::is_delete.eq(false)),
            )
            .first::<Vec<u8>>(conn)
            .map_err(|err| {
                debug!("Failed get file hash: {:?}", err);
                ServiceError::InternalServerError
            })?;
        Ok(hex::encode(hash))
    }

    /// Returns a pre-signed link to a file in the repository (without check access)
    pub(crate) fn get_download_string(
        file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<String> {
        let slim_file = file_ref::file_ref
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .filter(
                file_ref::uuid
                    .eq(file_uuid)
                    .and(file_ref::is_delete.eq(false)),
            )
            .first::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get download string for file: {:?}", err);
                ServiceError::InternalServerError
            })?;
        let naive_local_now = chrono::Local::now().naive_local();
        let get_url_from_db = presigned_url_ref::presigned_url_ref
            .select(presigned_url_ref::presigned_url)
            .filter(
                presigned_url_ref::file_uuid
                    .eq(&slim_file.uuid)
                    .and(presigned_url_ref::expiration_at.gt(naive_local_now)),
            )
            .limit(1)
            .load::<String>(conn)
            .map_err(|err| {
                debug!("Failed get presigned_url: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if let Some(url) = get_url_from_db.into_iter().next() {
            return Ok(url);
        }
        debug!("Failed get presigned_url");
        // creates and saves (updates) download presigned url for a file in the database
        let presigned_url = download_presigned_url(&StorageAccess::from_env(), &slim_file)?;
        // save presigned url to database
        save_presign_url(&slim_file.uuid, &presigned_url, conn)?;
        Ok(presigned_url)
    }
}

impl PreliminaryFileData {
    /// Creating data for write information about the file before upload to storage
    pub(crate) fn from_ipt_file_data(
        user_uuid: Uuid,
        object: ListObject,
        filename: &str,
        commit_uuid: Uuid,
        conn: &mut PgConnection,
    ) -> PreliminaryFileData {
        // set default parent
        let parent_file_uuid = get_default_image();
        // getting rid of dangerous names
        let filename = sanitize_filename::sanitize(filename);
        // get id for extension
        let id_ext = find_id_ext(&filename, conn);

        Self {
            parent_file_uuid,
            revision: 1,
            object,
            user_uuid,
            filename,
            id_ext,
            commit_uuid,
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
        Ok(DownloadFile {
            uuid: slim_file.uuid,
            hash: hex::encode(&slim_file.hash),
            filename: slim_file.filename.clone(),
            filesize: slim_file.filesize,
            download_url: SlimFile::get_download_string(&slim_file.uuid, conn)?,
        })
    }

    /// Gets a DownloadFile by file UUID.
    /// If the file is not found, returns the DownloadFile for the default image.
    pub(crate) fn get_by_file_uuid(
        target_file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<DownloadFile> {
        let file = match SlimFile::get_file_by_uuid(target_file_uuid, conn) {
            Ok(slim_file) => slim_file,
            Err(_) => SlimFile::get_file_by_uuid(&get_default_image(), conn)?,
        };

        DownloadFile::get_by_slim_file(&file, conn)
    }

    /// Get structures of DownloadFile by files uuidsget_by_file_uuids
    pub(crate) fn get_by_file_uuids(
        target_file_uuids: &[Uuid],
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let mut collect_res: Vec<DownloadFile> = Vec::new();

        if target_file_uuids.is_empty() {
            return Ok(collect_res);
        }

        let slim_files = SlimFile::get_by_file_uuids(target_file_uuids, paginate, conn)?;

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
            .filter(
                file_ref::uuid
                    .ne(parent_uuid)
                    .and(file_ref::parent_file_uuid.eq(parent_uuid)),
            )
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
            .filter(
                file_ref::uuid
                    .eq(parent_uuid)
                    .and(file_ref::parent_file_uuid.ne(parent_uuid)),
            )
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
        .filter(
            file_ref::uuid.eq(target_file_uuid).and(
                file_ref::is_hidden
                    .eq(true)
                    .and(file_ref::is_delete.eq(false)),
            ),
        )
        // .order(file_ref::filename.asc())
        .first::<String>(conn)
        .map_err(|err| {
            debug!("Failed get file: {:?}", err);
            ServiceError::InternalServerError
        })
}
