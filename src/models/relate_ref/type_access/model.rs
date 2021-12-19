use crate::schema::*;
// use crate::models::company::model::Company;
use crate::models::component::model::Component;
// use crate::models::standard::model::Standard;
// use crate::models::user::model::UserQuery;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

// TypeAccess models
#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "type_access_ref"]
pub struct TypeAccess {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "type_access_ref"]
pub struct InsertableTypeAccess {
    pub id: i32,
}

// TypeAccess translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(type_access_id, lang_id)]
#[belongs_to(TypeAccess, foreign_key = "type_access_id")]
#[belongs_to(Component, foreign_key = "type_access_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "type_access_translate_list"]
pub struct TypeAccessTranslateList {
    pub type_access_id: i32,
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptTypeAccessTranslateListData {
    pub lang_id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "type_access_translate_list"]
pub struct InsertableTypeAccessTranslateList {
    pub type_access_id: i32,
    pub lang_id: i32,
    pub name: String,
}
