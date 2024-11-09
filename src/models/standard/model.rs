use super::standard_status::model::StandardStatusTranslateList;
use crate::models::company::model::ShowCompanyShort;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    file::model::{ShowFileRelatedData, DownloadFile},
    file::util::get_default_image,
    region::model::RegionTranslateList,
    spec::model::SpecTranslateList,
    keyword::model::Keyword,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref ROOT_STANDARD_UUID : Uuid =
        Uuid::parse_str("303ec2aa-2066-42e3-93fb-de4fb9344bcb")
            .expect("Set root standard uuid failed!");
}

#[derive(Identifiable, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = standard_ref)]
pub(crate) struct Standard {
    pub(crate) uuid: Uuid,
    pub(crate) parent_standard_uuid: Uuid,
    pub(crate) classifier: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) specified_tolerance: String,
    pub(crate) technical_committee: String,
    pub(crate) publication_at: NaiveDateTime,
    pub(crate) image_file_uuid: Uuid,
    pub(crate) user_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) standard_status_id: i32,
    pub(crate) region_id: i32,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Full information about the standard (standardization document) and related data
#[derive(Debug, SimpleObject)]
pub(crate) struct StandardAndRelatedData {
    /// UUID of the standard on the platform
    pub(crate) uuid: Uuid,
    /// Parent standard UUID
    pub(crate) parent_standard_uuid: Uuid,
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
    /// Main region of application of the standard
    pub(crate) region: RegionTranslateList,
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

/// Abbreviated data about the standard
#[derive(Debug, SimpleObject)]
pub(crate) struct ShowStandardShort {
    /// Standard UUID on the platform
    pub(crate) uuid: Uuid,
    /// Standard classification
    pub(crate) classifier: String,
    /// Standard name
    pub(crate) name: String,
    /// Standard description
    pub(crate) description: String,
    /// Tolerance of the standard
    pub(crate) specified_tolerance: String,
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

#[derive(Debug, Insertable)]
#[diesel(table_name = standard_ref)]
pub(crate) struct InsertableStandard {
    uuid: Uuid,
    parent_standard_uuid: Uuid,
    classifier: String,
    name: String,
    description: String,
    specified_tolerance: String,
    technical_committee: String,
    publication_at: NaiveDateTime,
    image_file_uuid: Uuid,
    user_uuid: Uuid,
    company_uuid: Uuid,
    type_access_id: i32,
    standard_status_id: i32,
    region_id: i32,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableStandard {
    /// Check parent standard uuid on nil
    pub(crate) fn parent_uuid_is_nil(&self) -> bool {
        self.parent_standard_uuid.is_nil()
    }

    /// Change parent uuid to base for insert new row
    pub(crate) fn parent_uuid_to_base(&mut self) {
        self.parent_standard_uuid = *ROOT_STANDARD_UUID;
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

impl From<&IptStandardData> for InsertableStandard {
    fn from(ipt_data: &IptStandardData) -> Self {
        let IptStandardData {
            parent_standard_uuid,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            company_uuid,
            type_access_id,
            standard_status_id,
            region_id,
        } = ipt_data;

        let parent_standard_uuid = match parent_standard_uuid {
            Some(parent_uuid) => *parent_uuid,
            None => Uuid::nil(),
        };

        Self {
            uuid: Uuid::new_v4(),
            parent_standard_uuid,
            classifier: classifier.clone(),
            name: name.clone(),
            description: description.clone(),
            specified_tolerance: specified_tolerance.clone(),
            technical_committee: technical_committee.clone(),
            publication_at: *publication_at,
            image_file_uuid: Uuid::nil(),
            user_uuid: Uuid::nil(),
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            standard_status_id: *standard_status_id,
            region_id: *region_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
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

#[derive(Debug, Default)]
pub(crate) struct StandardsArg {
    pub(crate) filter_standards_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
}

impl From<IptStandardsArg> for StandardsArg {
    fn from(data: IptStandardsArg) -> Self {
        let IptStandardsArg {
            standards_uuids,
            company_uuid,
            favorite,
        } = data;

        Self {
            filter_standards_uuids: standards_uuids.unwrap_or_default(),
            company_uuid,
            favorite: favorite.unwrap_or(false),
        }
    }
}

/// Arguments for filtering and searching by standard files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesArg {
    /// Filter by standard UUID
    pub(crate) standard_uuid: Uuid,
    /// Filter by standard UUID files
    pub(crate) files_uuids: Option<Vec<Uuid>>,
}

#[derive(Debug)]
pub(crate) struct StandardFilesArg {
    pub(crate) standard_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}

impl From<IptStandardFilesArg> for StandardFilesArg {
    fn from(data: IptStandardFilesArg) -> Self {
        Self {
            standard_uuid: data.standard_uuid,
            file_uuids: data.files_uuids.unwrap_or_default(),
        }
    }
}
