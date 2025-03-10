use super::access::hash::{make_hash_salt, make_salt};
use super::certificate::model::UserCertificateAndFile;
use crate::models::relate_ref::{
    file::model::DownloadFile,
    file::util::get_default_image,
    region::model::RegionTranslateList,
    program::model::Program,
    type_access::model::TypeAccessTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub(crate) struct User {
    uuid: Uuid,
    // email: String,
    psw_hash: Vec<u8>,
    psw_salt: Vec<u8>,
    // firstname: String,
    // lastname: String,
    // secondname: String,
    username: String,
    // phone: String,
    // description: String,
    // address: String,
    // position: String,
    // time_zone: String,
    // image_file_uuid: Uuid,
    // region_id: i32,
    program_id: i32,
    // type_access_id: i32,
    // is_email_verified: bool,
    // is_enabled: bool,
    // is_delete: bool,
    // created_at: NaiveDateTime,
    // updated_at: NaiveDateTime,
}

impl User {
    /// Gets password hash
    pub(super) fn get_psw_hash(&self) -> &[u8] {
        &self.psw_hash
    }

    /// Gets password salt
    pub(super) fn get_psw_salt(&self) -> &[u8] {
        &self.psw_salt
    }
}

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = user_ref)]
pub(crate) struct UserQuery {
    pub(crate) uuid: Uuid,
    pub(crate) email: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) secondname: String,
    pub(crate) username: String,
    pub(crate) phone: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) position: String,
    pub(crate) time_zone: String,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) program_id: i32,
    pub(crate) type_access_id: i32,
    pub(crate) is_email_verified: bool,
    // pub(crate) is_enabled: bool,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Full information about your own user profile
#[derive(Debug, SimpleObject)]
pub(crate) struct UserAndRelatedData {
    /// User UUID on the platform
    pub(crate) uuid: Uuid,
    /// User's email
    pub(crate) email: String,
    /// First name
    pub(crate) firstname: String,
    /// Surname
    pub(crate) lastname: String,
    /// Patronymic
    pub(crate) secondname: String,
    /// User name (nickname)
    pub(crate) username: String,
    /// User phone number
    pub(crate) phone: String,
    /// User description
    pub(crate) description: String,
    /// User's address
    pub(crate) address: String,
    /// User's position (job title/specialization)
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    /// User's time zone
    pub(crate) time_zone: String,
    /// Data for displaying the user's main image (avatar)
    pub(crate) image_file: DownloadFile,
    /// User region
    pub(crate) region: RegionTranslateList,
    /// User's main software tool (CAD, program)
    pub(crate) program: Program,
    /// User data access type
    pub(crate) type_access: TypeAccessTranslateList,
    /// E-mail confirmation result flag
    pub(crate) is_email_verified: bool,
    /// Date of user profile creation
    pub(crate) created_at: NaiveDateTime,
    /// Date of user's main data update
    pub(crate) updated_at: NaiveDateTime,
    // Связанные данные
    /// List of user's certificates and credentials
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    /// Number of users added to bookmarks
    pub(crate) subscribers: i32,
    // for a quick request just count objects have user
    /// Number of companies created by the user
    pub(crate) companies_count: i32,
    /// Number of components created by the user
    pub(crate) components_count: i32,
    /// Number of standards created by the user
    pub(crate) standards_count: i32,
    // for a quick request just count the subscribers
    /// Number of companies in user's bookmarks
    pub(crate) fav_companies_count: i32,
    /// Number of components in user's bookmarks
    pub(crate) fav_components_count: i32,
    /// Number of standards in user bookmarks
    pub(crate) fav_standards_count: i32,
    /// Number of users in user bookmarks
    pub(crate) fav_users_count: i32,
}

/// Full user profile information
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowUserAndRelatedData {
    /// User UUID on the platform
    pub(crate) uuid: Uuid,
    /// First name
    pub(crate) firstname: String,
    /// Surname
    pub(crate) lastname: String,
    /// Patronymic
    pub(crate) secondname: String,
    /// Username (nickname)
    pub(crate) username: String,
    /// User description
    pub(crate) description: String,
    /// User's position (position/specialization)
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    /// Data for displaying the user's main image (avatar)
    pub(crate) image_file: DownloadFile,
    /// User region
    pub(crate) region: RegionTranslateList,
    /// User's main software tool (CAD, program)
    pub(crate) program: Program,
    /// User profile creation date
    pub(crate) created_at: NaiveDateTime,
    /// Date of user's main data update
    pub(crate) updated_at: NaiveDateTime,
    // Связанные данные
    /// List of user's certificates and diplomas
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    /// Number of people who added the user to bookmarks
    pub(crate) subscribers: i32,
    /// Flag of the user's presence in the user's (viewer's) bookmarks
    pub(crate) is_followed: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_ref)]
