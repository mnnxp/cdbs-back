// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Queryable)]
pub struct Component {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_component_parent: Uuid,
    pub name: String,
    pub description: String,
    pub uuid_user: Uuid,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct ShowComponent {
    pub uuid: Uuid,
    pub uuid_component_parent: Uuid,
    pub name: String,
    pub description: String,
    pub uuid_user: Uuid,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "component_ref"]
pub struct InsertableComponent {
    pub uuid: Uuid,
    pub uuid_component_parent: Uuid,
    pub name: String,
    pub description: String,
    pub uuid_user: Uuid,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct ComponentData {
    pub uuid_component_parent: Uuid,
    pub name: String,
    pub description: String,
    pub uuid_user: Uuid,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ComponentDataQuery {
    pub uuid_component_parent: String,
    pub name: String,
    pub description: String,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimComponent {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
    pub created_at: NaiveDateTime,
}

impl From<ComponentData> for InsertableComponent {
    fn from(data_component: ComponentData) -> Self {
        let ComponentData {
            uuid_component_parent,
            name,
            description,
            uuid_user,
            id_type_access,
            id_component_type,
            id_actual_status,
            is_standard,
            ..
        } = data_component;

        // let uuid_user = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b".parse().unwrap();
        // let uuid_component_parent =  "a5953fd9-7393-4f1e-a899-06b5e159dbf1".parse().unwrap();
        // let id_actual_status = 1;
        // let id_component_type = 1;
        // let is_delete = 0;
        // let id_type_access = 1;
        // let commentchange = "Not change".to_owned();
        // let is_standard = 0;

        Self {
            uuid: Uuid::new_v4(),
            uuid_component_parent,
            name,
            description,
            uuid_user,
            id_type_access,
            id_component_type,
            id_actual_status,
            is_standard,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<Component> for SlimComponent {
    fn from(component: Component) -> Self {
        let Component {
            uuid,
            name,
            description,
            id_type_access,
            id_component_type,
            id_actual_status,
            is_standard,
            created_at,
            ..
        } = component;

        Self {
            uuid,
            name,
            description,
            id_type_access,
            id_component_type,
            id_actual_status,
            is_standard,
            created_at,
        }
    }
}
