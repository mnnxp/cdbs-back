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
pub struct KeywordComponent {
    pub component_uuid: Uuid,
    pub keyword_id: i32,
}

#[Object]
impl KeywordComponent {
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

impl From<(KeywordComponent, Keyword)> for ComponentKeywordRelatedData {
    fn from(data: (KeywordComponent, Keyword)) -> Self {
        Self {
            keyword: data.1,
            component_uuid: data.0.component_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptKeywordComponentData {
    pub component_uuid: ID,
    pub keyword_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_component"]
pub struct InsertableKeywordComponent {
    pub component_uuid: Uuid,
    pub keyword_id: i32,
}

impl From<IptKeywordComponentData> for InsertableKeywordComponent {
    fn from(ipt_data: IptKeywordComponentData) -> Self {
        let IptKeywordComponentData {
            component_uuid,
            keyword_id,
            ..
        } = ipt_data;

        Self {
            component_uuid: Uuid::parse_str(&component_uuid.to_string()).unwrap(),
            keyword_id,
        }
    }
}
