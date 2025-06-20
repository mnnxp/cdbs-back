use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::util::check_access_component_for_user;
use crate::models::component::component_modification::model::{
    InsertableComponentModification, IptComponentModificationData, IptMultipleModificationsData,
};
use crate::models::component::component_modification::param::service::change::put_new_modification_params;
use crate::models::component::service::update::change_updated_at;
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;
use uuid::Uuid;

/// Создаёт новую модификацию для компонента.
pub(crate) fn create_component_modification(
    logged_user_uuid: &Uuid,
    data: &IptComponentModificationData,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn,
    )?;
    let mut insert_data: InsertableComponentModification = data.into();
    // update updated_at date for component
    change_updated_at(&data.component_uuid, None, conn)?;
    match insert_data.parent_uuid_is_nil() {
        true => single_modification(&mut insert_data, conn),
        false => diesel::insert_into(component_modification_list::component_modification_list)
            .values(&insert_data)
            .returning(component_modification_list::uuid)
            .get_result::<Uuid>(conn)
            .map_err(|err| {
                debug!("Error create modification data: {:?}", err);
                ServiceError::InternalServerError
            }),
    }
}

/// Массовое создание модификаций для компонента
pub(crate) fn creation_multiple_modifications(
    logged_user_uuid: &Uuid,
    data: &IptMultipleModificationsData,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &data.component_uuid,
        &need_access_level,
        conn,
    )?;

    let mut res = Vec::new();
    // for insert_item in InsertableComponentModification::get_multiple_data(data).iter_mut() {
    for new_mn in &data.modifications_data {
        // check the name already in use
        match check_unique_name(&data.component_uuid, &new_mn.modification_name, conn) {
            Ok(mn_uuid) => {
                debug!(
                    "Modification {:?} already had the name {:?}",
                    mn_uuid, new_mn.modification_name
                );
                continue;
            }
            Err(err) => {
                debug!(
                    "No modification with the same name was found. Message: {:?}",
                    err
                );
                let mut insert_data =
                    InsertableComponentModification::get_multiple_data(data.component_uuid, new_mn);
                let new_uuid = single_modification(&mut insert_data, conn)?;
                let new_params = put_new_modification_params(&new_uuid, &new_mn.parameters, conn)?;
                debug!("{} parameters added to new modification", new_params);
                res.push(new_uuid);
            }
        }
    }
    // update the updated_at of component if new modification are added
    if !res.is_empty() {
        change_updated_at(&data.component_uuid, None, conn)?;
    }
    Ok(res)
}

/// Checks the name of the modification to be added for uniqueness for this component
fn check_unique_name(
    component_uuid: &Uuid,
    modification_name: &String,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    component_modification_list::component_modification_list
        .select(component_modification_list::uuid)
        .filter(
            component_modification_list::component_uuid
                .eq(component_uuid)
                .and(component_modification_list::modification_name.eq(modification_name)),
        )
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Check unique name uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}

pub(crate) fn single_modification(
    insert_data: &mut InsertableComponentModification,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    insert_data.parent_uuid_to_base();
    // debug!("insert_data after: {:#?}", insert_data);

    let new_uuid = diesel::insert_into(component_modification_list::component_modification_list)
        .values(&*insert_data)
        .returning(component_modification_list::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Error create modification data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    diesel::update(component_modification_list::component_modification_list)
        .filter(component_modification_list::uuid.eq(&new_uuid))
        .set(component_modification_list::parent_modification_uuid.eq(&new_uuid))
        .returning(component_modification_list::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Error change parent modification uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}
