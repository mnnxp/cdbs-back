use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(id)]
#[table_name = "keyword_ref"]
pub struct Keyword {
    pub id: i32,
    pub keyword: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Clone, Debug)]
#[table_name = "keyword_ref"]
pub struct KeywordId {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_ref"]
pub struct InsertableKeyword {
    pub keyword: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptKeywordData {
    pub keyword: String,
}


impl From<&IptKeywordData> for InsertableKeyword {
    fn from(data: &IptKeywordData) -> Self {
        Self {
            keyword: data.keyword.clone()
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptKeywordArg {
    pub keyword_ids:  Option<Vec<i32>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct KeywordArg {
    pub keyword_ids: Vec<i32>,
    pub limit: i32,
    pub offset: i32,
}

impl Default for KeywordArg {
    fn default() -> Self {
        Self {
            keyword_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptKeywordArg> for KeywordArg {
    fn from(data: IptKeywordArg) -> Self {
        let IptKeywordArg {
            keyword_ids,
            limit,
            offset,
        } = data;

        Self {
            keyword_ids: keyword_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
