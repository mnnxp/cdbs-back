use crate::schema::*;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::param::model::ParamTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(modification_uuid)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(ParamTranslateList, foreign_key = "param_id")]
#[table_name = "param_to_modification"]
pub struct ParamModification {
    pub modification_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[Object]
impl ParamModification {
    async fn modification_uuid(&self) -> ID {
        self.modification_uuid.into()
    }
    async fn param_id(&self) -> &i32 {
        &self.param_id
    }
    async fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct ModificationParamWithTranslation {
    pub modification_uuid: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl From<(ParamModification, ParamTranslateList)> for ModificationParamWithTranslation {
    fn from(data: (ParamModification, ParamTranslateList)) -> Self {
        Self {
            modification_uuid: data.0.modification_uuid,
            param: data.1,
            value: data.0.value,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptParamModificationData {
    pub modification_uuid: ID,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_modification"]
pub struct InsertableParamModification {
    pub modification_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

impl From<IptParamModificationData> for InsertableParamModification {
    fn from(ipt_data: IptParamModificationData) -> Self {
        let IptParamModificationData {
            modification_uuid,
            param_id,
            value,
        } = ipt_data;

        Self {
            modification_uuid: Uuid::parse_str(&modification_uuid.to_string()).unwrap(),
            param_id,
            value,
        }
    }
}
