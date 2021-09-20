use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, keyword_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Keyword, foreign_key = "keyword_id")]
#[table_name = "keyword_to_component"]
pub struct ComponentKeyword {
    pub component_uuid: Uuid,
    pub keyword_id: i32,
}

#[Object]
impl ComponentKeyword {
    async fn keyword_id(&self) -> &i32 {
        &self.keyword_id
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentKeywordRelatedData {
    pub keyword: Keyword,
    pub component_uuid: Uuid,
}

impl From<(ComponentKeyword, Keyword)> for ComponentKeywordRelatedData {
    fn from(data: (ComponentKeyword, Keyword)) -> Self {
        Self {
            keyword: data.1,
            component_uuid: data.0.component_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentKeywordData {
    pub component_uuid: Uuid,
    pub keyword_ids: Vec<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_component"]
pub struct InsertableComponentKeyword {
    pub component_uuid: Uuid,
    pub keyword_id: i32,
}

impl From<IptComponentKeywordData> for Vec<InsertableComponentKeyword> {
    fn from(ipt_data: IptComponentKeywordData) -> Vec<InsertableComponentKeyword> {
        let IptComponentKeywordData {
            component_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        // let component_uuid = Uuid::parse_str(&component_uuid.to_string()).unwrap();

        let mut res = Vec::new();
        // create struct for each keyword
        for kw_id in keyword_ids.iter() {
            if kw_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableComponentKeyword {
                    component_uuid,
                    keyword_id: *kw_id,
                })
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct DeleteComponentKeyword {
    pub component_uuid: Uuid,
    pub keyword_ids: Vec<i32>,
}

impl From<IptComponentKeywordData> for DeleteComponentKeyword {
    fn from(ipt_data: IptComponentKeywordData) -> Self {
        let IptComponentKeywordData {
            component_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut good_kw_ids: Vec<i32> = Vec::new();
        // filter bad keyword id
        for kw_id in keyword_ids.iter() {
            if kw_id > &0 {
                good_kw_ids.push(*kw_id)
            }
        }

        Self{
            component_uuid,
            keyword_ids: good_kw_ids,
        }
    }
}
