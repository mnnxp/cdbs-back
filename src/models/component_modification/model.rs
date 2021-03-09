// use crate::user::model::{LoggedUser, User};
use crate::schema::*;
use chrono::*;
// use uuid::Uuid;
// use num::ToPrimitive;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct ComponentModification {
    pub id: i32,
    pub id_component: i32,
    pub modification_name: String,
    pub created_at: NaiveDateTime,
    pub id_name_cad: i32,
    pub comment: String,
    pub id_modification_parent: i32,
    pub commentchange: String,
    pub id_actual_status: i32,
    pub is_delete: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "component_modification_list"]
pub struct InsertableComponentModification {
    pub id_component: i32,
    pub modification_name: String,
    pub created_at: NaiveDateTime,
    pub id_name_cad: i32,
    pub comment: String,
    pub id_modification_parent: i32,
    pub commentchange: String,
    pub id_actual_status: i32,
    pub is_delete: i32,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ComponentModificationData {
    pub modification_name: String,
    pub comment: String,
    pub id_modification_parent: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimComponentModification {
    pub id_component: i32,
    pub modification_name: String,
    pub id_name_cad: i32,
    pub comment: String,
    pub id_modification_parent: i32,
    pub id_actual_status: i32,
    pub created_at: NaiveDateTime,
}

impl From<ComponentModificationData> for InsertableComponentModification {
    fn from(data_modification: ComponentModificationData) -> Self {
        let ComponentModificationData {
            modification_name,
            comment,
            id_modification_parent,
            ..
        } = data_modification;

        let id_component = 1;
        let id_name_cad = 1;
        // let id_modification_parent = 1;
        let commentchange = "Not change".to_owned();
        let id_actual_status = 1;
        let is_delete = 0;

        Self {
            id_component,
            modification_name,
            created_at: chrono::Local::now().naive_local(),
            id_name_cad,
            comment,
            id_modification_parent,
            commentchange,
            id_actual_status,
            is_delete,
        }
    }
}

impl From<ComponentModification> for SlimComponentModification {
    fn from(data_modification: ComponentModification) -> Self {
        let ComponentModification {
            id_component,
            modification_name,
            comment,
            created_at,
            id_name_cad,
            id_modification_parent,
            id_actual_status,
            ..
        } = data_modification;

        Self {
            id_component,
            modification_name,
            comment,
            created_at,
            id_name_cad,
            id_modification_parent,
            id_actual_status,
        }
    }
}
