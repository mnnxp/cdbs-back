use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::{
    ListObject,
    PreliminaryFileData,
    IptPreliminaryFileData,
    ShowFile,
    SlimFile
};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    pub fn get_file_by_uuid(
        target_uuid_file: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowFile> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq(target_uuid_file))
            .select((
                file_ref::uuid,
                file_ref::uuid_file_parent,
                file_ref::uuid_user,
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
        target_vec_uuid_file: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_vec_uuid_file))
            .select((
                file_ref::uuid,
                file_ref::uuid_file_parent,
                file_ref::uuid_user,
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
        target_uuid_file: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<SlimFile> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq(target_uuid_file))
            .select((
                file_ref::uuid,
                file_ref::filename,
                file_ref::filesize,
                file_ref::path_file,
            ))
            .first::<SlimFile>(conn)?)
    }

    pub fn get_file_by_vec_uuid(
        target_vec_uuid_file: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<SlimFile>> {
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_vec_uuid_file))
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
    pub fn from_ipt_preliminary_file_data(
        uuid_user: Uuid,
        uuid_file_parent: Uuid,
        object: ListObject,
        file_data: IptPreliminaryFileData,
        conn: &PgConnection,
    ) -> ServiceResult<PreliminaryFileData> {
        let IptPreliminaryFileData {
            filename,
            content_type,
            // filesize,
            ..
        } = file_data;

        // getting rid of dangerous names
        let filename = sanitize_filename::sanitize(&filename);

        // get id for extension
        let id_ext = super::util::find_id_ext(&filename, conn);

        Ok(Self {
            uuid_file_parent,
            object,
            uuid_user,
            filename,
            id_ext,
            content_type,
            // filesize,
        })
    }
}
