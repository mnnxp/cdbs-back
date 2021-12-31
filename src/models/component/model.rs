use crate::models::component::{
    component_type::model::ComponentTypeTranslateList,
    actual_status::model::ActualStatusTranslateList,
    param::model::ComponentParamWithTranslation,
    supplier::model::ComponentSupplierRelatedData,
    component_modification::model::ComponentModificationAndRelatedData,
};
use crate::models::user::model::ShowUserShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    license::model::License,
    keyword::model::Keyword,
    file::model::{ShowFileRelatedData, DownloadFile},
    spec::model::SpecTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, SimpleObject, Debug)]
#[primary_key(uuid)]
#[table_name = "component_ref"]
pub struct Component {
    pub uuid: Uuid,
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub user_uuid: Uuid,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub struct ComponentAndRelatedData {
    pub uuid: Uuid,
    pub parent_component_uuid: Uuid,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub type_access: TypeAccessTranslateList,
    pub component_type: ComponentTypeTranslateList,
    pub actual_status: ActualStatusTranslateList,
    pub is_base: bool,
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub licenses: Vec<License>,
    pub component_params: Vec<ComponentParamWithTranslation>,
    pub files: Vec<ShowFileRelatedData>,
    pub component_specs: Vec<SpecTranslateList>,
    pub component_keywords: Vec<Keyword>,
    pub component_modifications: Vec<ComponentModificationAndRelatedData>,
    pub component_suppliers: Vec<ComponentSupplierRelatedData>,
    // show the standards that fit the object
    pub component_standards: Vec<ShowStandardShort>,
}

#[derive(Debug, SimpleObject)]
pub struct ShowComponentShort {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub owner_user: ShowUserShort,
    pub type_access: TypeAccessTranslateList,
    pub component_type: ComponentTypeTranslateList,
    pub actual_status: ActualStatusTranslateList,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub is_base: bool,
    pub updated_at: NaiveDateTime,
    pub licenses: Vec<License>,
    // files for show image (models, draw)
    pub files: Vec<DownloadFile>,
    // show first supplier company
    pub component_suppliers: Vec<ComponentSupplierRelatedData>,
}

#[derive(Debug, Insertable)]
#[table_name = "component_ref"]
pub(crate) struct InsertableComponent {
    uuid: Uuid,
    parent_component_uuid: Uuid,
    name: String,
    description: String,
    user_uuid: Uuid,
    type_access_id: i32,
    component_type_id: i32,
    actual_status_id: i32,
    is_base: bool,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableComponent {
    /// Check parent component uuid on nil
    pub(crate) fn parent_uuid_is_nil(&self) -> bool {
        self.parent_component_uuid.is_nil()
    }

    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_component_uuid = Uuid::parse_str("a5953fd9-7393-4f1e-a899-06b5e159dbf1").unwrap();
    }

    /// Set user uuid (for set logged user as owner)
    pub(crate) fn set_user_uuid(&mut self, user_uuid: &Uuid) {
        self.user_uuid = *user_uuid;
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub struct IptComponentData {
    pub parent_component_uuid: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub type_access_id: i32,
    pub component_type_id: i32,
    pub actual_status_id: i32,
    pub is_base: bool,
}

impl From<&IptComponentData> for InsertableComponent {
    fn from(ipt_data: &IptComponentData) -> Self {
        let IptComponentData {
            parent_component_uuid,
            name,
            description,
            type_access_id,
            component_type_id,
            actual_status_id,
            is_base,
            ..
        } = ipt_data;

        let parent_component_uuid = match parent_component_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            parent_component_uuid,
            name: name.clone(),
            description: description.clone(),
            user_uuid: Uuid::nil(),
            type_access_id: *type_access_id,
            component_type_id: *component_type_id,
            actual_status_id: *actual_status_id,
            is_base: *is_base,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptUpdateComponentData {
    pub parent_component_uuid: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub component_type_id: Option<i32>,
    pub actual_status_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptComponentsArg {
    pub components_uuids:  Option<Vec<Uuid>>,
    pub company_uuid: Option<Uuid>,
    pub standard_uuid: Option<Uuid>,
    pub user_uuid: Option<Uuid>,
    pub favorite: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct ComponentsArg {
    pub filter_components_uuids: Vec<Uuid>,
    pub company_uuid: Option<Uuid>,
    pub standard_uuid: Option<Uuid>,
    pub user_uuid: Option<Uuid>,
    pub favorite: bool,
    pub limit: i32,
    pub offset: i32,
}

impl Default for ComponentsArg {
    fn default() -> Self {
        Self {
            filter_components_uuids: Vec::new(),
            company_uuid: None,
            standard_uuid: None,
            user_uuid: None,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptComponentsArg> for ComponentsArg {
    fn from(data: IptComponentsArg) -> Self {
        let IptComponentsArg {
            components_uuids,
            company_uuid,
            standard_uuid,
            user_uuid,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_components_uuids: components_uuids.unwrap_or_default(),
            company_uuid,
            standard_uuid,
            user_uuid,
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptComponentFilesArg {
    pub component_uuid:  Uuid,
    pub files_uuids: Option<Vec<Uuid>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct ComponentFilesArg {
    pub component_uuid:  Uuid,
    pub files_uuids: Vec<Uuid>,
    pub limit: i32,
    pub offset: i32,
}

impl From<IptComponentFilesArg> for ComponentFilesArg {
    fn from(data: IptComponentFilesArg) -> Self {
        let IptComponentFilesArg {
            component_uuid,
            files_uuids,
            limit,
            offset,
        } = data;

        Self {
            component_uuid,
            files_uuids: files_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
