use crate::schema::*;
use crate::models::user::model::ShowUserShort;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::param::model::ComponentParamWithTranslation;
use crate::models::component::spec::model::ComponentSpecWithTranslation;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::component::component_modification::model::ComponentModificationAndRelatedData;
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::file::model::ShowFileForDownload;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "component_ref"]
pub struct Component {
    pub uuid: Uuid,
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl Component {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn parent_component_uuid(&self) -> ID {
        self.parent_component_uuid.into()
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn type_access_id(&self) -> &i32 {
        &self.type_access_id
    }
    async fn component_type_id(&self) -> &i32 {
        &self.component_type_id
    }
    async fn actual_status_id(&self) -> &i32 {
        &self.actual_status_id
    }
    async fn is_base(&self) -> &bool {
        &self.is_base
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

#[derive(Debug, SimpleObject)]
pub struct ComponentAndRelatedData {
    pub uuid: Uuid,
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub type_access_id: i32, //TypeAccess
    pub component_type: ComponentTypeTranslateList,
    pub actual_status: ActualStatusTranslateList,
    pub is_base: bool,
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub updated_at: NaiveDateTime,
    // related data
    pub licenses: Vec<License>,
    pub component_params: Vec<ComponentParamWithTranslation>,
    pub files: Vec<ShowFileForDownload>,
    pub component_specs: Vec<ComponentSpecWithTranslation>,
    pub component_keywords: Vec<Keyword>,
    pub component_modifications: Vec<ComponentModificationAndRelatedData>,
    pub component_suppliers: Vec<ComponentSupplierRelatedData>,
    // show the standards that fit the object
    pub component_standards: Vec<ShowStandardShort>,
}


#[derive(Debug, SimpleObject)]
pub struct ShowComponentShort {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub type_access_id: i32, //TypeAccess
    pub component_type: ComponentTypeTranslateList,
    pub actual_status: ActualStatusTranslateList,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub is_base: bool,
    pub updated_at: NaiveDateTime,
    pub licenses: Vec<License>,
    // files for show image (models, draw)
    pub files: Vec<ShowFileForDownload>,
    // show first supplier company
    pub component_suppliers: Vec<ComponentSupplierRelatedData>,
}

#[derive(Debug, Insertable)]
#[table_name = "component_ref"]
pub struct InsertableComponent {
    pub uuid: Uuid,
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct ComponentData {
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptComponentData {
    pub parent_component_uuid: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
}

#[derive(Debug, Serialize, Queryable)]
pub struct SlimComponent {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
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
    async fn type_access_id(&self) -> &i32 {
         &self.type_access_id
    }
    async fn component_type_id(&self) -> &i32 {
        &self.component_type_id
    }
    async fn actual_status_id(&self) -> &i32 {
        &self.actual_status_id
    }
    async fn is_base(&self) -> &bool {
        &self.is_base
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

impl From<ComponentData> for InsertableComponent {
    fn from(data_component: ComponentData) -> Self {
        let ComponentData {
            parent_component_uuid,
            name,
            description,
            user_uuid,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            ..
        } = data_component;

        Self {
            uuid: Uuid::new_v4(),
            parent_component_uuid,
            name,
            description,
            user_uuid,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
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
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            updated_at,
            ..
        } = component;

        Self {
            uuid,
            name,
            description,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            updated_at,
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptUpdateComponentData {
    pub parent_component_uuid: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub component_type_id: Option<i32>,
    pub actual_status_id: Option<i32>,
}
