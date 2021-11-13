use crate::errors::{ServiceResult, ServiceError};
use super::model::{
    SlimUser,
    UserQuery,
    UserShort,
    ShowUserShort,
    UserAndRelatedData,
    ShowUserAndRelatedData,
};
use super::certificate::model::CertificateAndFile;
use super::user_fav::model::UserFav;
use super::access::util::check_access_user_for_user;
use crate::models::company::model::ShowCompanyShort;
use crate::models::component::model::ShowComponentShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::standard::standard_fav::model::StandardFav;
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::program::model::Program;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl SlimUser {
    /// Get slim user data from user_ref table by uuid
    pub(crate) fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<SlimUser> {
        Ok(user_ref::user_ref
            .filter(user_ref::uuid.eq(target_user_uuid)
            .and(user_ref::is_enabled.eq(true))
            .and(user_ref::is_delete.eq(false)))
            .select((
                user_ref::uuid,
                user_ref::username,
                user_ref::program_id,
            ))
            .first::<SlimUser>(conn)?)
    }
}

impl UserQuery {
    /// Get user data from user_ref table by uuid
    pub(crate) fn get_user_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserQuery> {
        Ok(user_ref::user_ref
            .filter(user_ref::uuid.eq(target_user_uuid)
            .and(user_ref::is_enabled.eq(true))
            .and(user_ref::is_delete.eq(false)))
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
                user_ref::is_enabled,
                user_ref::is_delete,
                user_ref::created_at,
                user_ref::updated_at,
            ))
            .first::<UserQuery>(conn)?)
    }
}

impl UserShort {
    /// get UserShort data for target uuid user
    pub(crate) fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserShort> {

    Ok(user_ref::user_ref
        .filter(user_ref::uuid.eq(target_user_uuid)
        .and(user_ref::is_enabled.eq(true))
        .and(user_ref::is_delete.eq(false)))
        .select((
            user_ref::uuid,
            user_ref::firstname,
            user_ref::lastname,
            user_ref::username,
            user_ref::image_file_uuid,
        ))
        .first::<UserShort>(conn)?)
    }
}

impl ShowUserShort {
    /// Gets ShowUserShort data by user_uuid with check access
    pub(crate) fn get_by_uuid(
        logged_user_uuid: &Uuid,
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        // check access user for target user
        check_access_user_for_user(
            logged_user_uuid,
            target_user_uuid,
            &need_access_level,
            conn
        )?;

        ShowUserShort::get_without_check_by_uuid(
            target_user_uuid,
            conn
        )
    }

    /// Gets user short data by user_uuid wtihout check access
    pub(crate) fn get_without_check_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        let user_data = user_ref::user_ref
            .filter(user_ref::uuid.eq(target_user_uuid)
            .and(user_ref::is_enabled.eq(true))
            .and(user_ref::is_delete.eq(false)))
            .select((
                user_ref::uuid,
                user_ref::firstname,
                user_ref::lastname,
                user_ref::username,
                user_ref::image_file_uuid,
            ))
            .first::<UserShort>(conn)
            .expect("Faile get user_data");

