use crate::schema::*;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Keyword component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_component, id_keyword)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(Keyword, foreign_key = "id_keyword")]
#[table_name = "keyword_to_component"]
pub struct KeywordComponent {
    pub uuid_component: Uuid,
    pub id_keyword: i32,
}

#[Object]
impl KeywordComponent {
    async fn id_keyword(&self) -> &i32 {
        &self.id_keyword
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
}

#[derive(Deserialize, SimpleObject, Description, Clone, Debug)]
pub struct ComponentKeywordRelatedData {
    pub keyword: Keyword,
    pub uuid_component: Uuid,
}

impl From<(KeywordComponent, Keyword)> for ComponentKeywordRelatedData {
    fn from(data: (KeywordComponent, Keyword)) -> Self {
        Self {
            keyword: data.1,
            uuid_component: data.0.uuid_component,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptKeywordComponentData {
    pub uuid_component: ID,
    pub id_keyword: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "keyword_to_component"]
pub struct InsertableKeywordComponent {
    pub uuid_component: Uuid,
    pub id_keyword: i32,
}

impl From<IptKeywordComponentData> for InsertableKeywordComponent {
    fn from(ipt_data: IptKeywordComponentData) -> Self {
        let IptKeywordComponentData {
            uuid_component,
            id_keyword,
            ..
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            id_keyword,
        }
    }
}
