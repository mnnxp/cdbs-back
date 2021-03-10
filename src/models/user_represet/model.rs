use crate::schema::*;
use chrono::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct UserRepreset {
    pub id: i32,
    pub id_user: i32,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Insertable)]
#[table_name = "user_represet_ref"]
pub struct InsertableUserRepreset {
    pub id_user: i32,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserRepresetData {
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUserRepreset {
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<UserRepresetData> for InsertableUserRepreset {
    fn from(user_represet_data: UserRepresetData) -> Self {
        let UserRepresetData {
            id_region,
            id_representation_type,
            name,
            address,
            phone,
            ..
        } = user_represet_data;

        let id_user = 1;

        Self {
            id_user,
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
            id_representation_type,
            name,
            address,
            phone,
            ..
        } = user_represet;

        Self {
            id_representation_type,
            name,
            address,
            phone,
        }
    }
}
