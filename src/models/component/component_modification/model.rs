use crate::schema::*;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[table_name = "component_modification_list"]
pub struct ComponentModification {
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
impl ComponentModification {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn uuid_modification_parent(&self) -> ID {
        self.uuid_modification_parent.into()
    }
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
    async fn is_delete(&self) -> &bool {
        &self.is_delete
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
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
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
    async fn is_delete(&self) -> &bool {
        &self.is_delete
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentModificationData {
    pub uuid_component: ID,
    pub uuid_modification_parent: ID,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ComponentModificationData {
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub id_actual_status: i32,
}

impl From<IptComponentModificationData> for ComponentModificationData {
    fn from(ipt_data: IptComponentModificationData) -> Self {
        let IptComponentModificationData {
            uuid_component,
            uuid_modification_parent,
            modification_name,
            description,
            id_actual_status,
        } = ipt_data;
        ComponentModificationData {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            uuid_modification_parent: Uuid::parse_str(&uuid_modification_parent.to_string()).unwrap(),
            modification_name,
            description,
            id_actual_status,
        }
    }
}

#[Object]
impl ComponentModificationData {
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn uuid_modification_parent(&self) -> ID {
        self.uuid_modification_parent.into()
    }
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimComponentModification {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub modification_name: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl SlimComponentModification {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
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
