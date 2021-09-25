use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::model::{
    SlimComponentModification, DelComponentModificationData
};
use crate::models::component::util::check_is_owned;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn del_component_modification(
    logged_user_uuid: &Uuid,
    data: &DelComponentModificationData,
    conn: &PgConnection
) -> ServiceResult<SlimComponentModification> {
    use crate::schema::component_modification_list::dsl::*;

    check_is_owned(
        logged_user_uuid,
        &data.component_uuid,
        conn
    )?;

    let delete_component_modification = diesel::delete(component_modification_list
        .filter(component_uuid.eq(data.component_uuid)
        .and(uuid.eq(data.modification_uuid))))
        .returning((
            uuid,
            component_uuid,
            modification_name,
            description,
            updated_at,
        ))
        .get_result::<SlimComponentModification>(conn);

    match delete_component_modification {
        Ok(res) => Ok(res),
        Err(err) => {
            debug!("Failed delete component modification: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed delete component modification".to_string()
            ))
        }
    }
}
