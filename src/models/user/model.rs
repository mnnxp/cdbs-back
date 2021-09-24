use super::util::{make_hash_salt, make_salt};
use super::certificate::model::CertificateWithShowFile;
use crate::models::relate_ref::file::model::ShowFile;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::program::model::Program;
use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct User {
    pub uuid: Uuid,
    pub email: String,
    pub psw_hash: Vec<u8>,
    pub psw_salt: String,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub username: String,
    pub phone: String,
    pub description: String,
    pub address: String,
    pub position: String, // todo!(in future: separate in table with translation)
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
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
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
    pub image_file: ShowFile,
    pub region: RegionTranslateList,
    pub program: Program,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub certificates: Vec<CertificateWithShowFile>,
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
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

#[derive(Debug, Insertable)]
#[table_name = "user_ref"]
pub struct InsertableUser {
    pub uuid: Uuid,
    pub email: String,
    pub psw_hash: Vec<u8>,
    pub psw_salt: String,
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
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUserData {
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
    pub region_id: i32,
    pub program_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
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

#[Object]
impl UserData {
    async fn email(&self) -> &String {
        &self.email
    }
    async fn firstname(&self) -> &String {
        &self.firstname
    }
    async fn lastname(&self) -> &String {
        &self.lastname
    }
    async fn secondname(&self) -> &String {
        &self.secondname
    }
    async fn username(&self) -> &String {
        &self.username
    }
    async fn phone(&self) -> &String {
        &self.phone
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn address(&self) -> &String {
        &self.address
    }
    async fn position(&self) -> &String {
        &self.position
    }
    async fn time_zone(&self) -> &String {
        &self.time_zone
    }
    async fn image_file_uuid(&self) -> ID {
        self.image_file_uuid.into()
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn program_id(&self) -> &i32 {
        &self.program_id
    }
}

impl From<UserData> for InsertableUser {
    fn from(user_data: UserData) -> Self {
        let UserData {
            email,
            password,
            firstname,
            lastname,
            secondname,
            username,
            phone,
            description,
            address,
            position,
            time_zone,
            image_file_uuid,
            region_id,
            program_id,
            ..
        } = user_data;

        let psw_salt = make_salt();
        let psw_hash = make_hash_salt(&password, &psw_salt).to_vec();

        Self {
            uuid: Uuid::new_v4(),
            email,
            psw_hash,
            psw_salt,
            firstname,
            lastname,
            secondname,
            username,
            phone,
            description,
            address,
            position,
            time_zone,
            image_file_uuid,
            region_id,
            program_id,
            is_email_verified: false,
            is_enabled: true,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct SlimUser {
    pub uuid: Uuid,
    pub program_id: i32,
    pub username: String,
}

#[Object]
impl SlimUser {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }

    async fn program_id(&self) -> &i32 {
        &self.program_id
    }

    async fn username(&self) -> &String {
        &self.username
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            program_id,
            username,
            ..
        } = user;

        Self {
            uuid,
            program_id,
            username,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct UserShort {
    pub uuid: Uuid,
    pub username: String,
    pub image_file_uuid: Uuid,
}

#[derive(Identifiable, Serialize, Deserialize, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "user_ref"]
pub struct ShowUserShort {
    pub uuid: Uuid,
    pub username: String,
    pub image_file: ShowFile,
}

#[Object]
impl ShowUserShort {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn username(&self) -> &String {
        &self.username
    }
    async fn image_file(&self) -> &ShowFile {
        &self.image_file
    }
}

impl From<(&UserShort, ShowFile)> for ShowUserShort {
    fn from(data: (&UserShort, ShowFile)) -> Self {
        Self {
            uuid: data.0.uuid,
            username: data.0.username.to_string(),
            image_file: data.1,
        }
    }
}
