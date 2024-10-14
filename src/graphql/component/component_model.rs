use crate::database::{get_conn, PooledConnection};
use crate::graphql::relate::attributes::{IptPaginate, IptSort};
use	crate::models::user::model::ShowUserShort;
use	crate::models::standard::model::ShowStandardShort;
use	crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    spec::model::SpecTranslateList,
    license::model::License,
    keyword::model::Keyword,
    file::model::ShowFileRelatedData,
    file::model::DownloadFile,
    language::get_set_language,
};
use	crate::models::component::{
    supplier::model::ComponentSupplierRelatedData,
    param::model::ComponentParamWithTranslation,
    component_type::model::ComponentTypeTranslateList,
    actual_status::model::ActualStatusTranslateList,
    component_modification::{
        param::model::ModificationParamWithTranslation,
        fileset_for_program::model::FilesetProgramRelatedData,
    },
};
use crate::models::search::order::{Paginate, Sort, TableName};
use crate::models::search::model::ExtraOptions;
use async_graphql::{Context, Object, InputObject};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Complete information about component (part) and related data.
/// Default sorting: `createdAt`. Sorting by `name`, `actualStatusId`, `updatedAt` is available.
#[derive(Debug)]
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
    /// Catalogs to which the component is added
    pub(crate) component_specs: Vec<SpecTranslateList>,
    /// Component keywords (tags)
    pub(crate) component_keywords: Vec<Keyword>,
}

#[Object]
impl ComponentAndRelatedData {
    /// Identifier of the component on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Parent component identifier
    async fn parent_component_uuid(&self) -> &Uuid {
        &self.parent_component_uuid
    }

    /// Component name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Component description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Data for displaying the main view of the component (part)
    async fn image_file(&self) -> &DownloadFile {
        &self.image_file
    }

    /// Data about the profile owning the component
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Type of access to the component data
    async fn type_access(&self) -> &TypeAccessTranslateList {
        &self.type_access
    }

    /// Component type (e.g. "standard")
    async fn component_type(&self) -> &ComponentTypeTranslateList {
        &self.component_type
    }

    /// Current status of the component (e.g. "in development")
    async fn actual_status(&self) -> &ActualStatusTranslateList {
        &self.actual_status
    }

    /// For basic components it is possible to link to multiple manufacturers/suppliers
    async fn is_base(&self) -> &bool {
        &self.is_base
    }

    /// Number of people who have added the component to bookmarks
    async fn subscribers(&self) -> &i32 {
        &self.subscribers
    }

    /// Flag of the presence of the component in the user's bookmarks
    async fn is_followed(&self) -> &bool {
        &self.is_followed
    }

    /// Date when the component profile was created
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date of updating the component's master data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    // Связанные с компонентом данные
    /// Component data distribution licenses
    async fn licenses(&self) -> &[License] {
        &self.licenses
    }

    /// Data on component parameters (list).
    /// Default sorting: `paramId`. Sorting by `paramname` and `value` is available.
    async fn component_params(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ComponentParamWithTranslation> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::ParamTranslateList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::parsing(TableName::ParamTranslateList, "", false));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ComponentParamWithTranslation::by_component_uuid(&self.uuid, &get_set_language(cxt), &s, &p, conn)
           .expect("Error loading component parameters")
    }

    /// Files associated with the component. Default sorting: `createdAt`.
    /// Sorting by `revision`, `filename`, `size`, `updatedAt` is available.
    async fn files(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ShowFileRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::FileRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::parsing(TableName::FileRef, "", false));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ShowFileRelatedData::by_component_uuid(&self.uuid, &s, &p, conn)
            .expect("Error loading component files")
    }

    /// Catalogs to which the component is added
    async fn component_specs(&self) -> &[SpecTranslateList] {
        &self.component_specs
    }

    /// Component keywords (tags)
    async fn component_keywords(&self) -> &[Keyword] {
        &self.component_keywords
    }

    /// Component modifications and related data (such as filesets for CADs)
    /// Default sorting: `createdAt`. Sorting by `name`, `actualStatusId`, `updatedAt` is available.
    async fn component_modifications(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ComponentModificationAndRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::ComponentModification, &s.by_field, s.as_desc))
            .unwrap_or(Sort::parsing(TableName::ComponentModification, "", false));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        // get list component modifications with related data and translation
        ComponentModificationAndRelatedData::by_args(&self.uuid, &s, &p, &get_set_language(cxt), conn)
            .expect("Error loading component modifications with related data")
    }

    /// Manufacturer or suppliers of the component (if component.is_base is true)
    async fn component_suppliers(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ComponentSupplierRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ComponentSupplierRelatedData::by_component_uuid(&self.uuid, &p, conn)
            .expect("Error loading component suppliers")
    }

    /// List of standards associated with the component.
    /// Standards include standardization documents and related information.
    async fn component_standards(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ShowStandardShort> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let options = ExtraOptions::from_cxt(cxt).expect("Failed to get options");
        // collect data for component standards
        ShowStandardShort::for_component(&self.uuid, &p, &options, conn)
            .expect("Error loading supplier component with relate")
    }

}

