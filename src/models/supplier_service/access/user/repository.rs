use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::supplier_service::access::user::model::{
    UserAccessService, UserAccessServiceAndRelatedData,
};
use crate::schema::user_access_to_service::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl UserAccessServiceAndRelatedData {
    /// Collect related data for users lits access service
    pub(crate) fn from_service_by_uuid(
        target_service_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<UserAccessServiceAndRelatedData>> {
        let list_users_with_access = user_access_to_service
            .filter(service_uuid.eq(target_service_uuid))
            .load::<UserAccessService>(conn)
            .map_err(|err| {
                debug!("Failed get users list with access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut res: Vec<UserAccessServiceAndRelatedData> = Vec::new();
        for x in list_users_with_access {
            let type_access = TypeAccessTranslateList::get_type_access_by_id(
                &x.type_access_id,
                set_lang_id,
                conn,
            )?;

            res.push(UserAccessServiceAndRelatedData {
                service_uuid: x.service_uuid,
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
