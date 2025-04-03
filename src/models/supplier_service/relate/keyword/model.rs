use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::supplier_service::model::Service;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword service models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(service_uuid, keyword_id))]
#[diesel(belongs_to(Service, foreign_key = service_uuid))]
#[diesel(belongs_to(Keyword, foreign_key = keyword_id))]
#[diesel(table_name = keyword_to_service)]
pub(crate) struct ServiceKeyword {
    pub(crate) service_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

/// Data for requests to add and remove keyword relationships to the service
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptServiceKeywordsData {
    /// Service UUID
    pub(crate) service_uuid: Uuid,
    /// Keyword identifiers (list)
    pub(crate) keyword_ids: Vec<i32>,
}

/// Data for a request to add a keyword relationship to a service
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptServiceKeywordsNames {
    /// Service UUID
    pub(crate) service_uuid: Uuid,
    /// Keyword identifiers (list)
    pub(crate) keywords: Vec<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keyword_to_service)]
pub(crate) struct InsertableServiceKeyword {
    pub(crate) service_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

impl From<&IptServiceKeywordsData> for Vec<InsertableServiceKeyword> {
    fn from(ipt_data: &IptServiceKeywordsData) -> Vec<InsertableServiceKeyword> {
        let IptServiceKeywordsData {
            service_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for kw_id in keyword_ids {
            if kw_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableServiceKeyword {
                    service_uuid: *service_uuid,
                    keyword_id: *kw_id,
                })
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DeleteServiceKeyword {
    pub(crate) service_uuid: Uuid,
    pub(crate) keyword_ids: Vec<i32>,
}

impl From<&IptServiceKeywordsData> for DeleteServiceKeyword {
    fn from(ipt_data: &IptServiceKeywordsData) -> Self {
        let IptServiceKeywordsData {
            service_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut good_kw_ids: Vec<i32> = Vec::new();
        // filter bad keyword id
        for kw_id in keyword_ids {
            if kw_id > &0 {
                good_kw_ids.push(*kw_id)
            }
        }

        Self{
            service_uuid: *service_uuid,
            keyword_ids: good_kw_ids,
        }
    }
}