use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::model::Component;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[primary_key(component_uuid, keyword_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Keyword, foreign_key = "keyword_id")]
#[table_name = "keyword_to_component"]
pub(crate) struct ComponentKeyword {
    pub(crate) component_uuid: Uuid,
    pub(crate) keyword_id: i32,
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub(crate) struct ComponentKeywordRelatedData {
    pub(crate) keyword: Keyword,
    pub(crate) component_uuid: Uuid,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentKeywordsData {
    pub(crate) component_uuid: Uuid,
    pub(crate) keyword_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentKeywordsNames {
    pub(crate) component_uuid: Uuid,
    pub(crate) keywords: Vec<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_component"]
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

        // let component_uuid = Uuid::parse_str(&component_uuid.to_string()).unwrap();

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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentKeywordsArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentKeywordsArg {
    pub(crate) component_uuid:  Uuid,
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
