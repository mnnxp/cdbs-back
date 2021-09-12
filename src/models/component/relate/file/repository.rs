use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::file::model::FileComponent;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFile {
    pub fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_vec_file_uuid: Vec<Uuid> = FileComponent::belonging_to(component)
            .select(file_to_component::file_uuid)
            .load::<Uuid>(conn)?;

        ShowFile::get_file_by_vec_uuid(&target_vec_file_uuid, conn)
    }
}
