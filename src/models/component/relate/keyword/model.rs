use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::model::Component;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(component_uuid, keyword_id))]
#[diesel(belongs_to(Component, foreign_key = component_uuid))]
#[diesel(belongs_to(Keyword, foreign_key = keyword_id))]
#[diesel(table_name = keyword_to_component)]
pub(crate) struct ComponentKeyword {
    pub(crate) component_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

/// Data for requests to add and remove keyword relationships to the component
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentKeywordsData {
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Keyword identifiers (list)
    pub(crate) keyword_ids: Vec<i32>,
}

/// Data for a request to add a keyword relationship to a component
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentKeywordsNames {
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Keyword identifiers (list)
    pub(crate) keywords: Vec<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keyword_to_component)]
pub(crate) struct InsertableComponentKeyword {
    pub(crate) component_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

impl From<&IptComponentKeywordsData> for Vec<InsertableComponentKeyword> {
    fn from(ipt_data: &IptComponentKeywordsData) -> Vec<InsertableComponentKeyword> {
        let IptComponentKeywordsData {
            component_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for kw_id in keyword_ids {
            if kw_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableComponentKeyword {
                    component_uuid: *component_uuid,
                    keyword_id: *kw_id,
                })
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DeleteComponentKeyword {
    pub(crate) component_uuid: Uuid,
    pub(crate) keyword_ids: Vec<i32>,
}

impl From<&IptComponentKeywordsData> for DeleteComponentKeyword {
    fn from(ipt_data: &IptComponentKeywordsData) -> Self {
        let IptComponentKeywordsData {
            component_uuid,
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
            component_uuid: *component_uuid,
            keyword_ids: good_kw_ids,
        }
    }
}

/// Data for querying keywords associated with the component
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentKeywordsArg {
    /// component UUID
    pub(crate) component_uuid: Uuid,
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentKeywordsArg {
    pub(crate) component_uuid: Uuid,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl From<IptComponentKeywordsArg> for ComponentKeywordsArg {
    fn from(data: IptComponentKeywordsArg) -> Self {
        let IptComponentKeywordsArg {
            component_uuid,
            limit,
            offset,
        } = data;

        Self {
            component_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
