use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::standard::model::Standard;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(standard_uuid, keyword_id)]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[belongs_to(Keyword, foreign_key = "keyword_id")]
#[table_name = "keyword_to_standard"]
pub struct StandardKeyword {
    pub standard_uuid: Uuid,
    pub keyword_id: i32,
}

#[Object]
impl StandardKeyword {
    async fn keyword_id(&self) -> &i32 {
        &self.keyword_id
    }
    async fn standard_uuid(&self) -> ID {
        self.standard_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct StandardKeywordRelatedData {
    pub keyword: Keyword,
    pub standard_uuid: Uuid,
}

impl From<(StandardKeyword, Keyword)> for StandardKeywordRelatedData {
    fn from(data: (StandardKeyword, Keyword)) -> Self {
        Self {
            keyword: data.1,
            standard_uuid: data.0.standard_uuid,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_standard"]
pub struct InsertableStandardKeyword {
    pub standard_uuid: Uuid,
    pub keyword_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardKeywordsData {
    pub standard_uuid: Uuid,
    pub keyword_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardKeywordsNames {
    pub standard_uuid: Uuid,
    pub keywords: Vec<String>,
}

impl From<&IptStandardKeywordsData> for Vec<InsertableStandardKeyword> {
    fn from(ipt_data: &IptStandardKeywordsData) -> Vec<InsertableStandardKeyword> {
        let IptStandardKeywordsData {
            standard_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for keyword_id in keyword_ids {
            if keyword_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableStandardKeyword {
                    standard_uuid: standard_uuid.to_owned(),
                    keyword_id: *keyword_id,
                })
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub struct DeleteStandardKeywords {
    pub standard_uuid: Uuid,
    pub keyword_ids: Vec<i32>,
}

impl From<&IptStandardKeywordsData> for DeleteStandardKeywords {
    fn from(ipt_data: &IptStandardKeywordsData) -> Self {
        let IptStandardKeywordsData {
            standard_uuid,
            keyword_ids,
            ..
        } = ipt_data;

        let mut good_keyword_ids: Vec<i32> = Vec::new();
        // filter bad keywords id
        for keyword_id in keyword_ids {
            if keyword_id > &0 {
                good_keyword_ids.push(*keyword_id)
            }
        }

        Self{
            standard_uuid: standard_uuid.to_owned(),
            keyword_ids: good_keyword_ids,
        }
    }
}
