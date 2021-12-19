use super::access::hash::{make_hash_salt, make_salt};
use super::certificate::model::UserCertificateAndFile;
use crate::models::relate_ref::{
    file::model::DownloadFile,
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
#[primary_key(uuid)]
#[table_name = "user_ref"]
pub struct UserQuery {
    pub uuid: Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub position: String,
    pub time_zone: String,
    pub image_file_uuid: Uuid,
    pub region_id: i32,
    pub program_id: i32,
    pub type_access_id: i32,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub struct UserAndRelatedData {
    pub uuid: Uuid,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub position: String, // <-- todo!(create a separate table with translation)
    pub time_zone: String,
    pub image_file: DownloadFile,
    pub region: RegionTranslateList,
    pub program: Program,
    pub type_access: TypeAccessTranslateList,
    pub is_email_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub certificates: Vec<UserCertificateAndFile>,
    pub subscribers: i32,
    // for a quick request just count objects have user
    pub companies_count: i32,
    pub components_count: i32,
    pub standards_count: i32,
    // for a quick request just count the subscribers
    pub fav_companies_count: i32,
    pub fav_components_count: i32,
    pub fav_standards_count: i32,
    pub fav_users_count: i32,
}

/// For show data about profile
#[derive(Debug, SimpleObject)]
pub struct ShowUserAndRelatedData {
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub description: String,
    pub position: String, // <-- todo!(create a separate table with translation)
    pub image_file: DownloadFile,
    pub region: RegionTranslateList,
    pub program: Program,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub certificates: Vec<UserCertificateAndFile>,
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Debug, Clone)]
pub struct UserData {
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub position: String,
    pub time_zone: String,
    pub image_file_uuid: Uuid,
    pub region_id: i32,
    pub program_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "user_ref"]
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserData {
    pub email: String,
    pub username: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub secondname: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub position: Option<String>,
    pub time_zone: Option<String>,
    pub region_id: Option<i32>,
    pub program_id: Option<i32>,
    pub type_access_id: Option<i32>,
}

impl From<&IptUserData> for InsertableUser {
    fn from(ipt_data: &IptUserData) -> Self {
        let IptUserData {
            email,
            username,
            password,
            firstname,
            lastname,
            secondname,
            phone,
            description,
            address,
            position,
            time_zone,
            region_id,
            program_id,
            type_access_id,
            ..
        } = ipt_data;

        let psw_salt = make_salt();
        let psw_hash = make_hash_salt(
            password.as_bytes(),
            &psw_salt,
        );

        // todo!(make fn for gets default uuid favicon)
        let image_file_uuid = Uuid::from_bytes([
            0xbc,0x1c,0x21,0x51,0x86,0xd0,0x46,0x56,0x9c,0x9d,0xd0,0x16,0xdd,0x58,0x42,0x97
        ]);

        // set default data
        let firstname = match firstname {
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let lastname = match lastname{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let secondname = match secondname{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let phone = match phone{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let description = match description{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let address = match address{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let position = match position{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let time_zone = match time_zone{
            Some(x) => x.to_string(),
            None => String::new(),
        };
        let region_id = region_id.unwrap_or(1);
        let program_id = program_id.unwrap_or(1);
        let type_access_id = type_access_id.unwrap_or(3);

        Self {
            uuid: Uuid::new_v4(),
            email: email.to_string(),
            psw_hash,
            psw_salt: psw_salt.to_vec(),
            firstname,
            lastname,
            secondname,
            username: username.to_string(),
            phone,
            description,
            address,
            position,
            time_zone,
            image_file_uuid, // default
            region_id,
            program_id,
            type_access_id,
            is_email_verified: false,
            is_enabled: true,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, SimpleObject)]
pub struct SlimUser {
    pub uuid: Uuid,
    pub username: String,
    pub program_id: i32,
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

#[derive(Identifiable, Serialize, Associations, Queryable, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "user_ref"]
pub struct UserShort {
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub image_file_uuid: Uuid,
}

#[derive(Clone, SimpleObject, Debug)]
pub struct ShowUserShort {
    pub uuid: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub image_file: DownloadFile,
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateUserData {
    pub email: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub secondname: Option<String>,
    pub username: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub position: Option<String>,
    pub time_zone: Option<String>,
    pub region_id: Option<i32>,
    pub program_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptUsersArg {
    pub users_uuids:  Option<Vec<Uuid>>,
    pub subscribers: Option<bool>,
    pub favorite: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct UsersArg {
    pub filter_users_uuids: Vec<Uuid>,
    pub subscribers: bool,
    pub favorite: bool,
    pub limit: i32,
    pub offset: i32,
}

impl Default for UsersArg {
    fn default() -> Self {
        Self {
            filter_users_uuids: Vec::new(),
            subscribers: false,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptUsersArg> for UsersArg {
    fn from(data: IptUsersArg) -> Self {
        let IptUsersArg {
            users_uuids,
            subscribers,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_users_uuids: users_uuids.unwrap_or_default(),
            subscribers: subscribers.unwrap_or(false),
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
