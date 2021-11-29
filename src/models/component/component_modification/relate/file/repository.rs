use crate::errors::ServiceResult;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::file::model::FileModification;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use crate::schema::file_to_modification::dsl as file_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    pub(crate) fn for_component_modification(
        component_modification: &ComponentModification,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let target_files_uuids: Vec<Uuid> = FileModification::belonging_to(component_modification)
            .select(file_to_modification::file_uuid)
            .load::<Uuid>(conn)?;
        ShowFileRelatedData::get_file_by_uuids(
            &target_files_uuids,
            conn
        )
    }
}
