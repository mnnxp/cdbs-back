use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "component_type_ref"]
pub struct ComponentType {
    pub id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "component_type_ref"]
pub struct InsertableComponentType {
    pub id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentTypeData {
    pub id: i32,
}

// ComponentType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[primary_key(component_type_id, lang_id)]
#[belongs_to(Component, foreign_key = "component_type_id")]
#[belongs_to(ComponentType, foreign_key = "component_type_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "component_type_translate_list"]
pub struct ComponentTypeTranslateList {
    pub component_type_id: i32,
    pub lang_id: i32,
    pub component_type: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentTypeTranslateListData {
    pub component_type_id: i32,
    pub lang_id: i32,
    pub component_type: String,
}

#[derive(Debug, Insertable)]
#[table_name = "component_type_translate_list"]
pub struct InsertableComponentTypeTranslateList {
    pub component_type_id: i32,
    pub lang_id: i32,
    pub component_type: String,
}

impl From<IptComponentTypeTranslateListData> for InsertableComponentTypeTranslateList {
    fn from(ipt_data: IptComponentTypeTranslateListData) -> Self {
        let IptComponentTypeTranslateListData {
            component_type_id,
            lang_id,
            component_type,
            ..
        } = ipt_data;

        Self {
            component_type_id,
            lang_id,
            component_type,
        }
    }
}
