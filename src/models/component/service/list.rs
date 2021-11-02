use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::model::{
    Component, ShowComponentShort, ComponentAndRelatedData
};
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
// use diesel::PgConnection;
use uuid::Uuid;

/// Gets components short data with filter by:
/// uuids, favorite list, user_uuid, company_uuid
pub(crate) fn find_components(
    logged_user_uuid: &Uuid,
    filter_components_uuids: &[Uuid],
    favorite: &bool,
    user_uuid: &Option<Uuid>,
    company_uuid: &Option<Uuid>,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowComponentShort>> {
    let need_access_level = 3;

    // if components owner self user - collect data without check access
    let mut flag_get_self_data = false;

    // limit filter uuids
    if filter_components_uuids.len() > 100 {
        return Err(ServiceError::BadRequest(
            "Maximum allowed filter less 100 components".to_string()
        ));
    }

    // select target components uuids
    let res_filter_uuids = match (
        filter_components_uuids, // &[Uuid]
        favorite, // &bool
        user_uuid, // &Option<Uuid>
        company_uuid,// &Option<Uuid>
    ) {
        // gets components of self favorite list
        // for authorized user with/without filter
        (fc_uuids, true, None, None) => {
            // change flag for get data without check
            flag_get_self_data = true;

            Component::get_fav_list_uuids_by_user(
                logged_user_uuid,
                fc_uuids,
                conn
            )?
        },
        (fc_uuids, false, Some(user_uuid), None) => {
            match logged_user_uuid == user_uuid {
                // gets components authorized user with/without filter
                true => {
                    // change flag for get data without check
                    flag_get_self_data = true;

                    Component::get_uuids_by_user(
                        logged_user_uuid,
                        fc_uuids,
                        conn
                    )?
                },
                // gets components user with/without filter
                false => {
                    if fc_uuids.is_empty() {
                        Component::get_uuids_by_user(
                            user_uuid,
                            fc_uuids,
                            conn
                        )?
                    } else {
                        fc_uuids.to_vec()
                    }
                },
            }
        },
        // gets components from user favorite list with/without filter
        (fc_uuids, true, Some(user_uuid), None) => {
            Component::get_fav_list_uuids_by_user(
                user_uuid,
                fc_uuids,
                conn
            )?
        },
        // gets components from company with/without filter
        (fc_uuids, false, None, Some(company_uuid)) => {
            Component::get_uuids_by_company(
                company_uuid,
                fc_uuids,
                conn
            )?
        },
        // gets components with/without filter
        (fc_uuids, _, _, _) => { // (fc_uuids, false, None, None)
            if fc_uuids.is_empty() {
                // if not set param and no filter
                return Err(ServiceError::BadRequest(
                    "Not correct parameters".to_string()
                ));
            }

            fc_uuids.to_vec()
        },
        // query with not correct parameters
        // _ => {
        //     return Err(ServiceError::BadRequest(
        //         "Not correct parameters".to_string()
        //     ));
        // },
    };

    if flag_get_self_data {
        Component::get_without_check_by_uuids(
            logged_user_uuid,
            &res_filter_uuids,
            set_lang_id,
            conn
        )
    } else {
        // remove components uuids without access
        let components_with_access = &Component::clear_uuids_without_access(
            logged_user_uuid,
            &res_filter_uuids,
            &need_access_level,
            conn
        )?;

        Component::get_without_check_by_uuids(
            logged_user_uuid,
            components_with_access,
            set_lang_id,
            conn
        )
    }
}

pub(crate) fn find_component_uuid(
    logged_user_uuid: &Uuid,
    target_component_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<ComponentAndRelatedData> {

    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        target_component_uuid,
        &need_access_level,
        conn
    )?;

    // collect data for component
    let result: ComponentAndRelatedData = ComponentAndRelatedData::collect_related_data(
        target_component_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading component and collect related data");

    debug!("Component data: {:#?}", result);

    Ok(result)
}
