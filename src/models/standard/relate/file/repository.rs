use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    /// Gets all files for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_vec_file_uuid: Vec<Uuid> = file_to_standard::file_to_standard
            .filter(file_to_standard::standard_uuid.eq(standard_uuid))
            .select(file_to_standard::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFile::get_file_by_uuids(&target_vec_file_uuid, conn)
    }
}
