use crate::schema::*;
// use chrono::*;
// use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct UserRepreset {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct ShowUserRepreset {
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub value_region: String,
    pub id_representation_type: i32,
    pub value_representation_type: String,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Insertable)]
#[table_name = "user_represet_ref"]
pub struct InsertableUserRepreset {
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserRepresetData {
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUserRepreset {
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<UserRepresetData> for InsertableUserRepreset {
    fn from(user_represet_data: UserRepresetData) -> Self {
        let UserRepresetData {
            uuid_user,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
            ..
        } = user_represet_data;

        // let uuid_user = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b".parse().unwrap();

        Self {
            uuid: Uuid::new_v4(),
            uuid_user,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        }
    }
}

impl From<UserRepreset> for SlimUserRepreset {
    fn from(user_represet: UserRepreset) -> Self {
        let UserRepreset {
            uuid,
            uuid_user,
            name,
            address,
            phone,
            ..
        } = user_represet;

        Self {
            uuid,
            uuid_user,
            name,
            address,
            phone,
        }
    }
}
