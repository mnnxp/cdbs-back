// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Queryable)]
pub struct ComponentModification {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub created_at: NaiveDateTime,
    pub id_name_cad: i32,
    pub comment: String,
    pub uuid_modification_parent: Uuid,
    pub commentchange: String,
    pub id_actual_status: i32,
    pub is_delete: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct ShowComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub created_at: NaiveDateTime,
    pub id_name_cad: i32,
    pub value_name_cad: String,
    pub comment: String,
    pub uuid_modification_parent: Uuid,
    pub commentchange: String,
    pub id_actual_status: i32,
    pub value_actual_status: String,
    pub is_delete: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "component_modification_list"]
pub struct InsertableComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub created_at: NaiveDateTime,
    pub id_name_cad: i32,
    pub comment: String,
    pub uuid_modification_parent: Uuid,
    pub commentchange: String,
    pub id_actual_status: i32,
    pub is_delete: i32,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ComponentModificationData {
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub uuid_modification_parent: Uuid,
    pub id_actual_status: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub uuid_modification_parent: Uuid,
    pub id_actual_status: i32,
    pub created_at: NaiveDateTime,
}

impl From<ComponentModificationData> for InsertableComponentModification {
    fn from(data_modification: ComponentModificationData) -> Self {
        let ComponentModificationData {
            uuid_component,
            modification_name,
            id_name_cad,
            comment,
            uuid_modification_parent,
            id_actual_status,
            ..
        } = data_modification;

        // let uuid_component = "a5953fd9-7393-4f1e-a899-06b5e159dbf1".parse().unwrap();
        // let id_name_cad = 1;
        // let uuid_modification_parent = "1".parse().unwrap();;
        let commentchange = "Not change".to_owned();
        // let id_actual_status = 1;
        let is_delete = 0;

        Self {
            uuid: Uuid::new_v4(),
            uuid_component,
            modification_name,
            created_at: chrono::Local::now().naive_local(),
            id_name_cad,
            comment,
            uuid_modification_parent,
            commentchange,
            id_actual_status,
            is_delete,
        }
    }
}

impl From<ComponentModification> for SlimComponentModification {
    fn from(data_modification: ComponentModification) -> Self {
        let ComponentModification {
            uuid,
            uuid_component,
            modification_name,
            comment,
            created_at,
            id_name_cad,
            uuid_modification_parent,
            id_actual_status,
            ..
        } = data_modification;

        Self {
            uuid,
            uuid_component,
            modification_name,
            comment,
            created_at,
            id_name_cad,
            uuid_modification_parent,
            id_actual_status,
        }
    }
}
