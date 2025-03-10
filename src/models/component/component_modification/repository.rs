use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::component_model::ComponentModificationAndRelatedData;
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_modification::model::{ComponentModification, ComponentModificationArg};
use crate::models::search::order::objects_order;
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentModification {
    /// Returns ComponentModification structures by modification uuids
    pub(crate) fn by_uuid(
        modification_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<ComponentModification> {
        // collect data for modifications the component
        component_modification_list::component_modification_list
            .filter(component_modification_list::uuid.eq(modification_uuid))
            .first::<ComponentModification>(conn)
            .map_err(|err| {
                debug!("Failed get component modification: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ComponentModificationAndRelatedData {
    pub(crate) fn by_args(
        args: &ComponentModificationArg,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
        let object_uuids = get_modification_uuids_by_component_uuid(&args.component_uuid, &args.filter, conn)?;
        // collect data for modifications the component
        let mut res = Vec::new();
        for modification_uuid in &objects_order(&object_uuids, &args.sort, &args.paginate, conn)? {
            let cm = ComponentModification::by_uuid(modification_uuid, conn)?;
            res.push(Self::for_modification(&cm, set_lang_id, conn)?)
        }
        Ok(res)
    }

    /// Get related data for component modification
    pub(crate) fn for_modification(
        cm: &ComponentModification,
        set_lang_id: &i32,
        conn: &mut PgConnection
    ) -> ServiceResult<ComponentModificationAndRelatedData> {
        let actual_status = ActualStatusTranslateList::get_by_id(
            &cm.actual_status_id,
            set_lang_id,
            conn
        )?;
        Ok(Self{
            uuid: cm.uuid,
            component_uuid: cm.component_uuid,
            parent_modification_uuid: cm.parent_modification_uuid,
            modification_name: cm.modification_name.clone(),
            description: cm.description.clone(),
            actual_status,
            created_at: cm.created_at,
            updated_at: cm.updated_at,
        })
    }
}

/// Returns an array of UUIDs of modification uuids relate with target component
pub(crate) fn get_modification_uuids_by_component_uuid(
    component_uuid: &Uuid,
    filter: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = component_modification_list::component_modification_list.into_boxed();
    query = match filter.is_empty() {
        true => query.filter(component_modification_list::component_uuid.eq(component_uuid)),
        false => query.filter(component_modification_list::component_uuid.eq(component_uuid)
            .and(component_modification_list::uuid.eq_any(filter))),
    };
    query
        .select(component_modification_list::uuid)
        .limit(1000)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get modification uuids for component: {:?}", err);
            ServiceError::InternalServerError
        })
}
