use crate::errors::ServiceResult;
use crate::models::user::user_fav::model::UserFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl UserFav {
    /// get list subscribers for user
    pub fn get_list_followers_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let target_list_user_uuid = user_fav::user_fav
            .filter(user_fav::user_follower_uuid.eq(target_user_uuid))
            .select(user_fav::user_follower_uuid)
            .load::<Uuid>(conn)
            .expect("Fail load uuid list target user");

        ShowUserShort::get_list_by_uuids(&target_list_user_uuid, conn)
    }

    /// Count subscribers for user
    pub fn get_count_followers_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(user_fav::user_fav
            .filter(user_fav::user_follower_uuid.eq(target_user_uuid))
            .execute(conn)? as i32)
    }
}