/// Abbreviated component data
#[derive(Debug)]
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
}

#[Object]
impl ShowComponentShort {
    /// Identifier of the component on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Component name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Component description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Data for displaying the main view of the component (part)
    async fn image_file(&self) -> &DownloadFile {
        &self.image_file
    }

    /// Data about the profile owning the component
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Type of access to the component data
    async fn type_access(&self) -> &TypeAccessTranslateList {
        &self.type_access
    }

    /// Component type (e.g. "standard")
    async fn component_type(&self) -> &ComponentTypeTranslateList {
        &self.component_type
    }

    /// Current status of the component (e.g. "in development")
    async fn actual_status(&self) -> &ActualStatusTranslateList {
        &self.actual_status
    }

    /// For basic components it is possible to link to multiple manufacturers/suppliers
    async fn is_base(&self) -> bool {
        self.is_base
    }

    /// Flag whether the component is available in the user's bookmarks
    async fn is_followed(&self) -> bool {
        self.is_followed
    }

    /// Update date of the basic component data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Component data distribution licenses
    async fn licenses(&self) -> &[License] {
        &self.licenses
    }

    /// Files (images by default) associated with the component
    async fn files(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
        images: Option<bool>,
    ) -> Vec<DownloadFile> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        match images {
            Some(false) => DownloadFile::by_component_uuid(&self.uuid, &p, conn)
                .expect("Error loading component files"),
            _ => DownloadFile::component_image_files(&self.uuid, &p, conn)
                .expect("Error loading component image files"),
        }
    }

    /// Manufacturer or suppliers of the component (if component.is_base is true)
    async fn component_suppliers(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ComponentSupplierRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ComponentSupplierRelatedData::by_component_uuid(&self.uuid, &p, conn)
            .expect("Error loading component suppliers")
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


/// Full information about component (part) modification and related data
#[derive(Deserialize, Debug)]
pub(crate) struct ComponentModificationAndRelatedData {
    /// UUID of the component modification
    pub(crate) uuid: Uuid,
    /// UUID of component
    pub(crate) component_uuid: Uuid,
    /// UUID of the parent modification of the component
    pub(crate) parent_modification_uuid: Uuid,
    /// Name of the component modification
    pub(crate) modification_name: String,
    /// Description of the component modification
    pub(crate) description: String,
    /// Current status of the component modification
    pub(crate) actual_status: ActualStatusTranslateList,
    /// Date of creation of the component modification
    pub(crate) created_at: NaiveDateTime,
    /// Date when the main data of the component modification was changed
    pub(crate) updated_at: NaiveDateTime,
}

#[Object]
impl ComponentModificationAndRelatedData {
    /// UUID of the component modification
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// UUID of component
    async fn component_uuid(&self) -> &Uuid {
        &self.component_uuid
    }

    /// UUID of the parent modification of the component
    async fn parent_modification_uuid(&self) -> &Uuid {
        &self.parent_modification_uuid
    }

    /// Name of the component modification
    async fn modification_name(&self) -> &String {
        &self.modification_name
    }

    /// Description of the component modification
    async fn description(&self) -> &String {
        &self.description
    }

    /// Current status of the component modification
    async fn actual_status(&self) -> &ActualStatusTranslateList {
        &self.actual_status
    }

    /// Date of creation of the component modification
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date when the main data of the component modification was changed
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Component modification file sets data (list)
    async fn filesets_for_program(&self, cxt: &Context<'_>) -> Vec<FilesetProgramRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        FilesetProgramRelatedData::by_modification_uuid(&self.uuid, conn)
            .expect("Error loading filesets for modification")
    }

    /// Data on component modification parameters (list).
    /// Default sorting: `paramId`. Sorting by `paramname` and `value` is available.
    async fn modification_params(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ModificationParamWithTranslation> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::ParamTranslateList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::parsing(TableName::ParamTranslateList, "", false));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ModificationParamWithTranslation::by_modification_uuid(&self.uuid, &get_set_language(cxt), &s, &p, conn)
            .expect("Error loading parameters for modification")
    }
}