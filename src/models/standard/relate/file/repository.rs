use crate::errors::ServiceResult;
use crate::models::standard::model::Standard;
use crate::models::standard::file::model::FileStandard;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    pub fn for_standard(
        standard: &Standard,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_vec_file_uuid: Vec<Uuid> = FileStandard::belonging_to(standard)
            .select(file_to_standard::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFile::get_file_by_vec_uuid(&target_vec_file_uuid, conn)
    }
}
