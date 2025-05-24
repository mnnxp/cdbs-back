use crate::schema::*;
use crate::models::relate_ref::param::model::{ParamTranslateList, IptParamData};
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Param service models
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = param_to_service)]
pub(crate) struct ServiceParam {
    pub(crate) service_uuid: Uuid,
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

/// Service parameter data with localization
#[derive(Debug, Deserialize, SimpleObject, Clone)]
pub(crate) struct ServiceParamWithTranslation {
    /// Service UUID (part identifier)
    pub(crate) service_uuid: Uuid,
    /// Data about the parameter (name) with localization
    pub(crate) param: ParamTranslateList,
    /// Service parameter value
    pub(crate) value: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = param_to_service)]
pub(crate) struct InsertableServiceParam {
    pub(crate) service_uuid: Uuid,
    pub(crate) param_id: i32,
    pub(crate) value: String,
}

/// Data for request to add/update service (part) parameters
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptServiceParamsData {
    /// Service UUID (part identifier)
    pub(crate) service_uuid: Uuid,
    /// List of parameters with values
    pub(crate) params: Vec<IptParamData>,
}

impl From<IptServiceParamsData> for Vec<InsertableServiceParam> {
    fn from(ipt_data: IptServiceParamsData) -> Vec<InsertableServiceParam> {
        let IptServiceParamsData {
            service_uuid,
            params,
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each param
        for param_d in params {
            // now off checking, check the before
            res.push(InsertableServiceParam {
                service_uuid,
                param_id: param_d.param_id,
                value: param_d.value,
            })
        }
        res
    }
}

/// Data for a request to delete service parameters
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelServiceParamData {
    /// Service UUID
    pub(crate) service_uuid: Uuid,
    /// Service parameter identifiers (list)
    pub(crate) param_ids: Vec<i32>,
}
