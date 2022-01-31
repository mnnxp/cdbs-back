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
#[primary_key(uuid)]
#[table_name = "user_ref"]
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

#[derive(Debug, SimpleObject)]
pub(crate) struct UserAndRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) email: String,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) secondname: String,
    pub(crate) username: String,
    pub(crate) phone: String,
    pub(crate) description: String,
    pub(crate) address: String,
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    pub(crate) time_zone: String,
    pub(crate) image_file: DownloadFile,
    pub(crate) region: RegionTranslateList,
    pub(crate) program: Program,
    pub(crate) type_access: TypeAccessTranslateList,
    pub(crate) is_email_verified: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
    // related data
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    pub(crate) subscribers: i32,
    // for a quick request just count objects have user
    pub(crate) companies_count: i32,
    pub(crate) components_count: i32,
    pub(crate) standards_count: i32,
    // for a quick request just count the subscribers
    pub(crate) fav_companies_count: i32,
    pub(crate) fav_components_count: i32,
    pub(crate) fav_standards_count: i32,
    pub(crate) fav_users_count: i32,
}

/// For show data about profile
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowUserAndRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) firstname: String,
    pub(crate) lastname: String,
    pub(crate) secondname: String,
    pub(crate) username: String,
    pub(crate) description: String,
    pub(crate) position: String, // <-- todo!(create a separate table with translation)
    pub(crate) image_file: DownloadFile,
    pub(crate) region: RegionTranslateList,
    pub(crate) program: Program,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
    // related data
    pub(crate) certificates: Vec<UserCertificateAndFile>,
    pub(crate) subscribers: i32,
    // for display the checkbox "favorites"
    pub(crate) is_followed: bool,
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
pub(crate) struct IptUserData {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
    pub(crate) secondname: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) position: Option<String>,
    pub(crate) time_zone: Option<String>,
    pub(crate) region_id: Option<i32>,
    pub(crate) program_id: Option<i32>,
    pub(crate) type_access_id: Option<i32>,
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
            // default favicon image
            image_file_uuid: get_default_image(),
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
pub(crate) struct SlimUser {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
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

#[derive(Identifiable, Serialize, Associations, Queryable, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "user_ref"]
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateUserData {
    pub(crate) email: Option<String>,
    pub(crate) firstname: Option<String>,
    pub(crate) lastname: Option<String>,
    pub(crate) secondname: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) phone: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) position: Option<String>,
    pub(crate) time_zone: Option<String>,
    pub(crate) region_id: Option<i32>,
    pub(crate) program_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptUsersArg {
    pub(crate) users_uuids:  Option<Vec<Uuid>>,
    pub(crate) subscribers: Option<bool>,
    pub(crate) favorite: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct UsersArg {
    pub(crate) filter_users_uuids: Vec<Uuid>,
    pub(crate) subscribers: bool,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptGetUserArg {
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) username: Option<String>,
}
