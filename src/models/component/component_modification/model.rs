use crate::models::component::relate::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_modification::param::model::ModificationParamWithTranslation;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgramRelatedData;
use crate::models::component::model::Component;
use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(uuid)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[table_name = "component_modification_list"]
pub struct ComponentModification {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ComponentModification {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
    async fn parent_modification_uuid(&self) -> ID {
        self.parent_modification_uuid.into()
    }
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn actual_status_id(&self) -> &i32 {
        &self.actual_status_id
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

#[derive(Deserialize, SimpleObject, Debug)]
pub struct ComponentModificationAndRelatedData {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status: ActualStatusTranslateList,
    // pub actual_status_id: i32,
    pub updated_at: NaiveDateTime,
    pub filesets_for_program: Vec<FilesetProgramRelatedData>,
    pub modification_params: Vec<ModificationParamWithTranslation>,
}

impl From<(
    ComponentModificationWithActualStatus,
    Vec<FilesetProgramRelatedData>,
    Vec<ModificationParamWithTranslation>
)> for ComponentModificationAndRelatedData {
    fn from(data: (
        ComponentModificationWithActualStatus,
        Vec<FilesetProgramRelatedData>,
        Vec<ModificationParamWithTranslation>
    )) -> Self {
        Self {
            uuid: data.0.modification.uuid,
            component_uuid: data.0.modification.component_uuid,
            parent_modification_uuid: data.0.modification.parent_modification_uuid,
            modification_name: data.0.modification.modification_name,
            description: data.0.modification.description,
            actual_status: data.0.actual_status,
            // actual_status_id: data.0.modification.actual_status_id,
            updated_at: data.0.modification.updated_at,
            filesets_for_program: data.1,
            modification_params: data.2,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentModificationWithActualStatus {
    pub modification: ComponentModification,
    pub actual_status: ActualStatusTranslateList,
}

impl From<(ComponentModification, ActualStatusTranslateList)> for ComponentModificationWithActualStatus {
    fn from(data: (ComponentModification, ActualStatusTranslateList)) -> Self {
        Self {
            modification: data.0,
            actual_status: data.1,
        }
    }
}


#[derive(Debug, Insertable)]
#[table_name = "component_modification_list"]
pub struct InsertableComponentModification {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentModificationData {
    pub component_uuid: Uuid,
    pub parent_modification_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimComponentModification {
    pub uuid: Uuid,
    pub component_uuid: Uuid,
    pub modification_name: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl SlimComponentModification {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
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

impl From<IptComponentModificationData> for InsertableComponentModification {
    fn from(ipt_data: IptComponentModificationData) -> Self {
        let IptComponentModificationData {
            component_uuid,
            parent_modification_uuid,
            modification_name,
            description,
            actual_status_id,
        } = ipt_data;

        Self {
            uuid: Uuid::new_v4(),
            component_uuid: Uuid::parse_str(&component_uuid.to_string()).unwrap(),
            parent_modification_uuid: Uuid::parse_str(&parent_modification_uuid.to_string())
                .unwrap(),
            modification_name,
            description,
            actual_status_id,
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
            component_uuid,
            modification_name,
            description,
            updated_at,
            ..
        } = data_modification;

        Self {
            uuid,
            component_uuid,
            modification_name,
            description,
            updated_at,
        }
    }
}
