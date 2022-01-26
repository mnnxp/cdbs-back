use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::model::{
    ShowComponentShort, ComponentAndRelatedData, ComponentsArg
};
use diesel::prelude::*;
// use diesel::PgConnection;
use uuid::Uuid;

/// Gets components short data with filter by:
/// uuids, favorite list, user_uuid, company_uuid
pub(crate) fn get_components(
    logged_user_uuid: &Uuid,
    arguments: &ComponentsArg,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {
    // structure for reduce the number of function arguments
    let ComponentsArg {
        filter_components_uuids,
        company_uuid,
        standard_uuid,
        user_uuid,
        favorite,
        limit,
        offset,
    } = arguments;

    // select target components uuids
    let target_components_uuids = match (favorite, user_uuid, standard_uuid, company_uuid) {
        // gets components of self favorite list for authorized user
        (true, None, None, None) => {
            get_components_followed_by_user(
                logged_user_uuid,
                conn
            )?
        },
        // gets components by user
        (false, Some(ur_uuid), None, None) => {
            get_components_uuids_by_user(
                ur_uuid, // user_uuid
                conn
            )?
        },
        // gets objects to which the user is subscribed
        (true, Some(ur_uuid), None, None) => {
            get_components_followed_by_user(
                ur_uuid, // user_uuid
                conn
            )?
        },
        // gets components relate with company
        (false, None, None, Some(cy_uuid)) => {
            get_components_uuids_by_company(
                cy_uuid, // company_uuid
                conn
            )?
        },
        // gets components relate with standard
        (false, None, Some(sd_uuid), None) => {
            get_components_uuids_by_standard(
                sd_uuid, // standard_uuid
                conn
            )?
        },
        // gets components with filter or all public
        (false, None, None, None) => {
            filter_components_uuids.to_vec()
        },
        _ => {
            return Err(ServiceError::BadRequest(
                "Failed match arguments".to_string()
            ))
        },
    };

    // return not found if set filters and not select components
    if (*favorite || user_uuid.is_some() || company_uuid.is_some() || standard_uuid.is_some()) &&
            target_components_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowComponentShort::get_components(
        logged_user_uuid,
        &target_components_uuids,
        limit,
        offset,
        set_lang_id,
        conn
    )
}

/// Gets all components uuids by target user (owner)
pub(crate) fn get_components_uuids_by_user(
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::component_ref::dsl as component_ref;

    component_ref::component_ref
        .filter(component_ref::user_uuid.eq(target_user_uuid)
        .and(component_ref::is_delete.eq(false)))
        .select(component_ref::uuid)
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets components uuids from favorite for user
pub(crate) fn get_components_followed_by_user(
    target_user_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::component_fav::dsl as component_fav;

    component_fav::component_fav
        .filter(component_fav::user_uuid.eq(target_user_uuid)
        .and(component_fav::is_enabled.eq(true)))
        .select(component_fav::component_uuid)
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets all components uuids when related with company
pub(crate) fn get_components_uuids_by_company(
    target_company_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::supplier_to_component::dsl as supplier_to_component;

    supplier_to_component::supplier_to_component
        .filter(supplier_to_component::company_uuid.eq(target_company_uuid))
        .select(supplier_to_component::component_uuid)
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets all components uuids when related with standard
pub(crate) fn get_components_uuids_by_standard(
    target_standard_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::standard_to_component::dsl as standard_to_component;

    standard_to_component::standard_to_component
        .filter(standard_to_component::standard_uuid.eq(target_standard_uuid))
        .select(standard_to_component::component_uuid)
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

pub(crate) fn get_component_by_uuid(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ComponentAndRelatedData> {
    // collect data for component
    ComponentAndRelatedData::get_component(
        target_component_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).map_err(|err| {
        debug!("Error loading component and collect related data: {:?}", err);
        ServiceError::BadRequest("Access denied".to_string())
    })
}
