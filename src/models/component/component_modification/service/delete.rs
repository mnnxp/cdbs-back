use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    component_modification::model::DelComponentModificationData,
    access::util::check_is_owner_with_err,
};
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_modification(
    logged_user_uuid: &Uuid,
    data: &DelComponentModificationData,
    conn: &PgConnection
) -> ServiceResult<Uuid> {
    check_is_owner_with_err(
        logged_user_uuid,
        &data.component_uuid,
        conn
    )?;

    diesel::delete(component_modification_list::component_modification_list
        .filter(component_modification_list::component_uuid.eq(data.component_uuid)
        .and(component_modification_list::uuid.eq(data.modification_uuid))))
        .returning(component_modification_list::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete component modification: {:?}", err);
            ServiceError::InternalServerError
        })
}
