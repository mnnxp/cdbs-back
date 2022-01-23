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
pub(crate) struct TypeAccess {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "type_access_ref"]
pub(crate) struct InsertableTypeAccess {
    pub(crate) id: i32,
}

// TypeAccess translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Default, Debug)]
#[primary_key(type_access_id, lang_id)]
#[belongs_to(TypeAccess, foreign_key = "type_access_id")]
#[belongs_to(Component, foreign_key = "type_access_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "type_access_translate_list"]
pub(crate) struct TypeAccessTranslateList {
    pub(crate) type_access_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptTypeAccessTranslateListData {
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Insertable)]
#[table_name = "type_access_translate_list"]
pub(crate) struct InsertableTypeAccessTranslateList {
    pub(crate) type_access_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) name: String,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptTypeAccessArg {
    pub(crate) type_access_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct TypeAccessArg {
    pub(crate) type_access_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for TypeAccessArg {
    fn default() -> Self {
        Self {
            type_access_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptTypeAccessArg> for TypeAccessArg {
    fn from(data: IptTypeAccessArg) -> Self {
        let IptTypeAccessArg {
            type_access_ids,
            limit,
            offset,
        } = data;

        Self {
            type_access_ids: type_access_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
