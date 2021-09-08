use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::standard::model::Standard;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_standard, id_keyword)]
#[belongs_to(Standard, foreign_key = "uuid_standard")]
#[belongs_to(Keyword, foreign_key = "id_keyword")]
#[table_name = "keyword_to_standard"]
pub struct KeywordStandard {
    pub uuid_standard: Uuid,
    pub id_keyword: i32,
}

#[Object]
impl KeywordStandard {
    async fn id_keyword(&self) -> &i32 {
        &self.id_keyword
    }
    async fn uuid_standard(&self) -> ID {
        self.uuid_standard.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct StandardKeywordRelatedData {
    pub keyword: Keyword,
    pub uuid_standard: Uuid,
}

impl From<(KeywordStandard, Keyword)> for StandardKeywordRelatedData {
    fn from(data: (KeywordStandard, Keyword)) -> Self {
        Self {
            keyword: data.1,
            uuid_standard: data.0.uuid_standard,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptKeywordStandardData {
    pub uuid_standard: ID,
    pub id_keyword: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_standard"]
pub struct InsertableKeywordStandard {
    pub uuid_standard: Uuid,
    pub id_keyword: i32,
}

impl From<IptKeywordStandardData> for InsertableKeywordStandard {
    fn from(ipt_data: IptKeywordStandardData) -> Self {
        let IptKeywordStandardData {
            uuid_standard,
            id_keyword,
            ..
        } = ipt_data;

        Self {
            uuid_standard: Uuid::parse_str(&uuid_standard.to_string()).unwrap(),
            id_keyword,
        }
    }
}
