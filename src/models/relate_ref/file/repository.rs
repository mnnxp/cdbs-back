use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::{
    ListObject,
    PreliminaryFileData,
    ShowFile,
    SlimFile
};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    pub fn get_file_by_uuid(
        target_file_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowFile> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq(target_file_uuid))
            .select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                file_ref::path_file,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .first::<ShowFile>(conn)?)
    }

    pub fn get_file_by_vec_uuid(
        target_vec_file_uuid: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_vec_file_uuid))
            .select((
                file_ref::uuid,
                file_ref::parent_file_uuid,
                file_ref::user_uuid,
                file_ref::filename,
                file_ref::content_type,
                file_ref::id_ext,
                file_ref::filesize,
                file_ref::path_file,
                file_ref::created_at,
                file_ref::updated_at,
            ))
            .load::<ShowFile>(conn)?)
    }
}


impl SlimFile {
    pub fn get_file_by_uuid(
        target_file_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<SlimFile> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq(target_file_uuid))
            .select((
                file_ref::uuid,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .first::<SlimFile>(conn)?)
    }

    pub fn get_file_by_vec_uuid(
        target_vec_file_uuid: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_vec_file_uuid))
            .select((
                file_ref::uuid,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .load::<SlimFile>(conn)?)
    }
}

impl PreliminaryFileData {
    /// Creating data for write information about the file before upload to storage
    pub fn from_ipt_file_data(
        user_uuid: Uuid,
        parent_file_uuid: Uuid,
        object: ListObject,
        filename: &str,
        conn: &PgConnection,
    ) -> ServiceResult<PreliminaryFileData> {
        // getting rid of dangerous names
        let filename = sanitize_filename::sanitize(&filename);

        // get id for extension
        let id_ext = super::util::find_id_ext(&filename, conn);

        Ok(Self {
            parent_file_uuid,
            object,
            user_uuid,
            filename,
            id_ext,
            content_type: "application/text".to_string(), // <-- todo!(add parse of filename)
        })
    }
}
