use crate::schema::*;
use crate::user::util::{make_hash, make_salt};
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub email_verified: i32,
    #[graphql(skip)]
    pub psw_hash: Vec<u8>,
    #[graphql(skip)]
    pub psw_salt: String,
    pub id_type_org: i32,
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
    pub time_zone: String,
    pub position: String,
    pub site_url: String,
    pub id_file_info_icon: i32,
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
    pub id_type_org: i32,
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
    pub time_zone: String,
    pub position: String,
    pub site_url: String,
    pub id_file_info_icon: i32,
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
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUser {
    pub uuid: Uuid,
    pub email: String,
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
            ..
        } = user_data;

        let psw_salt = make_salt();
        let psw_hash = make_hash(&password, &psw_salt).to_vec();
        Self {
            uuid: Uuid::new_v4(),
            email,
            email_verified: 0,
            psw_hash,
            psw_salt,
            id_type_org: 0,
            firstname: "A".to_owned(),
            lastname: "A".to_owned(),
            secondname: "A".to_owned(),
            nickname: "A".to_owned(),
            orgname: "A".to_owned(),
            shortname: "A".to_owned(),
            inn: "A".to_owned(),
            phone: "A".to_owned(),
            id_name_cad: 0,
            comment: "A".to_owned(),
            address: "A".to_owned(),
            time_zone: "A".to_owned(),
            position: "A".to_owned(),
            site_url: "A".to_owned(),
            id_file_info_icon: 0,
            id_region: 0,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        let User {
            uuid,
            email,
            ..
        } = user;

        Self {
            uuid,
            email,
        }
    }
}
