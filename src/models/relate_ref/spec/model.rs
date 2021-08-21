use crate::schema::*;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "spec_ref"]
pub struct Spec {
    pub id: i32,
    pub id_spec_parent: i32,
}

#[Object]
impl Spec {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn id_spec_parent(&self) -> &i32 {
        &self.id_spec_parent
    }
}

#[derive(Debug, Insertable)]
#[table_name = "spec_ref"]
pub struct InsertableSpec {
    pub id_spec_parent: i32,
}

// Spec translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(id_spec, id_lang)]
#[belongs_to(Spec, foreign_key = "id_spec")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "spec_translate_list"]
pub struct SpecTranslateList {
    pub id_spec: i32,
    pub id_lang: i32,
    pub spec: String,
}

#[Object]
impl SpecTranslateList {
    async fn id_spec(&self) -> &i32 {
        &self.id_spec
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn spec(&self) -> &String {
        &self.spec
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecTranslateListData {
    pub id_spec: i32,
    pub id_spec_parent: i32,
    pub id_lang: i32,
    pub spec: String,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_translate_list"]
pub struct InsertableSpecTranslateList {
    pub id_spec: i32,
    pub id_lang: i32,
    pub spec: String,
}

impl From<IptSpecTranslateListData> for InsertableSpec {
    fn from(ipt_data: IptSpecTranslateListData) -> Self {
        let IptSpecTranslateListData {
            id_spec_parent,
            ..
        } = ipt_data;

        Self {
            id_spec_parent
        }
    }
}
