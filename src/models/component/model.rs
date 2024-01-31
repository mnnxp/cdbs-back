use crate::models::component::{
    component_type::model::ComponentTypeTranslateList,
    actual_status::model::ActualStatusTranslateList,
    param::model::ComponentParamWithTranslation,
    supplier::model::ComponentSupplierRelatedData,
    component_modification::model::ComponentModificationAndRelatedData,
    util::get_root_component_uuid,
};
use crate::models::user::model::ShowUserShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    license::model::License,
    keyword::model::Keyword,
    file::model::{ShowFileRelatedData, DownloadFile},
    file::util::get_default_image,
    spec::model::SpecTranslateList,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = component_ref)]
pub(crate) struct Component {
    pub(crate) uuid: Uuid,
    pub(crate) parent_component_uuid: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) component_type_id: i32,
    pub(crate) actual_status_id: i32,
    pub(crate) is_base: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Complete information about the component (part) and related data
#[derive(Debug, SimpleObject)]
pub(crate) struct ComponentAndRelatedData {
    /// Identifier of the component on the platform
    pub(crate) uuid: Uuid,
    /// Parent component identifier
    pub(crate) parent_component_uuid: Uuid,
    /// Component name
    pub(crate) name: String,
    /// Component description
    pub(crate) description: String,
    /// Data for displaying the main view of the component (part)
    pub(crate) image_file: DownloadFile,
    /// Data about the profile owning the component
    pub(crate) owner_user: ShowUserShort,
    /// Type of access to the component data
    pub(crate) type_access: TypeAccessTranslateList,
    /// Component type (e.g. "standard")
    pub(crate) component_type: ComponentTypeTranslateList,
    /// Current status of the component (e.g. "in development")
    pub(crate) actual_status: ActualStatusTranslateList,
    /// For basic components it is possible to link to multiple manufacturers/suppliers
    pub(crate) is_base: bool,
    /// Number of people who have added the component to bookmarks
    pub(crate) subscribers: i32,
    /// Flag of the presence of the component in the user's bookmarks
    pub(crate) is_followed: bool,
    /// Date when the component profile was created
    pub(crate) created_at: NaiveDateTime,
    /// Date of updating the component's master data
    pub(crate) updated_at: NaiveDateTime,
    // Связанные с компонентом данные
    /// Component data distribution licenses
    pub(crate) licenses: Vec<License>,
    /// List of component parameters
    pub(crate) component_params: Vec<ComponentParamWithTranslation>,
    /// Files associated with the component
    pub(crate) files: Vec<ShowFileRelatedData>,
    /// Catalogs to which the component is added
    pub(crate) component_specs: Vec<SpecTranslateList>,
    /// Component keywords (tags)
    pub(crate) component_keywords: Vec<Keyword>,
    /// Component modifications and related data (such as CAD file sets)
    pub(crate) component_modifications: Vec<ComponentModificationAndRelatedData>,
    /// Manufacturer or suppliers of the component (if is_base is true)
    pub(crate) component_suppliers: Vec<ComponentSupplierRelatedData>,
    /// List of standardization documents associated with the component
    pub(crate) component_standards: Vec<ShowStandardShort>,
}

/// Abbreviated component data
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowComponentShort {
    /// Identifier of the component on the platform
    pub(crate) uuid: Uuid,
    /// Component name
    pub(crate) name: String,
    /// Component description
    pub(crate) description: String,
    /// Data for displaying the main view of the component (part)
    pub(crate) image_file: DownloadFile,
    /// Data about the profile owning the component
    pub(crate) owner_user: ShowUserShort,
    /// Type of access to the component data
    pub(crate) type_access: TypeAccessTranslateList,
    /// Component type (e.g. "standard")
    pub(crate) component_type: ComponentTypeTranslateList,
    /// Current status of the component (e.g. "in development")
    pub(crate) actual_status: ActualStatusTranslateList,
    /// For basic components it is possible to link to multiple manufacturers/suppliers
    pub(crate) is_base: bool,
    /// Flag whether the component is available in the user's bookmarks
    pub(crate) is_followed: bool,
    /// Update date of the basic component data
    pub(crate) updated_at: NaiveDateTime,
    /// Component data distribution licenses
    pub(crate) licenses: Vec<License>,
    /// Files (images) associated with the component
    pub(crate) files: Vec<DownloadFile>,
    /// Manufacturer or suppliers of the component (if is_base is true)
    pub(crate) component_suppliers: Vec<ComponentSupplierRelatedData>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = component_ref)]
pub(crate) struct InsertableComponent {
    uuid: Uuid,
    parent_component_uuid: Uuid,
    name: String,
    description: String,
    image_file_uuid: Uuid,
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
        self.parent_component_uuid = get_root_component_uuid();
    }

    /// Set image uuid (for set default image)
    pub(crate) fn set_image_uuid(&mut self) {
        self.image_file_uuid = get_default_image();
    }

    /// Set user uuid (for set logged user as owner)
    pub(crate) fn set_user_uuid(&mut self, user_uuid: &Uuid) {
        self.user_uuid = *user_uuid;
    }
}

#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptComponentData {
    pub(crate) parent_component_uuid: Option<Uuid>,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) type_access_id: i32,
    pub(crate) component_type_id: i32,
    pub(crate) actual_status_id: i32,
    pub(crate) is_base: bool,
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
            image_file_uuid: Uuid::nil(),
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
pub(crate) struct IptUpdateComponentData {
    pub(crate) parent_component_uuid: Option<Uuid>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) component_type_id: Option<i32>,
    pub(crate) actual_status_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentsArg {
    pub(crate) components_uuids:  Option<Vec<Uuid>>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentsArg {
    pub(crate) filter_components_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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
pub(crate) struct IptComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct ComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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
            file_uuids: files_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
