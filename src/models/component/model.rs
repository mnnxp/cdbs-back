use crate::schema::*;
use crate::models::user::model::SlimUser;
use crate::models::component::relate::component_type::model::ComponentTypeTranslateList;
use crate::models::component::relate::actual_status::model::ActualStatusTranslateList;
use crate::models::component::param::model::ComponentParamWithTranslation;
use crate::models::component::component_modification::model::ComponentModificationAndRelatedData;
// use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::license::model::License;
use crate::models::file::model::ShowFile;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "component_ref"]
pub struct Component {
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

#[Object]
impl Component {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_component_parent(&self) -> ID {
        self.uuid_component_parent.into()
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn id_type_access(&self) -> &i32 {
        &self.id_type_access
    }
    async fn id_component_type(&self) -> &i32 {
        &self.id_component_type
    }
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
    async fn is_standard(&self) -> &bool {
        &self.is_standard
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

#[derive(Debug, Deserialize, SimpleObject, Description)]
pub struct ComponentAndRelatedData {
    pub uuid: Uuid,
    pub uuid_component_parent: Uuid,
    pub name: String,
    pub description: String,
    pub slim_user: SlimUser,
    pub id_type_access: i32, //TypeAccess
    pub component_type: ComponentTypeTranslateList,
    pub actual_status: ActualStatusTranslateList,
    pub is_standard: bool,
    pub updated_at: NaiveDateTime,
    pub license: Vec<License>,
    pub param_component: Vec<ComponentParamWithTranslation>,
    pub file: Vec<ShowFile>,
    pub component_modification: Vec<ComponentModificationAndRelatedData>,
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

#[derive(Debug, Deserialize, InputObject)]
pub struct IptComponentData {
    pub uuid_component_parent: ID,
    pub name: String,
    pub description: String,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimComponent {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub id_type_access: i32,
    pub id_component_type: i32,
    pub id_actual_status: i32,
    pub is_standard: bool,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl SlimComponent {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn id_type_access(&self) -> &i32 {
         &self.id_type_access
    }
    async fn id_component_type(&self) -> &i32 {
        &self.id_component_type
    }
    async fn id_actual_status(&self) -> &i32 {
        &self.id_actual_status
    }
    async fn is_standard(&self) -> &bool {
        &self.is_standard
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
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
            updated_at,
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
            updated_at,
        }
    }
}
