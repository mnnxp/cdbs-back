use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::language::model::Language;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Associations, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "component_type_ref"]
pub(crate) struct ComponentType {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "component_type_ref"]
pub(crate) struct InsertableComponentType {
    pub(crate) id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentTypeData {
    pub(crate) id: i32,
}

// ComponentType translations
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Clone, Debug)]
#[primary_key(component_type_id, lang_id)]
#[belongs_to(Component, foreign_key = "component_type_id")]
#[belongs_to(ComponentType, foreign_key = "component_type_id")]
#[belongs_to(Language, foreign_key = "lang_id")]
#[table_name = "component_type_translate_list"]
pub(crate) struct ComponentTypeTranslateList {
    pub(crate) component_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) component_type: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentTypeTranslateListData {
    pub(crate) component_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) component_type: String,
}

#[derive(Debug, Insertable)]
#[table_name = "component_type_translate_list"]
pub(crate) struct InsertableComponentTypeTranslateList {
    pub(crate) component_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) component_type: String,
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
