use crate::errors::ServiceResult;
// use crate::models::user::model::SlimUser;
use crate::models::user::model::{UserShort, ShowUserShort};
use crate::models::relate_ref::file::model::SlimFile;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

// impl SlimUser {
//     /// get SlimUser data for target uuid user
//     pub fn get_by_uuid(
//         target_uuid_user: &Uuid,
//         conn: &PgConnection,
//     ) -> ServiceResult<SlimUser> {
//
//     Ok(user_ref::user_ref
//         .filter(user_ref::uuid.eq(target_uuid_user))
//         .select((
//             user_ref::uuid,
//             user_ref::id_program,
//             user_ref::username,
//         ))
//         .first::<SlimUser>(conn)?)
//     }
//
//
//     /// get SlimUser data for target list uuid user
//     pub fn get_list_by_uuids(
//         target_list_uuid_user: &[Uuid],
//         conn: &PgConnection,
//     ) -> ServiceResult<Vec<SlimUser>> {
//
//     Ok(user_ref::user_ref
//         .filter(user_ref::uuid.eq_any(target_list_uuid_user))
//         .select((
//             user_ref::uuid,
//             user_ref::id_program,
//             user_ref::username,
//         ))
//         .load::<SlimUser>(conn)?)
//     }
// }


impl ShowUserShort {
    /// get ShowUserShort data for target uuid user
    pub fn get_by_uuid(
        target_uuid_user: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        let user_data = user_ref::user_ref
            .filter(user_ref::uuid.eq(target_uuid_user))
            .select((
                user_ref::uuid,
                user_ref::username,
                user_ref::uuid_image_file,
            ))
            .first::<UserShort>(conn)
            .expect("Faile get user_data");

        Ok(ShowUserShort::from((
            &user_data,
            SlimFile::get_file_by_uuid(&user_data.uuid_image_file, conn)
                .expect("Failed get SlimFile for ShowUserShort")
        )))
    }

    /// get ShowUserShort data for target list uuid user
    pub fn get_list_by_uuids(
        target_list_uuid_user: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let list_users_data = user_ref::user_ref
            .filter(user_ref::uuid.eq_any(target_list_uuid_user))
            .select((
                user_ref::uuid,
                user_ref::username,
                user_ref::uuid_image_file,
            ))
            .load::<UserShort>(conn)
            .expect("Faile get list_users_data");

        let mut show_users_short_data: Vec<ShowUserShort> = Vec::new();
        for user_data in list_users_data.iter() {
            show_users_short_data.push(ShowUserShort::from((
                user_data,
                SlimFile::get_file_by_uuid(&user_data.uuid_image_file, conn)
                    .expect("Failed get SlimFile for ShowUserShort")
            )))
        }

        Ok(show_users_short_data)
    }
}
