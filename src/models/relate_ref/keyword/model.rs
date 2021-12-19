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
