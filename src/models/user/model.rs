use crate::schema::*;
use crate::models::user::util::{make_hash_salt, make_salt};
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    #[graphql(skip)]
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub email_verified: i32,
    #[graphql(skip)]
    pub psw_hash: Vec<u8>,
    #[graphql(skip)]
    pub psw_salt: String,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub nickname: String,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: Uuid,
    pub id_region: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "user_ref"]
pub struct InsertableUser {
    pub uuid: Uuid,
    pub email: String,
    pub email_verified: i32,
    pub psw_hash: Vec<u8>,
    pub psw_salt: String,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub nickname: String,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: Uuid,
    pub id_region: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserData {
    pub firstname: String,
    pub lastname: String,
    pub secondname: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub id_type_user: i32,
    pub is_supplier: i32,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub address: String,
    pub time_zone: i32,
    pub position: String,
    pub site_url: String,
    pub uuid_file_info_icon: Uuid,
    pub id_region: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub uuid: Uuid,
    pub is_supplier: i32,
    pub nickname: String,
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
            firstname,
            lastname,
            secondname,
            nickname,
            email,
            password,
            id_type_user,
            is_supplier,
            orgname,
            shortname,
            inn,
            phone,
            id_name_cad,
            comment,
            address,
            time_zone,
            position,
            site_url,
            uuid_file_info_icon,
            id_region,
            ..
        } = user_data;

        let psw_salt = make_salt();
        let psw_hash = make_hash_salt(&password, &psw_salt).to_vec();

        Self {
            uuid: Uuid::new_v4(),
            email,
            email_verified: 0,
            psw_hash,
            psw_salt,
            id_type_user,
            is_supplier,
            firstname,
            lastname,
            secondname,
            nickname,
            orgname,
            shortname,
            inn,
            phone,
            id_name_cad,
            comment,
            address,
            time_zone,
            position,
            site_url,
            uuid_file_info_icon,
            id_region,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            is_supplier,
            nickname,
            ..
        } = user;

        Self {
            uuid,
            is_supplier,
            nickname,
        }
    }
}
