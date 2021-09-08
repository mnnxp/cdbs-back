use crate::errors::ServiceResult;
use crate::models::standard::standard_fav::model::StandardFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl StandardFav {
    /// get list subscribers for standard
    pub fn get_list_followers_by_uuid(
        target_uuid_standard: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let target_list_uuid_user = standard_fav::standard_fav
            .filter(standard_fav::uuid_standard.eq(target_uuid_standard))
            .select(standard_fav::uuid_user)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowUserShort::get_list_by_uuids(&target_list_uuid_user, conn)
    }

    /// Count subscribers for standard
    pub fn get_count_followers_by_uuid(
        target_uuid_standard: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(standard_fav::standard_fav
            .filter(standard_fav::uuid_standard.eq(target_uuid_standard))
            .execute(conn)? as i32)
    }
}
