use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::file::model::ComponentFile;
use crate::models::relate_ref::file::model::ShowFileRelatedData;
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    pub(crate) fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let target_files_uuids: Vec<Uuid> = ComponentFile::belonging_to(component)
            .select(file_to_component::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFileRelatedData::get_file_by_uuids(
            &target_files_uuids,
            conn
        )
    }
}
