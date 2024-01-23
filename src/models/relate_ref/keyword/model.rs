use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
#[derive(SimpleObject, Clone, Default, Debug)]
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

/// Обертка для ключевого слова используемая для добавления новых ключевых слов
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptKeywordData {
    /// Ключевое слово/тег для упрощения поиска по фото и характеристикам
    pub(crate) keyword: String,
}

impl From<&IptKeywordData> for InsertableKeyword {
    fn from(data: &IptKeywordData) -> Self {
        Self {
            keyword: data.keyword.clone()
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptKeywordArg {
    pub(crate) keyword_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct KeywordArg {
    pub(crate) keyword_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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
