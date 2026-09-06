use crate::errors::{ServiceError, ServiceResult};
use crate::models::search::order::Paginate;
use crate::models::user::model::ShowUserShort;
use crate::models::user::user_fav::model::UserFav;
use crate::schema::user_fav::dsl as user_fav;
use crate::schema::user_ref::dsl as user_ref;
use crate::subscribers_count;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowUserShort {
    /// Get list of users who follow (subscribe to) the specified user.
    /// Supports text search and exclusion filters.
    pub(crate) fn get_followers_by_user_uuid(
        logged_user_uuid: &Uuid,
        filter_users_uuids: &[Uuid],
        search: &Option<String>,
        exclude_uuids: &Option<Vec<Uuid>>,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut query = user_fav::user_fav
            .inner_join(user_ref::user_ref.on(user_fav::user_follower_uuid.eq(user_ref::uuid)))
            .filter(user_fav::user_favorite_uuid.eq(logged_user_uuid))
            .filter(user_fav::is_enabled.eq(true))
            .filter(user_ref::is_enabled.eq(true))
            .filter(user_ref::is_delete.eq(false))
            .into_boxed();

        // Apply UUID filter if provided
        if !filter_users_uuids.is_empty() {
            query = query.filter(user_fav::user_follower_uuid.eq_any(filter_users_uuids));
        }

        // Apply text search
        if let Some(search_text) = search {
            if !search_text.is_empty() {
                let pattern = format!("%{}%", search_text);
                query = query.filter(
                    user_ref::username
                        .like(pattern.clone())
                        .or(user_ref::firstname.like(pattern.clone()))
                        .or(user_ref::lastname.like(pattern.clone()))
                        .or(user_ref::email.like(pattern)),
                );
            }
        }

        // Apply exclusion filter
        if let Some(exclude_list) = exclude_uuids {
            if !exclude_list.is_empty() {
                query = query.filter(user_fav::user_follower_uuid.ne_all(exclude_list));
            }
        }

        let target_list_user_uuid = query
            .select(user_ref::uuid)
            .limit(paginate.limit)
            .offset(paginate.offset)
            .order_by(user_ref::username.asc())
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get followers: {:?}", err);
                ServiceError::InternalServerError
            })?;

        ShowUserShort::get_users_by_uuids(logged_user_uuid, &target_list_user_uuid, domain, conn)
    }

    /// Get list of users that the specified user has favorited.
    /// Supports text search and exclusion filters.
    pub(crate) fn get_favorites_by_user_uuid(
        logged_user_uuid: &Uuid,
        filter_users_uuids: &[Uuid],
        search: &Option<String>,
        exclude_uuids: &Option<Vec<Uuid>>,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut query = user_fav::user_fav
            .inner_join(user_ref::user_ref.on(user_fav::user_favorite_uuid.eq(user_ref::uuid)))
            .filter(user_fav::user_follower_uuid.eq(logged_user_uuid))
            .filter(user_fav::is_enabled.eq(true))
            .filter(user_ref::is_enabled.eq(true))
            .filter(user_ref::is_delete.eq(false))
            .into_boxed();

        if !filter_users_uuids.is_empty() {
            query = query.filter(user_fav::user_favorite_uuid.eq_any(filter_users_uuids));
        }

        if let Some(search_text) = search {
            if !search_text.is_empty() {
                let pattern = format!("%{}%", search_text);
                query = query.filter(
                    user_ref::username
                        .like(pattern.clone())
                        .or(user_ref::firstname.like(pattern.clone()))
                        .or(user_ref::lastname.like(pattern.clone()))
                        .or(user_ref::email.like(pattern)),
                );
            }
        }

        if let Some(exclude_list) = exclude_uuids {
            if !exclude_list.is_empty() {
                query = query.filter(user_fav::user_favorite_uuid.ne_all(exclude_list));
            }
        }

        let target_list_user_uuid = query
            .select(user_ref::uuid)
            .limit(paginate.limit)
            .offset(paginate.offset)
            .order_by(user_ref::username.asc())
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get favorites: {:?}", err);
                ServiceError::InternalServerError
            })?;

        ShowUserShort::get_users_by_uuids(logged_user_uuid, &target_list_user_uuid, domain, conn)
    }
}

impl UserFav {
    /// Count subscribers for user
    pub(crate) fn get_count_followers_by_uuid(
        logged_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        subscribers_count!(user_fav, user_favorite_uuid, logged_user_uuid, conn)
    }

    /// Count favorite for user
    pub(crate) fn get_count_favorites_by_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<i32> {
        subscribers_count!(user_fav, user_follower_uuid, target_user_uuid, conn)
    }
}
