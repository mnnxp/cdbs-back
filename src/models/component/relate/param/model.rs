use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::param::model::{
    ParamTranslateList, IptParamData
};
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(component_uuid, param_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(ParamTranslateList, foreign_key = "param_id")]
#[table_name = "param_to_component"]
pub struct ComponentParam {
    pub component_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub struct ComponentParamWithTranslation {
    pub component_uuid: Uuid,
    pub param: ParamTranslateList,
    pub value: String,
}

impl ComponentParamWithTranslation {
    /// Set component uuid and param value without param translate
    pub(crate) fn new(data: &ComponentParam) -> Self {
        Self {
            component_uuid: data.component_uuid,
            param: Default::default(),
            value: data.value.clone(),
        }
    }

    /// Change component param
    pub(crate) fn put_param_translate(&mut self, param: ParamTranslateList) {
        self.param = param;
    }
}

#[derive(Debug, Insertable)]
#[table_name = "param_to_component"]
pub struct InsertableComponentParam {
    pub component_uuid: Uuid,
    pub param_id: i32,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentParamData {
    pub component_uuid: Uuid,
    pub params: Vec<IptParamData>,
}

impl From<IptComponentParamData> for Vec<InsertableComponentParam> {
    fn from(ipt_data: IptComponentParamData) -> Vec<InsertableComponentParam> {
        let IptComponentParamData {
            component_uuid,
            params,
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each param
        for param_d in params {
            // now off checking, check the before
            // if param_d.param_id > 0 { // <-- additionally we check the correctness of the id
                res.push(InsertableComponentParam {
                    component_uuid,
                    param_id: param_d.param_id,
                    value: param_d.value,
                })
            // }
        }
        res
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelComponentParamData {
    pub component_uuid: Uuid,
    pub param_ids: Vec<i32>,
}
