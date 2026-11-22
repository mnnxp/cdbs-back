use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::component_model::{ComponentAndRelatedData, ShowComponentShort};
use crate::models::component::{
    access::repository::get_accessible_component_uuids,
    actual_status::model::ActualStatusTranslateList, component_fav::model::ComponentFav,
    component_fav::util::check_subscriber_by_uuid,
    component_type::model::ComponentTypeTranslateList, model::Component,
};
use crate::models::relate_ref::cache::{
    actual_status_by_id, component_type_by_id, type_access_by_id,
};
use crate::models::relate_ref::{
    file::model::DownloadFile, license::model::License, type_access::model::TypeAccessTranslateList,
};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::{objects_order, Paginate, Sort};
use crate::models::user::model::ShowUserShort;
use crate::schema::component_ref::dsl as component_ref;
use crate::schema::spec_ref::dsl as spec_ref;
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    /// Get component data from component_ref table by uuid
    pub(crate) fn get_component_by_uuid(
        target_component_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Component> {
        component_ref::component_ref
            .filter(
                component_ref::uuid
                    .eq(target_component_uuid)
                    .and(component_ref::is_delete.eq(false)),
            )
            .select((
                component_ref::uuid,
                component_ref::parent_component_uuid,
                component_ref::name,
                component_ref::description,
                component_ref::image_file_uuid,
                component_ref::user_uuid,
                component_ref::type_access_id,
                component_ref::component_type_id,
                component_ref::actual_status_id,
                component_ref::is_base,
                component_ref::created_at,
                component_ref::updated_at,
            ))
            .first::<Component>(conn)
            .map_err(|err| {
                debug!("Failed get component: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowComponentShort {
    /// Gets components by filter or all public
    pub(crate) fn get_components(
        filter_component_uuids: &[Uuid],
        options: &ExtraOptions,
        spec_id: Option<i32>,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        match filter_component_uuids.is_empty() {
            true => ShowComponentShort::get_all_public(options, spec_id, sort, paginate, conn),
            false => ShowComponentShort::get_list_by_uuids(
                filter_component_uuids,
                options,
                sort,
                paginate,
                conn,
            ),
        }
    }

    /// Gets component short data without checking access
    pub(crate) fn get_without_check_by_uuid(
        target_component_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowComponentShort> {
        // get target component
        let component = Component::get_component_by_uuid(target_component_uuid, conn)?;

        // get image file (favicon) for component
        let image_file =
            DownloadFile::get_by_file_uuid(&component.image_file_uuid, &options.domain, conn)?;

        // get component owner
        let owner_user =
            ShowUserShort::get_without_check_by_uuid(&component.user_uuid, &options.domain, conn)?;

        let (type_access, component_type, actual_status) =
            component_dictionaries(&component, options.set_lang_id)?;

        // check whether the object is being tracked auth user
        let is_followed =
            check_subscriber_by_uuid(target_component_uuid, &options.logged_user_uuid, conn)?;

        // get licenses for component
        let licenses = License::get_by_component_uuid(&component.uuid, conn)?;

        Ok(ShowComponentShort {
            uuid: component.uuid,
            name: component.name,
            description: component.description,
            image_file,
            owner_user,
            type_access,
            component_type,
            actual_status,
            is_base: component.is_base,
            is_followed,
            created_at: component.created_at,
            updated_at: component.updated_at,
            licenses,
        })
    }

    /// Gets components short data by uuids
    pub(crate) fn get_list_by_uuids(
        component_uuids: &[Uuid],
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // filter by access
        let accessible = get_accessible_component_uuids(
            &options.logged_user_uuid,
            AccessOperation::Read,
            component_uuids,
            conn,
        )?;
        if accessible.is_empty() {
            return Ok(Vec::new());
        }
        // sort and paginate the accessible set
        let ordered_uuids = objects_order(&accessible, sort, paginate, conn)?;
        // hydrate (no access check, already filtered)
        let mut result = Vec::with_capacity(ordered_uuids.len());
        for uuid in &ordered_uuids {
            result.push(ShowComponentShort::get_without_check_by_uuid(
                uuid, options, conn,
            )?);
        }
        Ok(result)
    }

    /// Gets all public components short data
    pub(crate) fn get_all_public(
        options: &ExtraOptions,
        spec_id: Option<i32>,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        let mut component_uuids = component_ref::component_ref
            .filter(
                component_ref::type_access_id
                    .eq(3)
                    .and(component_ref::is_delete.eq(false)),
            )
            .select(component_ref::uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get public components: {:?}", err);
                ServiceError::InternalServerError
            })?;
        if let Some(sc_id) = spec_id {
            component_uuids = filter_components_uuids_by_spec(&component_uuids, sc_id, conn)?;
        }
        // the result for store the result :)
        let mut result: Vec<ShowComponentShort> = Vec::new();
        // collecting data for each component without check
        for ct_uuid in objects_order(&component_uuids, sort, paginate, conn)? {
            result.push(ShowComponentShort::get_without_check_by_uuid(
                &ct_uuid, options, conn,
            )?);
        }
        Ok(result)
    }
}

impl ComponentAndRelatedData {
    /// Collecting component data and related data using uuid
    pub(crate) fn get_component(
        target_component_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ComponentAndRelatedData> {
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::Component,
            target_component_uuid,
            AccessOperation::Read,
            conn,
        )?;

        // collect data for component
        let component = Component::get_component_by_uuid(target_component_uuid, conn)?;

        // get image file (favicon) for component
        let image_file =
            DownloadFile::get_by_file_uuid(&component.image_file_uuid, &options.domain, conn)?;

        // get component owner
        let owner_user =
            ShowUserShort::get_without_check_by_uuid(&component.user_uuid, &options.domain, conn)?;

        let (type_access, component_type, actual_status) =
            component_dictionaries(&component, options.set_lang_id)?;

        // count subscribers component
        let subscribers: i32 = ComponentFav::get_count_followers_by_uuid(&component.uuid, conn)?;

        // check whether the object is being tracked auth user
        let is_followed =
            check_subscriber_by_uuid(target_component_uuid, &options.logged_user_uuid, conn)?;

        // get licenses for component
        let licenses = License::get_by_component_uuid(&component.uuid, conn)?;

        Ok(ComponentAndRelatedData {
            uuid: component.uuid,
            parent_component_uuid: component.parent_component_uuid,
            name: component.name,
            description: component.description,
            image_file,
            owner_user,
            type_access,
            component_type,
            actual_status,
            is_base: component.is_base,
            subscribers,
            is_followed,
            created_at: component.created_at,
            updated_at: component.updated_at,
            licenses,
        })
    }
}

/// Retrieves a list of component UUIDs associated with the specified catalog and its descendants.
/// If `filter_component_uuids` is empty, no filtering by component UUIDs is applied.
pub(crate) fn filter_components_uuids_by_spec(
    filter_component_uuids: &[Uuid],
    spec_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    // Load IDs of all descendant specs
    let mut descendant_ids = spec_ref::spec_ref
        .filter(spec_ref::path.like(format!("%.{}.%", spec_id)))
        .select(spec_ref::id)
        .load::<i32>(conn)
        .map_err(|err| {
            debug!(
                "Failed to load descendant ids for spec {}: {:?}",
                spec_id, err
            );
            ServiceError::InternalServerError
        })?;

    // Include the original spec_id
    descendant_ids.push(spec_id);

    let mut query = spec_to_component::spec_to_component.into_boxed();

    if filter_component_uuids.is_empty() {
        query = query.filter(spec_to_component::spec_id.eq_any(&descendant_ids))
    } else {
        query = query.filter(
            spec_to_component::spec_id
                .eq_any(&descendant_ids)
                .and(spec_to_component::component_uuid.eq_any(filter_component_uuids)),
        );
    }

    query
        .select(spec_to_component::component_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!(
                "Failed to load components for spec hierarchy {}: {:?}",
                spec_id, err
            );
            ServiceError::InternalServerError
        })
}

/// Resolves a component's dictionaries (access type, component type, actual status).
/// Cached, so no SQL after warm-up.
pub(crate) fn component_dictionaries(
    component: &Component,
    set_lang_id: i32,
) -> ServiceResult<(
    TypeAccessTranslateList,
    ComponentTypeTranslateList,
    ActualStatusTranslateList,
)> {
    Ok((
        type_access_by_id(component.type_access_id, set_lang_id)?,
        component_type_by_id(component.component_type_id, set_lang_id)?,
        actual_status_by_id(component.actual_status_id, set_lang_id)?,
    ))
}
