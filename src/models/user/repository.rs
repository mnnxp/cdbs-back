use super::{
    certificate::model::UserCertificateAndFile,
    model::{
        ShowUserAndRelatedData, ShowUserShort, SlimUser, UserAndRelatedData, UserQuery, UserShort,
    },
    relate::util::{count_companies_for_user, count_components_for_user, count_standards_for_user},
    user_fav::model::UserFav,
};
use crate::schema::user_ref::dsl as user_ref;
use crate::{
    auth::{require_permission, AccessEntity, AccessOperation},
    models::{
        company::company_fav::model::CompanyFav,
        component::component_fav::model::ComponentFav,
        relate_ref::{
            file::model::DownloadFile, program::model::Program, region::model::RegionTranslateList,
            type_access::model::TypeAccessTranslateList,
        },
        search::model::ExtraOptions,
        standard::standard_fav::model::StandardFav,
    },
};
use crate::{
    errors::{ServiceError, ServiceResult},
    models::search::order::Paginate,
};
use diesel::prelude::*;
use uuid::Uuid;

impl SlimUser {
    /// Get slim user data from user_ref table by uuid
    pub(crate) fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<SlimUser> {
        user_ref::user_ref
            .filter(
                user_ref::uuid
                    .eq(target_user_uuid)
                    .and(user_ref::is_enabled.eq(true))
                    .and(user_ref::is_delete.eq(false)),
            )
            .select((user_ref::uuid, user_ref::username, user_ref::program_id))
            .first::<SlimUser>(conn)
            .map_err(|err| {
                debug!("Failed get user: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl UserQuery {
    /// Get user data from user_ref table by uuid
    pub(crate) fn get_user_by_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<UserQuery> {
        user_ref::user_ref
            .filter(
                user_ref::uuid
                    .eq(target_user_uuid)
                    .and(user_ref::is_enabled.eq(true))
                    .and(user_ref::is_delete.eq(false)),
            )
            .select((
                user_ref::uuid,
                user_ref::email,
                user_ref::firstname,
                user_ref::lastname,
                user_ref::secondname,
                user_ref::username,
                user_ref::phone,
                user_ref::description,
                user_ref::address,
                user_ref::position,
                user_ref::time_zone,
                user_ref::image_file_uuid,
                user_ref::region_id,
                user_ref::program_id,
                user_ref::type_access_id,
                user_ref::is_email_verified,
                user_ref::created_at,
                user_ref::updated_at,
            ))
            .first::<UserQuery>(conn)
            .map_err(|err| {
                debug!("Failed get user: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl UserShort {
    /// get UserShort data for target uuid user
    pub(crate) fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<UserShort> {
        user_ref::user_ref
            .select((
                user_ref::uuid,
                user_ref::firstname,
                user_ref::lastname,
                user_ref::username,
                user_ref::image_file_uuid,
            ))
            .filter(
                user_ref::uuid
                    .eq(target_user_uuid)
                    .and(user_ref::is_enabled.eq(true))
                    .and(user_ref::is_delete.eq(false)),
            )
            .first::<UserShort>(conn)
            .map_err(|err| {
                debug!("Failed get user: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowUserShort {
    /// Gets ShowUserShort data by user_uuid with check access
    pub(crate) fn get_by_uuid(
        logged_user_uuid: &Uuid,
        target_user_uuid: &Uuid,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        require_permission(
            logged_user_uuid,
            AccessEntity::User,
            target_user_uuid,
            AccessOperation::Read,
            conn,
        )?;

        ShowUserShort::get_without_check_by_uuid(target_user_uuid, domain, conn)
    }

    /// Gets user short data by user_uuid wtihout check access
    pub(crate) fn get_without_check_by_uuid(
        target_user_uuid: &Uuid,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        let user_data = UserShort::get_by_uuid(target_user_uuid, conn)?;

        let mut data = ShowUserShort::new(&user_data);
        data.put_image_file(DownloadFile::get_by_file_uuid(
            &user_data.image_file_uuid,
            domain,
            conn,
        )?);

        Ok(data)
    }

    /// get ShowUserShort data of public users with filter by user_uuids
    pub(crate) fn get_all_public_users(
        search: &Option<String>,
        exclude_uuids: &Option<Vec<Uuid>>,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut query = user_ref::user_ref
            .filter(user_ref::type_access_id.eq(3))
            .filter(user_ref::is_enabled.eq(true))
            .filter(user_ref::is_delete.eq(false))
            .into_boxed();

        // Search by text
        if let Some(search_text) = search {
            if !search_text.is_empty() {
                let pattern = format!("%{}%", search_text);
                query = query.filter(
                    user_ref::username
                        .like(pattern.clone())
                        .or(user_ref::firstname.like(pattern.clone()))
                        .or(user_ref::lastname.like(pattern.clone()))
                        .or(user_ref::email.like(pattern.clone())),
                );
            }
        }

        // Exclusion of users by UUID
        if let Some(exclude_list) = exclude_uuids {
            if !exclude_list.is_empty() {
                query = query.filter(user_ref::uuid.ne_all(exclude_list));
            }
        }

        let users = query
            .select((
                user_ref::uuid,
                user_ref::firstname,
                user_ref::lastname,
                user_ref::username,
                user_ref::image_file_uuid,
            ))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .order_by(user_ref::username.asc())
            .load::<UserShort>(conn)
            .map_err(|err| {
                debug!("Failed get user_data: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut users_with_image: Vec<ShowUserShort> = Vec::new();
        for user in users.iter() {
            let mut data = ShowUserShort::new(user);
            data.put_image_file(DownloadFile::get_by_file_uuid(
                &user.image_file_uuid,
                domain,
                conn,
            )?);

            users_with_image.push(data);
        }

        Ok(users_with_image)
    }

    /// Gets users short data by uuids
    pub(crate) fn get_users_by_uuids(
        logged_user_uuid: &Uuid,
        target_users_uuids: &[Uuid],
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut result: Vec<ShowUserShort> = Vec::new();
        for target_user_uuid in target_users_uuids.iter() {
            match ShowUserShort::get_by_uuid(logged_user_uuid, target_user_uuid, domain, conn) {
                Ok(value) => result.push(value),
                Err(err) => {
                    debug!("Failed get user short data: {:?}", err);
                }
            }
        }

        Ok(result)
    }
}

impl UserAndRelatedData {
    /// Gathers full data for a user and related data by uuid
    pub(crate) fn collect_related_data(
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<UserAndRelatedData> {
        // collect data for user
        let user: UserQuery = UserQuery::get_user_by_uuid(&options.logged_user_uuid, conn)
            .expect("Error loading user");

        // get image file (favicon) for user
        let image_file =
            DownloadFile::get_by_file_uuid(&user.image_file_uuid, &options.domain, conn)
                .expect("Error loading user file");

        // get region for user
        let region: RegionTranslateList =
            RegionTranslateList::get_region_by_id(user.region_id, options.set_lang_id, conn)
                .expect("Error loading user_type");

        // get program set default for user
        let program: Program =
            Program::get_program_by_id(user.program_id, conn).expect("Error get set program");

        // get type access set for user profile
        let type_access: TypeAccessTranslateList = TypeAccessTranslateList::get_type_access_by_id(
            user.type_access_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error get set type access");

        // count subscribers user
        let subscribers: i32 = UserFav::get_count_followers_by_uuid(&user.uuid, conn)?;

        // get certificates with slimfile for user
        let certificates: Vec<UserCertificateAndFile> =
            UserCertificateAndFile::from_user(&user.uuid, &options.domain, conn)
                .expect("Error loading spec user with translate");

        // counting companies owned by the user
        let companies_count = count_companies_for_user(&options.logged_user_uuid, conn)
            .expect("Error get count companies_count");

        // counting components owned by the user
        let components_count = count_components_for_user(&options.logged_user_uuid, conn)
            .expect("Error get count components_count");

        // counting standards owned by the user
        let standards_count = count_standards_for_user(&options.logged_user_uuid, conn)
            .expect("Error get count standards_count");

        // counting companies in a user's favorite
        let fav_companies_count: i32 =
            CompanyFav::get_count_by_user_uuid(&options.logged_user_uuid, conn)
                .expect("Error get count fav_companies_count");

        // counting components in a user's favorite
        let fav_components_count: i32 =
            ComponentFav::get_count_by_user_uuid(&options.logged_user_uuid, conn)
                .expect("Error get count fav_components_count");

        // counting standards in a user's favorite
        let fav_standards_count: i32 =
            StandardFav::get_count_by_user_uuid(&options.logged_user_uuid, conn)
                .expect("Error get count fav_standards_count");

        // counting users in a user's favorite
        let fav_users_count: i32 =
            UserFav::get_count_favorites_by_uuid(&options.logged_user_uuid, conn)
                .expect("Error get count fav_users_count");

        Ok(UserAndRelatedData {
            uuid: user.uuid,
            email: user.email,
            firstname: user.firstname,
            lastname: user.lastname,
            secondname: user.secondname,
            username: user.username,
            phone: user.phone,
            description: user.description,
            address: user.address,
            position: user.position,
            time_zone: user.time_zone,
            image_file,
            region,
            program,
            type_access,
            is_email_verified: user.is_email_verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
            certificates,
            subscribers,
            companies_count: companies_count as i32,
            components_count: components_count as i32,
            standards_count: standards_count as i32,
            fav_companies_count,
            fav_components_count,
            fav_standards_count,
            fav_users_count,
        })
    }
}

impl ShowUserAndRelatedData {
    /// Gathers data for a user and related data by uuid
    pub(crate) fn collect_related_data(
        target_user_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        // collect data for user
        let user: UserQuery =
            UserQuery::get_user_by_uuid(target_user_uuid, conn).expect("Error loading user");

        // get image file (favicon) for user
        let image_file =
            DownloadFile::get_by_file_uuid(&user.image_file_uuid, &options.domain, conn)
                .expect("Error loading user file");

        // get region for user
        let region: RegionTranslateList =
            RegionTranslateList::get_region_by_id(user.region_id, options.set_lang_id, conn)
                .expect("Error loading user_type");

        // get program set default for user
        let program: Program =
            Program::get_program_by_id(user.program_id, conn).expect("Error get set program");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::user::user_fav::util::check_subscriber_by_uuid(
            target_user_uuid,
            &options.logged_user_uuid,
            conn,
        )
        .expect("Error get value is_followed");

        // count subscribers user
        let subscribers: i32 = UserFav::get_count_followers_by_uuid(&user.uuid, conn)?;

        // get certificates with slimfile for user
        let certificates: Vec<UserCertificateAndFile> =
            UserCertificateAndFile::from_user(&user.uuid, &options.domain, conn)
                .expect("Error loading spec user with translate");

        Ok(ShowUserAndRelatedData {
            uuid: user.uuid,
            firstname: user.firstname,
            lastname: user.lastname,
            secondname: user.secondname,
            username: user.username,
            description: user.description,
            position: user.position,
            image_file,
            region,
            program,
            created_at: user.created_at,
            updated_at: user.updated_at,
            certificates,
            subscribers,
            is_followed,
        })
    }

    /// Gets user with related data, with translate by uuid
    pub(crate) fn get_user_by_uuid(
        target_user_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::User,
            target_user_uuid,
            AccessOperation::Read,
            conn,
        )?;

        // collect data for user
        ShowUserAndRelatedData::collect_related_data(target_user_uuid, options, conn)
    }
}
