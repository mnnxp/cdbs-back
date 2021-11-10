use crate::errors::ServiceResult;
use crate::models::standard::standard_fav::model::StandardFav;
// use crate::models::standard::model::ShowStandardShort;
use crate::schema::standard_fav::dsl as standard_fav;
use diesel::prelude::*;
use uuid::Uuid;

// impl ShowStandardShort {
//     /// get list subscribers for standard
//     pub(crate) fn get_followers_by_standard_uuid(
//         target_standard_uuid: &Uuid,
//         conn: &PgConnection,
//     ) -> ServiceResult<Vec<ShowStandardShort>> {
//         let target_list_user_uuid = standard_fav::standard_fav
//             .filter(standard_fav::standard_uuid.eq(target_standard_uuid))
//             .select(standard_fav::user_uuid)
//             .load::<Uuid>(conn)
//             .expect("Fail load uuid list target user");
//
//         ShowStandardShort::get_list_by_uuids(&target_list_user_uuid, conn)
//     }
// }

impl StandardFav {
    /// Count subscribers for standard
    pub(crate) fn get_count_followers_by_uuid(
        target_standard_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(standard_fav::standard_fav
            .filter(standard_fav::standard_uuid.eq(target_standard_uuid))
            .execute(conn)? as i32)
    }
}
