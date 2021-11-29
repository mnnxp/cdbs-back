use crate::errors::ServiceResult;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use crate::schema::file_to_standard::dsl as file_to_standard;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    /// Gets all files for standard by uuid
    pub(crate) fn for_standard_by_uuid(
        standard_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let target_files_uuids: Vec<Uuid> = file_to_standard::file_to_standard
            .filter(file_to_standard::standard_uuid.eq(standard_uuid))
            .select(file_to_standard::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFileRelatedData::get_file_by_uuids(
            &target_files_uuids,
            conn
        )
    }
}
