use crate::database::{get_conn, PooledConnection};
use crate::graphql::file::ShowFileRelatedData;
use crate::models::company::model::ShowCompanyShort;
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, region::model::RegionTranslateList,
    spec::model::SpecTranslateList, type_access::model::TypeAccessTranslateList,
};
use crate::models::standard::standard_status::model::StandardStatusTranslateList;
use crate::models::user::model::ShowUserShort;
use async_graphql::{Context, InputObject, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Full information about the standard (standardization document) and related data
#[derive(Debug)]
pub(crate) struct StandardAndRelatedData {
    /// UUID of the standard on the platform
    pub(crate) uuid: Uuid,
    /// Parent standard UUID
    pub(crate) parent_standard_uuid: Uuid,
    /// Standard name
    pub(crate) name: String,
    /// Standard description
    pub(crate) description: String,
    /// Date of publication of the document (standard)
    pub(crate) publication_at: NaiveDateTime,
    /// Data for displaying the main image of the standard
    pub(crate) image_file: DownloadFile,
    /// Data about the profile that uploaded the standard
    pub(crate) owner_user: ShowUserShort,
    /// Data about the company that owns the standard
    pub(crate) owner_company: ShowCompanyShort,
    /// Type of access to the standard data
    pub(crate) type_access: TypeAccessTranslateList,
    /// Current status of the standard (e.g., "in development")
    pub(crate) standard_status: StandardStatusTranslateList,
    /// Date the standard card was created
    pub(crate) created_at: NaiveDateTime,
    /// Date the standard's master data was updated
    pub(crate) updated_at: NaiveDateTime,
    // Связанные со стандартом данные
    /// Standard files (documentation, etc.)
    pub(crate) standard_files: Vec<ShowFileRelatedData>,
    /// Catalogs to which the standard has been added
    pub(crate) standard_specs: Vec<SpecTranslateList>,
    /// Key words (tags) of the standard
    pub(crate) standard_keywords: Vec<Keyword>,
    /// Number of people who have added the standard to their bookmarks
    pub(crate) subscribers: i32,
    /// Flag of standard presence in user's bookmarks
    pub(crate) is_followed: bool,
}

#[Object]
impl StandardAndRelatedData {
    /// UUID of the standard on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Parent standard UUID
    async fn parent_standard_uuid(&self) -> &Uuid {
        &self.parent_standard_uuid
    }

    /// Standard classification (removed)
    async fn classifier(&self) -> String {
        String::new()
    }

    /// Standard name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Standard description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Tolerance of the standard (removed)
    async fn specified_tolerance(&self) -> String {
        String::new()
    }

    /// Technical Committee (standardization body) (removed)
    async fn technical_committee(&self) -> String {
        String::new()
    }

    /// Date of publication of the document (standard)
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }

    /// Data for displaying the main image of the standard
    async fn image_file(&self) -> &DownloadFile {
        &self.image_file
    }

    /// Data about the profile that uploaded the standard
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Data about the company that owns the standard
    async fn owner_company(&self) -> &ShowCompanyShort {
        &self.owner_company
    }

    /// Type of access to the standard data
    async fn type_access(&self) -> &TypeAccessTranslateList {
        &self.type_access
    }

    /// Current status of the standard (e.g., "in development")
    async fn standard_status(&self) -> &StandardStatusTranslateList {
        &self.standard_status
    }

    /// Main region of application of the standard
    async fn region(&self, cxt: &Context<'_>) -> RegionTranslateList {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        RegionTranslateList::get_region_by_id(&8, &1, conn).expect("Error get fake reging")
    }

    /// Date the standard card was created
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date the standard's master data was updated
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Standard files (documentation, etc.)
    async fn standard_files(&self) -> &[ShowFileRelatedData] {
        &self.standard_files
    }

    /// Catalogs to which the standard has been added
    async fn standard_specs(&self) -> &[SpecTranslateList] {
        &self.standard_specs
    }

    /// Key words (tags) of the standard
    async fn standard_keywords(&self) -> &[Keyword] {
        &self.standard_keywords
    }

    /// Number of people who have added the standard to their bookmarks
    async fn subscribers(&self) -> &i32 {
        &self.subscribers
    }

    /// Flag of standard presence in user's bookmarks
    async fn is_followed(&self) -> &bool {
        &self.is_followed
    }
}

