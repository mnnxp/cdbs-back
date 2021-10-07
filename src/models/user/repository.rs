use super::model::{
    UserQuery,
    UserShort,
    ShowUserShort,
    UserAndRelatedData
};
use super::certificate::model::CertificateWithShowFile;
use super::user_fav::model::UserFav;
use crate::models::company::model::ShowCompanyShort;
use crate::models::component::model::ShowComponentShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::standard::standard_fav::model::StandardFav;
use crate::models::relate_ref::file::model::ShowFile;
use crate::models::relate_ref::program::model::Program;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::errors::ServiceResult;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl UserQuery {
    /// Get user data from user_ref table by uuid
    pub fn get_user_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserQuery> {
        Ok(user_ref::user_ref
            .filter(user_ref::uuid.eq(target_user_uuid))
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
    pub fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<UserShort> {

    Ok(user_ref::user_ref
        .filter(user_ref::uuid.eq(target_user_uuid))
        .select((
            user_ref::uuid,
            user_ref::username,
            user_ref::image_file_uuid,
        ))
        .first::<UserShort>(conn)?)
    }
}

impl ShowUserShort {
    /// get ShowUserShort data for target uuid user
    pub fn get_by_uuid(
        target_user_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<ShowUserShort> {
        let user_data = user_ref::user_ref
            .filter(user_ref::uuid.eq(target_user_uuid))
            .select((
                user_ref::uuid,
                user_ref::username,
                user_ref::image_file_uuid,
            ))
            .first::<UserShort>(conn)
            .expect("Faile get user_data");

        Ok(ShowUserShort::from((
            &user_data,
            &ShowFile::get_file_by_uuid(&user_data.image_file_uuid, conn)
                .expect("Failed get CertificateWithShowFile for ShowUserShort")
        )))
    }

    /// get ShowUserShort data for target list uuid user
    pub fn get_list_by_uuids(
        target_users_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        let mut show_users_short_data: Vec<ShowUserShort> = Vec::new();
        for target_user_uuid in target_users_uuids.iter() {
            show_users_short_data.push(ShowUserShort::get_by_uuid(
                target_user_uuid,
                conn
            )?)
        }

        Ok(show_users_short_data)
    }
}

impl UserAndRelatedData {
    /// Collecting user data and related data using uuid
    pub fn collect_related_data(
        target_user_uuid: &Uuid,
        logged_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<UserAndRelatedData> {
        // collect data for user
        let user: UserQuery = UserQuery::get_user_by_uuid(
            target_user_uuid,
            conn
        ).expect("Error loading user");

        // get image file (favicon) for user
        let image_file = ShowFile::get_file_by_uuid(&user.image_file_uuid, conn)
            .expect("Error loading user file");

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
        let certificates: Vec<CertificateWithShowFile> = CertificateWithShowFile::from_user(
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
        let fav_users_count: i32 = UserFav::get_count_followers_by_uuid(
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
            is_email_verified: user.is_email_verified,
            is_enabled: user.is_enabled,
            is_delete: user.is_delete,
            created_at: user.created_at,
            updated_at: user.updated_at,
            certificates,
            subscribers,
            is_followed,
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
