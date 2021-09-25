use crate::models::component::component_modification::model::ComponentModification;
use crate::models::relate_ref::param::model::{
    ParamTranslateList, IptParamData
};
use crate::schema::*;

use async_graphql::types::ID;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, PartialEq, Clone, Debug)]
#[primary_key(modification_uuid)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(ParamTranslateList, foreign_key = "param_id")]
#[table_name = "param_to_modification"]
pub struct ModificationParam {
    pub modification_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[Object]
impl ModificationParam {
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

impl From<(ModificationParam, ParamTranslateList)> for ModificationParamWithTranslation {
    fn from(data: (ModificationParam, ParamTranslateList)) -> Self {
        Self {
            modification_uuid: data.0.modification_uuid,
            param: data.1,
            value: data.0.value,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_modification"]
pub struct InsertableModificationParam {
    pub modification_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptModificationParamData {
    pub modification_uuid: Uuid,
    pub params: Vec<IptParamData>,
}

impl From<IptModificationParamData> for Vec<InsertableModificationParam> {
    fn from(ipt_data: IptModificationParamData) -> Vec<InsertableModificationParam> {
        let IptModificationParamData {
            modification_uuid,
            params,
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each param
        for param_d in params {
            // now off checking, check the before
            res.push(InsertableModificationParam {
                modification_uuid,
                param_id: param_d.param_id,
                value: param_d.value,
            })
        }
        res
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelModificationParamData {
    pub modification_uuid: Uuid,
    pub param_ids: Vec<i32>,
}
