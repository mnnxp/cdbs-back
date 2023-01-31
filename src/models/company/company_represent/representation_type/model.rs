use crate::schema::*;
use crate::models::company::company_represent::model::CompanyRepresent;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = representation_type_ref)]
pub(crate) struct RepresentationType {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = representation_type_ref)]
pub(crate) struct InsertableRepresentationType {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRepresentationTypeData {
    pub(crate) id: i32,
}

// RepresentationType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Default, SimpleObject, Debug)]
#[diesel(primary_key(representation_type_id, lang_id))]
#[diesel(belongs_to(CompanyRepresent, foreign_key = representation_type_id))]
#[diesel(belongs_to(Language, foreign_key = lang_id))]
#[diesel(table_name = representation_type_translate_list)]
pub(crate) struct RepresentationTypeTranslateList {
    pub(crate) representation_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) representation_type: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptRepresentationTypeTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) representation_type: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = representation_type_translate_list)]
pub(crate) struct InsertableRepresentationTypeTranslateList {
    pub(crate) representation_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) representation_type: String,
}