pub(crate) struct InsertableUser {
    uuid: Uuid,
    email: String,
    psw_hash: Vec<u8>,
    psw_salt: Vec<u8>,
    firstname: String,
    lastname: String,
    secondname: String,
    username: String,
    phone: String,
    description: String,
    address: String,
    position: String,
    time_zone: String,
    image_file_uuid: Uuid,
    region_id: i32,
    program_id: i32,
    type_access_id: i32,
    is_email_verified: bool,
    is_enabled: bool,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

/// Data for adding a new user
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUserData {
    /// User's e-mail
    pub(crate) email: String,
    /// Username (nickname)
    pub(crate) username: String,
    /// Password
    pub(crate) password: String,
    /// First name (optional)
    pub(crate) firstname: Option<String>,
    /// Last name (optional)
    pub(crate) lastname: Option<String>,
    /// Patronymic (optional)
    pub(crate) secondname: Option<String>,
    /// User phone number (optional)
    pub(crate) phone: Option<String>,
    /// User description (optional)
    pub(crate) description: Option<String>,
    /// User's address (optional)
    pub(crate) address: Option<String>,
    /// User's position (job title/specialization) (optional)
    pub(crate) position: Option<String>,
    /// User's time zone (optional)
    pub(crate) time_zone: Option<String>,
    /// User Region ID (optional)
    pub(crate) region_id: Option<i32>,
    /// Identifier of the user's main software (for example, some CAD software) (optional)
    pub(crate) program_id: Option<i32>,
    /// User data access type identifier (optional)
    pub(crate) type_access_id: Option<i32>,
}

impl InsertableUser {
    pub(crate) fn by_arg(ipt_data: IptUserData) -> Self {
        let psw_salt = make_salt();
        let psw_hash = make_hash_salt(
            ipt_data.password.as_bytes(),
            &psw_salt,
        );

        Self {
            uuid: Uuid::new_v4(),
            email: ipt_data.email.to_string(),
            psw_hash,
            psw_salt: psw_salt.to_vec(),
            firstname: ipt_data.firstname.unwrap_or_default(),
            lastname: ipt_data.lastname.unwrap_or_default(),
            secondname: ipt_data.secondname.unwrap_or_default(),
            username: ipt_data.username.trim().to_string(),
            phone: ipt_data.phone.unwrap_or_default(),
            description: ipt_data.description.unwrap_or_default(),
            address: ipt_data.address.unwrap_or_default(),
            position: ipt_data.position.unwrap_or_default(),
            time_zone: ipt_data.time_zone.unwrap_or_default(),
            // default favicon image
            image_file_uuid: get_default_image(),
            region_id: ipt_data.region_id.unwrap_or(1),
            program_id: ipt_data.program_id.unwrap_or(1),
            type_access_id: ipt_data.type_access_id.unwrap_or(3),
            is_email_verified: false,
            is_enabled: true,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Minimum user data
#[derive(Debug, Serialize, Deserialize, Queryable, Clone, SimpleObject)]
pub(crate) struct SlimUser {
    /// User UUID on the platform
    pub(crate) uuid: Uuid,
    /// User name (nickname)
    pub(crate) username: String,
    /// Identifier of the user's main software tool
    pub(crate) program_id: i32,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            username,
            program_id,
            ..
        } = user;

        Self {
            uuid,
            username,
            program_id,
        }
    }
}

#[derive(Identifiable, Serialize, Queryable, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = user_ref)]
pub(crate) struct UserShort {
    pub(crate) uuid: Uuid,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) username: String,
    pub(crate) image_file_uuid: Uuid,
}

#[derive(Clone, SimpleObject, Debug)]
pub(crate) struct ShowUserShort {
    pub(crate) uuid: Uuid,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) username: String,
    pub(crate) image_file: DownloadFile,
}

impl ShowUserShort {
    /// Create struct with UserShort data, DownloadFile data set default
    pub(crate) fn new(data: &UserShort) -> Self {
        Self{
            uuid: data.uuid,
            firstname: data.firstname.clone(),
            lastname: data.lastname.clone(),
            username: data.username.clone(),
            image_file: Default::default(),
        }
    }

    /// Change image_file data
    pub(crate) fn put_image_file(&mut self, image_file: DownloadFile) {
        self.image_file = image_file;
    }
}

/// Data for updating the user profile.
/// The data is updated only for the specified values.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateUserData {
    /// User's e-mail
    pub(crate) email: Option<String>,
    /// First name
    pub(crate) firstname: Option<String>,
    /// Surname
    pub(crate) lastname: Option<String>,
    /// Patronymic
    pub(crate) secondname: Option<String>,
    /// Username (nickname)
    pub(crate) username: Option<String>,
    /// User phone number
    pub(crate) phone: Option<String>,
    /// User description
    pub(crate) description: Option<String>,
    /// User's address
    pub(crate) address: Option<String>,
    /// User's position (job title/specialization)
    pub(crate) position: Option<String>,
    /// User's time zone
    pub(crate) time_zone: Option<String>,
    /// User region identifier
    pub(crate) region_id: Option<i32>,
    /// Identifier of the user's main software (e.g. some CAD software)
    pub(crate) program_id: Option<i32>,
}

/// Arguments for filtering and searching by company
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptUsersArg {
    /// Filter users by UUID
    pub(crate) users_uuids: Option<Vec<Uuid>>,
    /// Filter by active user's subscribers
    pub(crate) subscribers: Option<bool>,
    /// Filter by the presence of users in favorites of the active user
    pub(crate) favorite: Option<bool>,
}

#[derive(Debug, Default)]
pub(crate) struct UsersArg {
    pub(crate) filter_users_uuids: Vec<Uuid>,
    pub(crate) subscribers: bool,
    pub(crate) favorite: bool,
}

impl From<IptUsersArg> for UsersArg {
    fn from(data: IptUsersArg) -> Self {
        let IptUsersArg {
            users_uuids,
            subscribers,
            favorite,
        } = data;

        Self {
            filter_users_uuids: users_uuids.unwrap_or_default(),
            subscribers: subscribers.unwrap_or(false),
            favorite: favorite.unwrap_or(false),
        }
    }
}

/// Data to request complete user profile data
/// One of two options is specified
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptGetUserArg {
    /// User UUID for data retrieval
    pub(crate) user_uuid: Option<Uuid>,
    /// User name (alias) for data retrieval
    pub(crate) username: Option<String>,
}
