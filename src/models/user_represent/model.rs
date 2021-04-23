use crate::schema::*;
// use chrono::*;
// use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct UserRepresent {
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
pub struct ShowUserRepresent {
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
#[table_name = "user_represent_ref"]
pub struct InsertableUserRepresent {
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct UserRepresentData {
    pub uuid_user: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimUserRepresent {
    pub uuid: Uuid,
    pub uuid_user: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<UserRepresentData> for InsertableUserRepresent {
    fn from(user_represent_data: UserRepresentData) -> Self {
        let UserRepresentData {
            uuid_user,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
            ..
        } = user_represent_data;

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

impl From<UserRepresent> for SlimUserRepresent {
    fn from(user_represent: UserRepresent) -> Self {
        let UserRepresent {
            uuid,
            uuid_user,
            name,
            address,
            phone,
            ..
        } = user_represent;

        Self {
            uuid,
            uuid_user,
            name,
            address,
            phone,
        }
    }
}
