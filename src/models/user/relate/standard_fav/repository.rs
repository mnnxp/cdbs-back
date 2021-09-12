use crate::errors::ServiceResult;
use crate::models::standard::standard_fav::model::StandardFav;
use crate::models::standard::model::ShowStandardShort;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl StandardFav {
    /// get list subscribers for standard
    pub fn get_by_user_uuid(
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        let target_standards_uuids = standard_fav::standard_fav
            .filter(standard_fav::user_uuid.eq(target_user_uuid))
            .select(standard_fav::standard_uuid)
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
    pub fn get_count_by_user_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(standard_fav::standard_fav
            .filter(standard_fav::user_uuid.eq(target_user_uuid))
            .execute(conn)? as i32)
    }
}
