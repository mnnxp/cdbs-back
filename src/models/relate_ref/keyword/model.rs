use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Clone, Default, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = keyword_ref)]
pub(crate) struct Keyword {
    pub(crate) id: i32,
    pub(crate) keyword: String,
}

#[derive(Serialize, Deserialize, Queryable, QueryableByName, Clone, Debug)]
#[diesel(table_name = keyword_ref)]
pub(crate) struct KeywordId {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = keyword_ref)]
pub(crate) struct InsertableKeyword {
    pub(crate) keyword: String,
}

/// Keyword wrapper used for request to add new keywords
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptKeywordData {
    /// Keyword/tag to simplify search by photo and features
    pub(crate) keyword: String,
}

impl From<&IptKeywordData> for InsertableKeyword {
    fn from(data: &IptKeywordData) -> Self {
        Self {
            keyword: data.keyword.clone(),
        }
    }
}
