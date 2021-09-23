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
pub struct KeywordStandard {
    pub standard_uuid: Uuid,
    pub keyword_id: i32,
}

#[Object]
impl KeywordStandard {
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

impl From<(KeywordStandard, Keyword)> for StandardKeywordRelatedData {
    fn from(data: (KeywordStandard, Keyword)) -> Self {
        Self {
            keyword: data.1,
            standard_uuid: data.0.standard_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptKeywordStandardData {
    pub standard_uuid: Uuid,
    pub keyword_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_standard"]
pub struct InsertableKeywordStandard {
    pub standard_uuid: Uuid,
    pub keyword_id: i32,
}

impl From<IptKeywordStandardData> for InsertableKeywordStandard {
    fn from(ipt_data: IptKeywordStandardData) -> Self {
        let IptKeywordStandardData {
            standard_uuid,
            keyword_id,
            ..
        } = ipt_data;

        Self {
            standard_uuid: Uuid::parse_str(&standard_uuid.to_string()).unwrap(),
            keyword_id,
        }
    }
}
