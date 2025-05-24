use crate::schema::*;
use async_graphql::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = component_type_ref)]
pub(crate) struct ComponentType {
    pub(crate) id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_type_ref)]
pub(crate) struct InsertableComponentType {
    pub(crate) id: i32,
}

/// Component type data with localization (translation) for the specified language
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = component_type_translate_list)]
pub(crate) struct ComponentTypeTranslateList {
    /// Component type identifier
    pub(crate) component_type_id: i32,
    /// Localization language identifier
    pub(crate) lang_id: i32,
    /// Component type name
    pub(crate) component_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct IptComponentTypeTranslateListData {
    pub(crate) component_type_id: i32,
    pub(crate) lang_id: i32,
    pub(crate) component_type: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_type_translate_list)]
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
