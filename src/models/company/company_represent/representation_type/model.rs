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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Default, Debug)]
#[primary_key(id_representation_type, id_lang)]
#[belongs_to(CompanyRepresent, foreign_key = "id_representation_type")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "representation_type_translate_list"]
pub struct RepresentationTypeTranslateList {
    pub id_representation_type: i32,
    pub id_lang: i32,
    pub representation_type: String,
}

#[Object]
impl RepresentationTypeTranslateList {
    async fn id_representation_type(&self) -> &i32 {
        &self.id_representation_type
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn representation_type(&self) -> &String {
        &self.representation_type
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptRepresentationTypeTranslateListData {
    pub id_lang: i32,
    pub representation_type: String,
}

#[derive(Debug, Insertable)]
#[table_name = "representation_type_translate_list"]
pub struct InsertableRepresentationTypeTranslateList {
    pub id_representation_type: i32,
    pub id_lang: i32,
    pub representation_type: String,
}
