use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    /// Get files by component_uuid
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let target_files_uuids: Vec<Uuid> = file_to_component::file_to_component
            .filter(file_to_component::component_uuid.eq(component_uuid))
            .select(file_to_component::file_uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get files for component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        ShowFileRelatedData::get_file_by_uuids(
            &target_files_uuids,
            conn
        )
    }
}
