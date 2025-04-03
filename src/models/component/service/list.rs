use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::graphql::component_model::{ComponentAndRelatedData, ShowComponentShort};
use crate::models::search::order::{Paginate, Sort, objects_order};
use crate::models::search::model::{ExtraOptions, IptSearchArg};
use crate::models::component::{
    model::ComponentsArg,
    search::search_components,
    access::util::check_access_component_for_user,
};
use diesel::prelude::*;
// use diesel::PgConnection;
use uuid::Uuid;

/// Gets components short data by uuids
pub(crate) fn get_components_by_uuids(
    args: &IptSearchArg,
    options: &ExtraOptions,
    sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {
    let need_access_level = 3; // todo!(create enum for manage access level)
    let mut filter_uuids = Vec::new();
    // gets uuids for other search attributes
    if args.favorite {
        if let Some(ref ur_uuid) = args.user_uuid {
            filter_uuids.append(&mut get_components_followed_by_user(ur_uuid, conn)?);
        } else {
            filter_uuids.append(&mut get_components_followed_by_user(&options.logged_user_uuid, conn)?);
        }
    }
    // gets components by user
    if let Some(ref ur_uuid) = args.user_uuid {
        filter_uuids.append(&mut get_components_uuids_by_user(ur_uuid, conn)?);
    }
    // gets components relate with company
    if let Some(ref cy_uuid) = args.company_uuid {
        filter_uuids.append(&mut get_components_uuids_by_company(cy_uuid, conn)?);
    }
    // gets components relate with standard
    if let Some(ref sd_uuid) = args.standard_uuid {
        filter_uuids.append(&mut get_components_uuids_by_standard(sd_uuid, conn)?);
    }
    // gets components relate with service
    if let Some(ref se_uuid) = args.service_uuid {
        filter_uuids.append(&mut get_components_uuids_by_service(se_uuid, conn)?);
    }
    // search for all components matching the text query
    let mut found_component_uuids = search_components(args, filter_uuids.to_vec(), conn)?;
    // duplicate filtering
    found_component_uuids.sort_unstable();
    found_component_uuids.dedup();

    let mut ct_uuids_with_check = Vec::new();
    // selection of available components
    for ct_uuid in found_component_uuids {
        // check access user for select component
        match check_access_component_for_user(
            &options.logged_user_uuid,
            &ct_uuid,
            &need_access_level,
            conn
        ){
            Ok(true) => ct_uuids_with_check.push(ct_uuid),
            err => debug!("Bad access (get_list_by_uuids): {:?}", err),
        }
    }
    ct_uuids_with_check = objects_order(&ct_uuids_with_check, sort, paginate, conn)?;
    // for store the result
    let mut result: Vec<ShowComponentShort> = Vec::new();
    // collecting data for each component
    for ct_uuid in ct_uuids_with_check.iter() {
        result.push(
            ShowComponentShort::get_without_check_by_uuid(ct_uuid, options, conn)
                .map_err(|err| {
                    debug!("Failed get components: {:?}", err);
                    ServiceError::InternalServerError
                })?
        );
    }
    Ok(result)
}

/// Возвращает агрегированные данные о компонентах.
/// Получает краткие данные о компонентах с фильтром по: UUID, компании, стандарту, пользователю, избранному (для себя или другого пользователя).
pub(crate) fn get_components(
    arguments: &ComponentsArg,
    options: &ExtraOptions,
    sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {
    // structure for reduce the number of function arguments
    let ComponentsArg {
        filter_components_uuids,
        company_uuid,
        standard_uuid,
        service_uuid,
        user_uuid,
        favorite,
    } = arguments;
    // select target components uuids
    let target_components_uuids = match (favorite, user_uuid, standard_uuid, company_uuid, service_uuid) {
        // gets components of self favorite list for authorized user
        (true, None, None, None, None) => {
            get_components_followed_by_user(
                &options.logged_user_uuid,
                conn
            )?
        },
        // gets components by user
        (false, Some(ur_uuid), None, None, None) => {
            get_components_uuids_by_user(
                ur_uuid, // user_uuid
                conn
            )?
        },
        // gets objects to which the user is subscribed
        (true, Some(ur_uuid), None, None, None) => {
            get_components_followed_by_user(
                ur_uuid, // user_uuid
                conn
            )?
        },
        // gets components relate with company
        (false, None, None, Some(cy_uuid), None) => {
            get_components_uuids_by_company(
                cy_uuid, // company_uuid
                conn
            )?
        },
        // gets components relate with standard
        (false, None, Some(sd_uuid), None, None) => {
            get_components_uuids_by_standard(
                sd_uuid, // standard_uuid
                conn
            )?
        },
        // gets components relate with service
        (false, None, None, None, Some(se_uuid)) => {
            get_components_uuids_by_service(
                se_uuid, // service_uuid
                conn
            )?
        },
        // gets components with filter or all public
        (false, None, None, None, None) => {
            filter_components_uuids.to_vec()
        },
        _ => return Err(get_err_msg(ErrorMessage::FailedMatchArguments)),
    };

    // return not found if set filters and not select components
    if (*favorite || user_uuid.is_some() || company_uuid.is_some() || standard_uuid.is_some() ||
            service_uuid.is_some()) && target_components_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowComponentShort::get_components(
        &target_components_uuids,
        options,
        sort,
        paginate,
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
            debug!("Fail load uuid list target component: {:?}", err);
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
            debug!("Fail load uuid list target standard: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets all components uuids when related with service
pub(crate) fn get_components_uuids_by_service(
    target_service_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::component_to_service::dsl as component_to_service;

    component_to_service::component_to_service
        .filter(component_to_service::service_uuid.eq(target_service_uuid))
        .select(component_to_service::component_uuid)
        .load::<Uuid>(conn).map_err(|err| {
            debug!("Fail load uuid list target service: {:?}", err);
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
