use crate::models::component::relate::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_modification::param::model::ParamModificationRelate;
use crate::models::component::model::Component;
use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
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

#[derive(Identifiable, Deserialize, Queryable, Associations, SimpleObject, Description, Debug)]
#[primary_key(uuid)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[table_name = "component_modification_list"]
pub struct ComponentModificationAndRelatedData {
    pub uuid: Uuid,
    pub uuid_component: Uuid,
    pub uuid_modification_parent: Uuid,
    pub modification_name: String,
    pub description: String,
    pub actual_status: ActualStatusTranslateList,
    // pub id_actual_status: i32,
    pub updated_at: NaiveDateTime,
    pub param_modification: Vec<ParamModificationRelate>,
}

impl From<(ComponentModificationWithActualStatus, Vec<ParamModificationRelate>)> for ComponentModificationAndRelatedData {
    fn from(data: (ComponentModificationWithActualStatus, Vec<ParamModificationRelate>)) -> Self {
        Self {
            uuid: data.0.modification.uuid,
            uuid_component: data.0.modification.uuid_component,
            uuid_modification_parent: data.0.modification.uuid_modification_parent,
            modification_name: data.0.modification.modification_name,
            description: data.0.modification.description,
            actual_status: data.0.actual_status,
            // id_actual_status: data.0.modification.id_actual_status,
            updated_at: data.0.modification.updated_at,
            param_modification: data.1,
        }
    }
}

// #[derive(Deserialize, Debug)]
// pub struct ComponentModificationWithParam {
//     pub info: ComponentModificationWithActualStatus,
//     pub params: Vec<ParamModificationRelate>,
// }
//
//
// impl From<(ComponentModificationWithActualStatus, Vec<ParamModificationRelate>)> for ComponentModificationWithParam {
//     fn from(data: (ComponentModificationWithActualStatus, Vec<ParamModificationRelate>)) -> Self {
//         Self {
//             info: data.0,
//             params: data.1,
//         }
//     }
// }


#[derive(Deserialize, Clone, Debug)]
pub struct ComponentModificationWithActualStatus {
    pub modification: ComponentModification,
    pub actual_status: ActualStatusTranslateList,
    // pub params: Vec<ParamModificationRelate>,
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

impl From<IptComponentModificationData> for InsertableComponentModification {
    fn from(ipt_data: IptComponentModificationData) -> Self {
        let IptComponentModificationData {
            uuid_component,
            uuid_modification_parent,
            modification_name,
            description,
            id_actual_status,
        } = ipt_data;

        Self {
            uuid: Uuid::new_v4(),
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            uuid_modification_parent: Uuid::parse_str(&uuid_modification_parent.to_string())
                .unwrap(),
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
