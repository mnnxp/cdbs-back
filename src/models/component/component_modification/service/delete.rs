use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::service::update::change_updated_at;
use crate::models::component::{
    access::util::check_is_owner_with_err,
    component_modification::model::DelComponentModificationData,
    component_modification::relate::fileset_for_program::service::delete::delete_filesets_files_by_modifications,
};
use crate::models::relate_ref::file::service::delete::delete_file_by_uuids;
use crate::schema::component_modification_list::dsl as component_modification_list;
use crate::schema::file_to_modification::dsl as file_to_modification;
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет модификацию компонента.
pub(crate) fn del_component_modification(
    logged_user_uuid: &Uuid,
    data: &DelComponentModificationData,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    check_is_owner_with_err(logged_user_uuid, &data.component_uuid, conn)?;
    // update the updated_at component if modification has been removed
    change_updated_at(&data.component_uuid, None, conn)?;
    diesel::delete(
        component_modification_list::component_modification_list.filter(
            component_modification_list::component_uuid
                .eq(data.component_uuid)
                .and(component_modification_list::uuid.eq(data.modification_uuid)),
        ),
    )
    .returning(component_modification_list::uuid)
    .get_result::<Uuid>(conn)
    .map_err(|err| {
        debug!("Failed delete component modification: {:?}", err);
        ServiceError::InternalServerError
    })
}

/// Set the delete flags for all modifications and filesets files associated with the component
pub(crate) fn delete_modifications_files_by_component(
    component_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // get modifications related with component
    let modification_uuids = component_modification_list::component_modification_list
        .filter(component_modification_list::component_uuid.eq(component_uuid))
        .select(component_modification_list::uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets modifications for component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // set flags for modification fileset files
    delete_filesets_files_by_modifications(&modification_uuids, conn)?;

    let del_file_uuids = file_to_modification::file_to_modification
        .filter(file_to_modification::modification_uuid.eq_any(&modification_uuids))
        .select(file_to_modification::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of component: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
