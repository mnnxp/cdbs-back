use crate::models::{
    component::component_modification::model::ComponentModification,
    relate_ref::param::model::{ParamTranslateList, IptParamData},
};
use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(PartialEq, Clone, Debug, SimpleObject)]
#[primary_key(modification_uuid)]
#[belongs_to(ComponentModification, foreign_key = "modification_uuid")]
#[belongs_to(ParamTranslateList, foreign_key = "param_id")]
#[table_name = "param_to_modification"]
pub struct ModificationParam {
    pub modification_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, SimpleObject, Clone, Default)]
pub struct ModificationParamWithTranslation {
    pub modification_uuid: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl ModificationParamWithTranslation {
    /// Set modification uuid and param value without param translate
    pub(crate) fn new(data: &ModificationParam) -> Self {
        Self {
            modification_uuid: data.modification_uuid,
            value: data.value.clone(),
            ..Default::default()
        }
    }

    /// Change modification param
    pub(crate) fn put_param_translate(&mut self, param: ParamTranslateList) {
        self.param = param;
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
