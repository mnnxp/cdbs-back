use	crate::models::user::model::ShowUserShort;
use	crate::models::standard::model::ShowStandardShort;
use	crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    spec::model::SpecTranslateList,
    license::model::License,
    keyword::model::Keyword,
    file::model::ShowFileRelatedData,
    file::model::DownloadFile,
};
use	crate::models::component::{
    supplier::model::ComponentSupplierRelatedData,
    param::model::ComponentParamWithTranslation,
    component_type::model::ComponentTypeTranslateList,
    component_modification::model::ComponentModificationAndRelatedData,
    actual_status::model::ActualStatusTranslateList,
};
use async_graphql::{SimpleObject, InputObject};
use chrono::NaiveDateTime;
// use async_graphql::{Context, Object, SimpleObject, InputObject};
// use chrono::{NaiveDateTime, Local};
use uuid::Uuid;

/// Complete information about the component (part) and related data
#[derive(Debug, SimpleObject)]
pub struct ComponentAndRelatedData {
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
pub struct ShowComponentShort {
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

// #[Object]
// impl ShowComponentShort {
//         /// Identifier of the component on the platform
//         async fn uuid(&self) -> Uuid {
//             Uuid::nil()
//         }
//         /// Component name
//         async fn name(&self) -> String {
//             String::new()
//         }
//         /// Component description
//         async fn description(&self) -> String {
//             String::new()
//         }
//         /// Data for displaying the main view of the component (part)
//         async fn image_file(&self) -> DownloadFile {
//             DownloadFile {
//                 uuid: Uuid::nil(),
//                 hash: "hash".to_string(),
//                 filename: "filename".to_string(),
//                 filesize: 1,
//                 download_url: "download_url".to_string(),
//             }
//         }
//         /// Data about the profile owning the component
//         async fn owner_user(&self) -> ShowUserShort {
//             ShowUserShort {
//                 uuid: Uuid::nil(),
//                 firstname: "firstname".to_string(),
//                 lastname: "lastname".to_string(),
//                 username: "username".to_string(),
//                 image_file: DownloadFile {
//                     uuid: Uuid::nil(),
//                     hash: "hash".to_string(),
//                     filename: "filename".to_string(),
//                     filesize: 1,
//                     download_url: "download_url".to_string(),
//                 },
//             }
//         }
//         /// Type of access to the component data
//         async fn type_access(&self) -> TypeAccessTranslateList {
//             TypeAccessTranslateList {
//                 type_access_id: 1,
//                 lang_id: 1,
//                 name: "1".to_string(),
//             }
//         }
//         /// Component type (e.g. "standard")
//         async fn component_type(&self) -> ComponentTypeTranslateList {
//             ComponentTypeTranslateList {
//                 component_type_id: 1,
//                 lang_id: 1,
//                 component_type: "1".to_string(),
//             }
//         }
//         /// Current status of the component (e.g. "in development")
//         async fn actual_status(&self) -> ActualStatusTranslateList {
//             ActualStatusTranslateList {
//                 actual_status_id: 1,
//                 lang_id: 1,
//                 name: "1".to_string(),
//             }
//         }
//         /// For basic components it is possible to link to multiple manufacturers/suppliers
//         async fn is_base(&self) -> bool {
//             false
//         }
//         /// Flag whether the component is available in the user's bookmarks
//         async fn is_followed(&self) -> bool {
//             false
//         }
//         /// Update date of the basic component data
//         async fn updated_at(&self) -> NaiveDateTime {
//             Local::now().naive_local()
//         }
//         /// Component data distribution licenses
//         async fn licenses(&self) -> Vec<License> {
//             Vec::new()
//         }
//         // /// Files (images) associated with the component
//         async fn files(&self, ctx: &Context<'_>, material_type: Option<String>) -> Vec<DownloadFile> {
//             Vec::new()
//         }
//         /// Manufacturer or suppliers of the component (if is_base is true)
//         async fn component_suppliers(&self) -> Vec<ComponentSupplierRelatedData> {
//             Vec::new()
//         }
// }

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
    pub(crate) order_by: Option<String>,
    pub(crate) as_desc: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptComponentFilesArg {
    pub(crate) component_uuid:  Uuid,
    pub(crate) files_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}
