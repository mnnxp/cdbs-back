use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    file::model::{
        ListObject, PreliminaryFileData, ShowFile,
        ShowFileRelatedData, DownloadFile, SlimFile,
    },
    program::model::Program,
};
use crate::storage::model::StorageAccess;
use crate::storage::presigned_url::{download_presigned_url, save_presign_url};
use crate::schema::file_ref::dsl as file_ref;
use crate::schema::presigned_url_ref::dsl as presigned_url_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    pub(crate) fn get_file_by_uuid(
        target_file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowFileRelatedData> {
        let file_data: ShowFile = file_ref::file_ref
            .filter(file_ref::uuid.eq(target_file_uuid)
            .and(file_ref::is_delete.eq(false)))
            .select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                // file_ref::path_file,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .first::<ShowFile>(conn)
            .expect("Error get file data");

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

        let result = ShowFileRelatedData {
            uuid: file_data.uuid,
            filename: file_data.filename,
            parent_file_uuid: file_data.parent_file_uuid,
            owner_user,
            content_type: file_data.content_type,
            filesize: file_data.filesize,
            program,
            created_at: file_data.created_at,
            updated_at: file_data.updated_at,
        };

        Ok(result)
    }

    pub(crate) fn get_file_by_uuids(
        target_files_uuids: &[Uuid],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let mut result: Vec<ShowFileRelatedData> = Vec::new();

        for tfu in target_files_uuids {
            result.push(ShowFileRelatedData::get_file_by_uuid(
                tfu,
                conn
            )?)
        }
        // sorting the list of files by date updated
        result.sort_by(|a, b| a.updated_at.cmp(&b.updated_at));
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
            .filter(file_ref::uuid.eq(target_file_uuid)
            .and(file_ref::is_delete.eq(false)))
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .first::<SlimFile>(conn)
            .map_err(|err| {
                debug!("Failed get file: {:?}", err);
                ServiceError::InternalServerError
            })
    }

    /// Collect SlimFiles data by target files uuids
    pub(crate) fn get_by_files_uuids(
        target_files_uuids: &[Uuid],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_files_uuids)
            .and(file_ref::is_delete.eq(false)))
            .select((
                file_ref::uuid,
                file_ref::hash,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .order(file_ref::filename.asc())
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
        parent_file_uuid: Uuid,
        object: ListObject,
        filename: &str,
        conn: &mut PgConnection,
    ) -> PreliminaryFileData {
        // getting rid of dangerous names
        let filename = sanitize_filename::sanitize(filename);

        // get id for extension
        let id_ext = super::util::find_id_ext(&filename, conn);

        Self {
            parent_file_uuid,
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
            .filter(presigned_url_ref::file_uuid.eq(&slim_file.uuid)
            .and(presigned_url_ref::expiration_at.gt(naive_local_now)))
            .select(presigned_url_ref::presigned_url)
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
                // generate new url
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

    /// Get DownloadFile by file uuid
    /// data will be received for SlimFile
    /// and then build DownloadFile with generated presigned_url
    pub(crate) fn get_by_file_uuid(
        target_file_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<DownloadFile> {
        let file: SlimFile = SlimFile::get_file_by_uuid(
            target_file_uuid,
            conn
        )?;

        DownloadFile::get_by_slim_file(&file, conn)
    }

    /// Get structures of DownloadFile by files uuids
    pub(crate) fn get_by_files_uuids (
        target_files_uuids: &[Uuid],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let mut collect_res: Vec<DownloadFile> = Vec::new();

        if target_files_uuids.is_empty() {
            return Ok(collect_res);
        }

        for tfu in target_files_uuids {
            let res = DownloadFile::get_by_file_uuid(tfu, conn).map_err(|err| {
                debug!("Failed get data DownloadFile: {:?}", err);
                ServiceError::InternalServerError
            })?;

            collect_res.push(res);
        }

        Ok(collect_res)
    }
}
