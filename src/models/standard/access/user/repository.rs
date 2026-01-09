use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::standard::access::user::model::{
    UserAccessStandard, UserAccessStandardAndRelatedData,
};
use crate::schema::user_access_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl UserAccessStandardAndRelatedData {
    /// Collect related data for users lits access standard
    pub(crate) fn from_standard_by_uuid(
        target_standard_uuid: &Uuid,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<UserAccessStandardAndRelatedData>> {
        let list_users_with_access = user_access_to_standard
            .filter(standard_uuid.eq(target_standard_uuid))
            .load::<UserAccessStandard>(conn)
            .map_err(|err| {
                debug!("Failed get users list with access: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut res: Vec<UserAccessStandardAndRelatedData> = Vec::new();
        for x in list_users_with_access {
            let type_access = TypeAccessTranslateList::get_type_access_by_id(
                x.type_access_id,
                set_lang_id,
                conn,
            )?;

            res.push(UserAccessStandardAndRelatedData {
                standard_uuid: x.standard_uuid,
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
