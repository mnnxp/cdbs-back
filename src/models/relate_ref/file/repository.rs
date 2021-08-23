use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::file::model::FileComponent;
use crate::models::component::component_modification::file::model::FileModification;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::file_ref::dsl as file_ref;
use crate::schema::file_to_component::dsl as file_to_component;
use crate::schema::file_to_modification::dsl as file_to_modification;
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

    pub fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_uuid_file: Vec<Uuid> = FileComponent::belonging_to(component)
            .select(file_to_component::uuid_file)
            .load::<Uuid>(conn)?;
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_uuid_file))
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

    pub fn for_component_modification(
        component_modification: &ComponentModification,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowFile>> {
        let target_uuid_file: Vec<Uuid> = FileModification::belonging_to(component_modification)
            .select(file_to_modification::uuid_file)
            .load::<Uuid>(conn)?;
        Ok(file_ref::file_ref
            .filter(file_ref::uuid.eq_any(target_uuid_file))
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
