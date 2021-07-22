use crate::schema::*;
use crate::models::user::util::{make_hash_salt, make_salt};
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
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
    pub time_zone: i32,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowUser {
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
    pub time_zone: i32,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub time_zone: i32,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
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
    pub time_zone: i32,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_program: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimUser {
    pub uuid: Uuid,
    pub id_program: i32,
    pub username: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct LoggedUser(pub Option<SlimUser>);

impl From<SlimUser> for LoggedUser {
    fn from(slim_user: SlimUser) -> Self {
        LoggedUser(Some(slim_user))
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
