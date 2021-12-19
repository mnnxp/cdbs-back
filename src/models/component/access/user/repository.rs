use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::access::user::model::{
    UserAccessComponent, UserAccessComponentAndRelatedData
};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::user_access_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl UserAccessComponentAndRelatedData {
    /// Collect related data for users lits access component
    pub(crate) fn from_component_by_uuid(
        target_component_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
        let list_users_with_access = user_access_to_component
            .filter(component_uuid.eq(target_component_uuid))
            .load::<UserAccessComponent>(conn)
            .map_err(|err| {
                debug!("Failed get list accesses for component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut res: Vec<UserAccessComponentAndRelatedData> = Vec::new();
        for x in list_users_with_access {
            let type_access = TypeAccessTranslateList::get_type_access_by_id(
                &x.type_access_id,
                set_lang_id,
                conn
            )?;
            res.push(UserAccessComponentAndRelatedData{
                component_uuid: x.component_uuid,
                user_uuid: x.user_uuid,
                type_access: type_access.clone(),
                is_enabled: x.is_enabled,
                created_at: x.created_at,
                updated_at: x.updated_at,
            });
        }

        Ok(res)
    }
}
