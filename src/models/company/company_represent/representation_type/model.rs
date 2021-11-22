use crate::schema::*;
use crate::models::company::company_represent::model::CompanyRepresent;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "representation_type_ref"]
pub struct RepresentationType {
    pub id: i32,
}

#[Object]
impl RepresentationType {
    async fn id(&self) -> &i32 {
        &self.id
    }
}

#[derive(Debug, Insertable)]
#[table_name = "representation_type_ref"]
pub struct InsertableRepresentationType {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRepresentationTypeData {
    pub id: i32,
}

// RepresentationType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Default, SimpleObject, Debug)]
#[primary_key(representation_type_id, lang_id)]
#[belongs_to(CompanyRepresent, foreign_key = "representation_type_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "representation_type_translate_list"]
pub struct RepresentationTypeTranslateList {
    pub representation_type_id: i32,
    pub lang_id: i32,
    pub representation_type: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRepresentationTypeTranslateListData {
    pub lang_id: i32,
    pub representation_type: String,
}

#[derive(Debug, Insertable)]
#[table_name = "representation_type_translate_list"]
pub struct InsertableRepresentationTypeTranslateList {
    pub representation_type_id: i32,
    pub lang_id: i32,
    pub representation_type: String,
}
