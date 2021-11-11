use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::user_fav::model::UserFav;
use crate::models::user::model::ShowUserShort;
use crate::schema::user_fav::dsl as user_fav;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowUserShort {
    /// get list subscribers for user
    pub(crate) fn get_followers_by_user_uuid(
        logged_user_uuid: &Uuid,
        filter_users_uuids: &[Uuid],
        limit: &i32,
        offset: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut query = user_fav::user_fav.into_boxed();
        query = match filter_users_uuids.is_empty() {
            true => {
                query.filter(user_fav::user_favorite_uuid.eq(logged_user_uuid)
                    .and(user_fav::is_enabled.eq(true)))
            },
            // add filter user_uuid if it set
            false => {
                query.filter(user_fav::user_favorite_uuid.eq(logged_user_uuid)
                    .and(user_fav::is_enabled.eq(true)
                    .and(user_fav::user_follower_uuid.eq_any(filter_users_uuids))))
            },
        };

        let target_list_user_uuid = query
            .select(user_fav::user_follower_uuid)
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Fail load uuid list target user: {:?}", err);
                ServiceError::InternalServerError
            })?;

        ShowUserShort::get_users_by_uuids(
            logged_user_uuid,
            &target_list_user_uuid,
            conn
        )
    }

    /// get favorite list for user
    pub(crate) fn get_favorites_by_user_uuid(
        logged_user_uuid: &Uuid,
        filter_users_uuids: &[Uuid],
        limit: &i32,
        offset: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut query = user_fav::user_fav.into_boxed();
        query = match filter_users_uuids.is_empty() {
            true => {
                query.filter(user_fav::user_follower_uuid.eq(logged_user_uuid)
                    .and(user_fav::is_enabled.eq(true)))
            },
            // add filter user_uuid if it set
            false => {
                query.filter(user_fav::user_follower_uuid.eq(logged_user_uuid)
                    .and(user_fav::is_enabled.eq(true)
                    .and(user_fav::user_favorite_uuid.eq_any(filter_users_uuids))))
            },
        };

        let target_list_user_uuid = query
            .select(user_fav::user_favorite_uuid)
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Fail load uuid list target user: {:?}", err);
                ServiceError::InternalServerError
            })?;

        ShowUserShort::get_users_by_uuids(
            logged_user_uuid,
            &target_list_user_uuid,
            conn
        )
    }
}

impl UserFav {
    /// Count subscribers for user
    pub(crate) fn get_count_followers_by_uuid(
        logged_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(user_fav::user_fav
            .filter(user_fav::user_favorite_uuid.eq(logged_user_uuid)
            .and(user_fav::is_enabled.eq(true)))
            .execute(conn)? as i32)
    }

    /// Count favorite for user
    pub(crate) fn get_count_favorites_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<i32> {
        Ok(user_fav::user_fav
            .filter(user_fav::user_follower_uuid.eq(target_user_uuid)
            .and(user_fav::is_enabled.eq(true)))
            .execute(conn)? as i32)
    }
}
