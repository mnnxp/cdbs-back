use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct ComponentModification {
    pub id: i32,
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ShowComponentModification {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn uuid_modification_parent(&self) -> ID {
        self.uuid_modification_parent.into()
    }
    async fn modification_name(&self) -> String {
        self.modification_name.clone()
    }
    async fn description(&self) -> String {
        self.description.clone()
    }
    async fn id_actual_status(&self) -> i32 {
        self.id_actual_status.into()
    }
    async fn is_delete(&self) -> bool {
        self.is_delete.into()
    }
    async fn created_at(&self) -> NaiveDateTime {
        self.created_at.into()
    }
    async fn updated_at(&self) -> NaiveDateTime {
        self.updated_at.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "component_modification_list"]
pub struct InsertableComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct ComponentModificationData {
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
}

impl From<ComponentModificationData> for InsertableComponentModification {
    fn from(data_modification: ComponentModificationData) -> Self {
        let ComponentModificationData {
            uuid_component,
            uuid_modification_parent,
            modification_name,
            description,
            id_actual_status,
            ..
        } = data_modification;

        // let uuid_component = "a5953fd9-7393-4f1e-a899-06b5e159dbf1".parse().unwrap();
        // let id_name_cad = 1;
        // let uuid_modification_parent = "1".parse().unwrap();;
        // let commentchange = "Not change".to_owned();
        // let id_actual_status = 1;
        // let is_delete = 0;

        Self {
            uuid: Uuid::new_v4(),
            uuid_component,
            uuid_modification_parent,
            modification_name,
            description,
            id_actual_status,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<ComponentModification> for SlimComponentModification {
    fn from(data_modification: ComponentModification) -> Self {
        let ComponentModification {
            uuid,
            uuid_component,
            modification_name,
            description,
            updated_at,
            ..
        } = data_modification;

        Self {
            uuid,
            uuid_component,
            modification_name,
            description,
            updated_at,
        }
    }
}
