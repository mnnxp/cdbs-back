use super::util::{make_hash_salt, make_salt};
use super::certificate::model::CertificateWithSlimFile;
use crate::models::relate_ref::file::model::SlimFile;
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
    pub position: String,
    pub time_zone: String,
    pub uuid_image_file: Uuid,
    // pub id_type_access todo!(need add for user)
    pub id_region: i32,
    pub id_program: i32,
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
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
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
    pub image_file: SlimFile,
    pub region: RegionTranslateList,
    pub program: Program,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub certificates: Vec<CertificateWithSlimFile>,
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
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
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
    pub uuid_image_file: ID,
    pub id_region: i32,
    pub id_program: i32,
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
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
}

impl From<IptUserData> for UserData {
    fn from(ipt_data: IptUserData) -> Self {
        let IptUserData {
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
            uuid_image_file,
            id_region,
            id_program,
        } = ipt_data;
        UserData {
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
            uuid_image_file: Uuid::parse_str(&uuid_image_file.to_string()).unwrap(),
            id_region,
            id_program,
        }
    }
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
    async fn uuid_image_file(&self) -> ID {
        self.uuid_image_file.into()
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
    async fn id_program(&self) -> &i32 {
        &self.id_program
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
            uuid_image_file,
            id_region,
            id_program,
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
            uuid_image_file,
            id_region,
            id_program,
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
    pub id_program: i32,
    pub username: String,
}

#[Object]
impl SlimUser {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }

    async fn id_program(&self) -> &i32 {
        &self.id_program
    }

    async fn username(&self) -> &String {
        &self.username
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            id_program,
            username,
            ..
        } = user;

        Self {
            uuid,
            id_program,
            username,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct UserShort {
    pub uuid: Uuid,
    pub username: String,
    pub uuid_image_file: Uuid,
}

#[derive(Identifiable, Serialize, Deserialize, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[table_name = "user_ref"]
pub struct ShowUserShort {
    pub uuid: Uuid,
    pub username: String,
    pub image_file: SlimFile,
}

#[Object]
impl ShowUserShort {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn username(&self) -> &String {
        &self.username
    }
    async fn image_file(&self) -> &SlimFile {
        &self.image_file
    }
}

impl From<(&UserShort, SlimFile)> for ShowUserShort {
    fn from(data: (&UserShort, SlimFile)) -> Self {
        Self {
            uuid: data.0.uuid,
            username: data.0.username.to_string(),
            image_file: data.1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TargetUser (pub Uuid);

impl From<&Uuid> for TargetUser {
    fn from(data: &Uuid) -> Self {
        Self (data.to_owned())
    }
}
