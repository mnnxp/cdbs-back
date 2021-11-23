use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "spec_ref"]
pub struct Spec {
    pub id: i32,
    pub parent_spec_id: i32,
}

#[Object]
impl Spec {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn parent_spec_id(&self) -> &i32 {
        &self.parent_spec_id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "spec_ref"]
pub struct InsertableSpec {
    pub parent_spec_id: i32,
}

// Spec translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(spec_id, lang_id)]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "spec_translate_list"]
pub struct SpecTranslateList {
    pub spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

#[Object]
impl SpecTranslateList {
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }
    async fn lang_id(&self) -> &i32 {
        &self.lang_id
    }
    async fn spec(&self) -> &String {
        &self.spec
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecTranslateListData {
    pub spec_id: i32,
    pub parent_spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_translate_list"]
pub struct InsertableSpecTranslateList {
    pub spec_id: i32,
    pub lang_id: i32,
    pub spec: String,
}

impl From<IptSpecTranslateListData> for InsertableSpec {
    fn from(ipt_data: IptSpecTranslateListData) -> Self {
        let IptSpecTranslateListData {
            parent_spec_id,
            ..
        } = ipt_data;

        Self {
            parent_spec_id
        }
    }
}

#[derive(Serialize, SimpleObject, Debug)]
pub struct SpecPath {
    pub spec_id: i32,
    pub path: String,
}
