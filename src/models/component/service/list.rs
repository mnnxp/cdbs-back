use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::ExtraOptions;
use crate::models::component::model::{
    ShowComponentShort, ComponentAndRelatedData, ComponentsArg
};
use diesel::prelude::*;
// use diesel::PgConnection;
use uuid::Uuid;

/// Возвращает агрегированные данные о компонентах.
/// Получает краткие данные о компонентах с фильтром по: UUID, компании, стандарту, пользователю, избранному (для себя или другого пользователя).
pub(crate) fn get_components(
    logged_user_uuid: &Uuid,
    arguments: &ComponentsArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
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
        _ => return Err(get_err_msg(ErrorMessage::FailedMatchArguments)),
    };

    // return not found if set filters and not select components
    if (*favorite || user_uuid.is_some() || company_uuid.is_some() || standard_uuid.is_some()) &&
            target_components_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowComponentShort::get_components(
        &target_components_uuids,
        &ExtraOptions {
            logged_user_uuid: *logged_user_uuid,
            set_lang_id: *set_lang_id,
            limit: *limit,
            offset: *offset,
        },
        conn
    )
}

/// Gets all components uuids by target user (owner)
pub(crate) fn get_components_uuids_by_user(
    target_user_uuid: &Uuid,
    conn: &mut PgConnection,
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
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::component_fav::dsl as component_fav;

    component_fav::component_fav
        .filter(component_fav::user_uuid.eq(target_user_uuid)
        .and(component_fav::is_enabled.eq(true)))
        .select(component_fav::component_uuid)
        .order(component_fav::created_at.desc())
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target user: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets all components uuids when related with company
pub(crate) fn get_components_uuids_by_company(
    target_company_uuid: &Uuid,
    conn: &mut PgConnection,
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
    conn: &mut PgConnection,
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

/// Возвращает полную информацию о компоненте по UUID.
pub(crate) fn get_component_by_uuid(
    target_component_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<ComponentAndRelatedData> {
    // collect data for component
    ComponentAndRelatedData::get_component(
        target_component_uuid,
        options,
        conn
    ).map_err(|err| {
        debug!("Error loading component and collect related data: {:?}", err);
        get_err_msg(ErrorMessage::AccessDenied)
    })
}