/// Abbreviated data about the standard
#[derive(Debug)]
pub(crate) struct ShowStandardShort {
    /// Standard UUID on the platform
    pub(crate) uuid: Uuid,
    /// Standard name
    pub(crate) name: String,
    /// Standard description
    pub(crate) description: String,
    /// Date of publication of the document (standard)
    pub(crate) publication_at: NaiveDateTime,
    /// Data for displaying the main image of the standard
    pub(crate) image_file: DownloadFile,
    /// Data about the company that owns the standard
    pub(crate) owner_company: ShowCompanyShort,
    /// Current status of the standard (e.g., "in development")
    pub(crate) standard_status: StandardStatusTranslateList,
    /// Date when the standard's main data was updated
    pub(crate) updated_at: NaiveDateTime,
    /// Flag of the standard in the user's bookmarks
    pub(crate) is_followed: bool,
}

#[Object]
impl ShowStandardShort {
    /// Standard UUID on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Standard classification (removed)
    async fn classifier(&self) -> String {
        String::new()
    }

    /// Standard name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Standard description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Tolerance of the standard (removed)
    async fn specified_tolerance(&self) -> String {
        String::new()
    }

    /// Date of publication of the document (standard)
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }

    /// Data for displaying the main image of the standard
    async fn image_file(&self) -> &DownloadFile {
        &self.image_file
    }

    /// Data about the company that owns the standard
    async fn owner_company(&self) -> &ShowCompanyShort {
        &self.owner_company
    }

    /// Current status of the standard (e.g., "in development")
    async fn standard_status(&self) -> &StandardStatusTranslateList {
        &self.standard_status
    }

    /// Date when the standard's main data was updated
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Flag of the standard in the user's bookmarks
    async fn is_followed(&self) -> &bool {
        &self.is_followed
    }
}

/// Data for registering a new standard on the platform
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardData {
    /// Parent standard UUID (optional)
    pub(crate) parent_standard_uuid: Option<Uuid>,
    /// Standard classification
    pub(crate) classifier: String,
    /// Standard name
    pub(crate) name: String,
    /// Standard description
    pub(crate) description: String,
    /// Tolerance of the standard
    pub(crate) specified_tolerance: String,
    /// Technical Committee (standardization body)
    pub(crate) technical_committee: String,
    /// Date of publication of the document (standard)
    pub(crate) publication_at: NaiveDateTime,
    /// Identifier of the company owning the standard
    pub(crate) company_uuid: Uuid,
    /// Identifier of the type of access to the data of the standard
    pub(crate) type_access_id: i32,
    /// Identifier of the status of the state (readiness) of the standard
    pub(crate) standard_status_id: i32,
    /// Identifier of the region of application (development) of the standard
    pub(crate) region_id: i32,
}

/// Data for updating the standard card.
/// The data is only updated for the specified values.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateStandardData {
    /// Standard classification
    pub(crate) classifier: Option<String>,
    /// Standard name
    pub(crate) name: Option<String>,
    /// Standard description
    pub(crate) description: Option<String>,
    /// Tolerance of the standard
    pub(crate) specified_tolerance: Option<String>,
    /// Technical Committee (standardization body)
    pub(crate) technical_committee: Option<String>,
    /// Date of publication of the document (standard)
    pub(crate) publication_at: Option<NaiveDateTime>,
    /// Identifier of the company owning the standard
    pub(crate) company_uuid: Option<Uuid>,
    /// Identifier of the status of the state (readiness) of the standard
    pub(crate) standard_status_id: Option<i32>,
    /// Identifier of the region of application (development) of the standard
    pub(crate) region_id: Option<i32>,
}

/// Arguments for filtering and searching by standards
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardsArg {
    /// Filter by standard UUID
    pub(crate) standards_uuids: Option<Vec<Uuid>>,
    /// Filter by company owning the standard
    pub(crate) company_uuid: Option<Uuid>,
    /// Filter by the presence of the standard in the user's favorites
    pub(crate) favorite: Option<bool>,
}

/// Arguments for filtering and searching by standard files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesArg {
    /// Filter by standard UUID
    pub(crate) standard_uuid: Uuid,
    /// Filter by standard UUID files
    pub(crate) files_uuids: Option<Vec<Uuid>>,
}
