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
pub struct DataComponent {
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

impl From<DataComponent> for InsertableComponent {
    fn from(data_for_component: DataComponent) -> Self {
        let DataComponent {
            name,
            id_user,
            comment,
            id_component_parent,
            id_actual_status,
            id_component_type,
            is_delete,
            id_type_access,
            commentchange,
            ..
        } = data_for_component;
        
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
            is_standard: 0,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}