        Ok(ShowUserShort::from((
            &user_data,
            &DownloadFile::get_by_file_uuid(
                &user_data.image_file_uuid,
                conn
            ).expect("Failed get CertificateAndFile for ShowUserShort")
        )))
    }

    /// get ShowUserShort data of public users
    /// with filter by user_uuids
    pub(crate) fn get_all_public_users(
        limit: &i32,
        offset: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let users = user_ref::user_ref
            .filter(user_ref::type_access_id.eq(3)
            .and(user_ref::is_enabled.eq(true))
            .and(user_ref::is_delete.eq(false)))
            .limit(*limit as i64)
            .offset(*offset as i64)
            .select((
                user_ref::uuid,
                user_ref::firstname,
                user_ref::lastname,
                user_ref::username,
                user_ref::image_file_uuid,
            ))
            .load::<UserShort>(conn)
            .map_err(|err| {
                debug!("Faile get user_data: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut users_with_image: Vec<ShowUserShort> = Vec::new();
        for user in users.iter() {
            let download_favicon = DownloadFile::get_by_file_uuid(
                    &user.image_file_uuid,
                    conn
                ).unwrap();

            users_with_image.push(ShowUserShort::from((
                user,
                &download_favicon,
            )));
        }

        Ok(users_with_image)
    }

    /// Gets users short data by uuids
    pub(crate) fn get_users_by_uuids(
        logged_user_uuid: &Uuid,
        target_users_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut result: Vec<ShowUserShort> = Vec::new();
        for target_user_uuid in target_users_uuids.iter() {
            match ShowUserShort::get_by_uuid(
                logged_user_uuid,
                target_user_uuid,
                conn
            ) {
                Ok(value) => result.push(value),
                Err(err) => {
                    debug!("Failed get user short data: {:?}", err);
                },
            }
        }

        Ok(result)
    }
}

impl UserAndRelatedData {
    /// Collecting user data and related data using uuid
    pub(crate) fn collect_related_data(
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<UserAndRelatedData> {
        // collect data for user
        let user: UserQuery = UserQuery::get_user_by_uuid(
            target_user_uuid,
            conn
        ).expect("Error loading user");

        // get image file (favicon) for user
        let image_file = DownloadFile::get_by_file_uuid(
            &user.image_file_uuid,
            conn
        ).expect("Error loading user file");

        // get region for user
        let region: RegionTranslateList = RegionTranslateList::get_region_by_id(
            &user.region_id,
            set_lang_id,
            conn
        ).expect("Error loading user_type");

        // get program set default for user
        let program: Program = Program::get_program_by_id(
            &user.program_id,
            conn
        ).expect("Error get set program");

        // get type access set for user profile
        let type_access: TypeAccessTranslateList = TypeAccessTranslateList::get_type_access_by_id(
            &user.type_access_id,
            set_lang_id,
            conn
        ).expect("Error get set program");

        // count subscribers user
        let subscribers: i32 = UserFav::get_count_followers_by_uuid(&user.uuid, conn)?;

        // get certificates with slimfile for user
        let certificates: Vec<CertificateAndFile> = CertificateAndFile::from_user(
            &user.uuid,
            conn
        ).expect("Error loading spec user with translate");

        // counting companies owned by the user
        let companies_count: i32 = ShowCompanyShort::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count companies_count");

        // counting components owned by the user
        let components_count: i32 = ShowComponentShort::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count components_count");

        // counting standards owned by the user
        let standards_count: i32 = ShowStandardShort::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count standards_count");

        // counting companies in a user's favorite
        let fav_companies_count: i32 = CompanyFav::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count fav_companies_count");

        // counting components in a user's favorite
        let fav_components_count: i32 = ComponentFav::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count fav_components_count");

        // counting standards in a user's favorite
        let fav_standards_count: i32 = StandardFav::get_count_by_user_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count fav_standards_count");

        // counting users in a user's favorite
        let fav_users_count: i32 = UserFav::get_count_favorites_by_uuid(
            target_user_uuid,
            conn
        ).expect("Error get count fav_users_count");

        let result = UserAndRelatedData {
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
            is_enabled: user.is_enabled,
            is_delete: user.is_delete,
            created_at: user.created_at,
            updated_at: user.updated_at,
            certificates,
            subscribers,
            companies_count,
            components_count,
            standards_count,
            fav_companies_count,
            fav_components_count,
            fav_standards_count,
            fav_users_count,
        };

        Ok(result)
    }
}

impl ShowUserAndRelatedData {
    /// Collecting user data and related data using uuid
    pub(crate) fn collect_related_data(
        target_user_uuid: &Uuid,
        logged_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        // collect data for user
        let user: UserQuery = UserQuery::get_user_by_uuid(
            target_user_uuid,
            conn
        ).expect("Error loading user");

        // get image file (favicon) for user
        let image_file = DownloadFile::get_by_file_uuid(
            &user.image_file_uuid,
            conn
        ).expect("Error loading user file");

        // get region for user
        let region: RegionTranslateList = RegionTranslateList::get_region_by_id(
            &user.region_id,
            set_lang_id,
            conn
        ).expect("Error loading user_type");

        // get program set default for user
        let program: Program = Program::get_program_by_id(
            &user.program_id,
            conn
        ).expect("Error get set program");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::user::user_fav::util::check_subscriber_by_uuid(
            target_user_uuid,
            logged_user_uuid,
            conn
        ).expect("Error get value is_followed");

        // count subscribers user
        let subscribers: i32 = UserFav::get_count_followers_by_uuid(&user.uuid, conn)?;

        // get certificates with slimfile for user
        let certificates: Vec<CertificateAndFile> = CertificateAndFile::from_user(
            &user.uuid,
            conn
        ).expect("Error loading spec user with translate");

        let result = ShowUserAndRelatedData {
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
        };

        Ok(result)
    }

    /// Gets user with related data, with translate by uuid
    pub(crate) fn get_user_by_uuid(
        logged_user_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserAndRelatedData> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        // check access user for user
        check_access_user_for_user(
            logged_user_uuid,
            target_user_uuid,
            &need_access_level,
            conn
        )?;

        // collect data for user
        let result: ShowUserAndRelatedData = ShowUserAndRelatedData::collect_related_data(
            target_user_uuid,
            logged_user_uuid,
            set_lang_id,
            conn
        ).expect("Error loading user and collect related data");

        debug!("User data: {:#?}", result);

        Ok(result)
    }
}
