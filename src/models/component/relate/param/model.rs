use crate::models::relate_ref::param::model::{IptParamData, ParamTranslateList};
use crate::schema::*;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

/// Component parameter data with localization
#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub(crate) struct ComponentParamWithTranslation {
    /// Component UUID (part identifier)
    pub(crate) component_uuid: Uuid,
    /// Data about the parameter (name) with localization
    pub(crate) param: ParamTranslateList,
    /// Component parameter value
    pub(crate) value: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_to_component)]
pub(crate) struct InsertableComponentParam {
    pub(crate) component_uuid: Uuid,
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

/// Data for request to add/update component (part) parameters
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentParamsData {
    /// Component UUID (part identifier)
    pub(crate) component_uuid: Uuid,
    /// List of parameters with values
    pub(crate) params: Vec<IptParamData>,
}

impl From<IptComponentParamsData> for Vec<InsertableComponentParam> {
    fn from(ipt_data: IptComponentParamsData) -> Vec<InsertableComponentParam> {
        let IptComponentParamsData {
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

/// Data for a request to delete component parameters
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelComponentParamData {
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Component parameter identifiers (list)
    pub(crate) param_ids: Vec<i32>,
}
