use crate::errors::ServiceResult;
use crate::models::standard::model::ShowStandardShort;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowStandardShort {
    /// get list subscribers for standard
    pub(crate) fn _get_by_owner_user_uuid(
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        let target_standards_uuids = standard_ref::standard_ref
            .filter(standard_ref::user_uuid.eq(target_user_uuid))
            .select(standard_ref::uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowStandardShort::get_list_by_uuids(
            &target_standards_uuids,
            target_user_uuid,
            set_lang_id,
            conn,
        )
    }

    /// Count subscribers for standard
    pub(crate) fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(standard_ref::standard_ref
            .filter(standard_ref::user_uuid.eq(target_user_uuid))
            .execute(conn)? as i32)
    }
}
