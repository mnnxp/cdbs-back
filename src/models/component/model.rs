// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
// use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub id_user: i32,
    pub comment: String,
    pub id_component_parent: i32,
    pub id_actual_status: i32,
    pub id_component_type: i32,
    pub is_delete: i32,
    pub id_type_access: i32,
    pub commentchange: String,
    pub is_standard: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "component_ref"]
pub struct InsertableComponent {
    pub name: String,
    pub id_user: i32,
    pub comment: String,
    pub id_component_parent: i32,
    pub id_actual_status: i32,
    pub id_component_type: i32,
    pub is_delete: i32,
    pub id_type_access: i32,
    pub commentchange: String,
    pub is_standard: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ComponentData {
    pub name: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimComponent {
    pub name: String,
    pub comment: String,
    pub id_actual_status: i32,
    pub created_at: NaiveDateTime,
}

impl From<ComponentData> for InsertableComponent {
    fn from(data_component: ComponentData) -> Self {
        let ComponentData {
            name,
            comment,
            ..
        } = data_component;

        let id_user = 1;
        let id_component_parent = 1;
        let id_actual_status = 1;
        let id_component_type = 1;
        let is_delete = 0;
        let id_type_access = 1;
        let commentchange = "Not change".to_owned();
        let is_standard = 0;

        Self {
            name,
            id_user,
            comment,
            id_component_parent,
            id_actual_status,
            id_component_type,
            is_delete,
            id_type_access,
            commentchange,
            is_standard,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<Component> for SlimComponent {
    fn from(component: Component) -> Self {
        let Component {
            name,
            comment,
            id_actual_status,
            created_at,
            ..
        } = component;

        Self {
            name,
            comment,
            id_actual_status,
            created_at,
        }
    }
}
