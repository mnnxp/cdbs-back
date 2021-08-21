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

#[Object]
impl ComponentType {
    async fn id(&self) -> &i32 {
        &self.id
    }
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
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(id_component_type, id_lang)]
#[belongs_to(Component, foreign_key = "id_component_type")]
#[belongs_to(ComponentType, foreign_key = "id_component_type")]
#[belongs_to(Language, foreign_key = "id_lang")]
#[table_name = "component_type_translate_list"]
pub struct ComponentTypeTranslateList {
    pub id_component_type: i32,
    pub id_lang: i32,
    pub component_type: String,
}

#[Object]
impl ComponentTypeTranslateList {
    async fn id_component_type(&self) -> &i32 {
        &self.id_component_type
    }
    async fn id_lang(&self) -> &i32 {
        &self.id_lang
    }
    async fn component_type(&self) -> &String {
        &self.component_type
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentTypeTranslateListData {
    pub id_component_type: i32,
    pub id_lang: i32,
    pub component_type: String,
}

#[derive(Debug, Insertable)]
#[table_name = "component_type_translate_list"]
pub struct InsertableComponentTypeTranslateList {
    pub id_component_type: i32,
    pub id_lang: i32,
    pub component_type: String,
}

impl From<IptComponentTypeTranslateListData> for InsertableComponentTypeTranslateList {
    fn from(ipt_data: IptComponentTypeTranslateListData) -> Self {
        let IptComponentTypeTranslateListData {
            id_component_type,
            id_lang,
            component_type,
            ..
        } = ipt_data;

        Self {
            id_component_type,
            id_lang,
            component_type,
        }
    }
}
