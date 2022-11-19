use crate::models::{
    component::component_modification::model::ComponentModification,
    relate_ref::param::model::{ParamTranslateList, IptParamData},
};
use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(PartialEq, Clone, Debug, SimpleObject)]
#[diesel(primary_key(modification_uuid))]
#[diesel(belongs_to(ComponentModification, foreign_key = modification_uuid))]
#[diesel(belongs_to(ParamTranslateList, foreign_key = param_id))]
#[diesel(table_name = param_to_modification)]
pub(crate) struct ModificationParam {
    pub(crate) modification_uuid: Uuid,
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

#[derive(Debug, Deserialize, SimpleObject, Clone, Default)]
pub(crate) struct ModificationParamWithTranslation {
    pub(crate) modification_uuid: Uuid,
    pub(crate) param: ParamTranslateList,
    pub(crate) value: String,
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
#[diesel(table_name = param_to_modification)]
pub(crate) struct InsertableModificationParam {
    pub(crate) modification_uuid: Uuid,
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptModificationParamData {
    pub(crate) modification_uuid: Uuid,
    pub(crate) params: Vec<IptParamData>,
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
pub(crate) struct DelModificationParamData {
    pub(crate) modification_uuid: Uuid,
    pub(crate) param_ids: Vec<i32>,
}
