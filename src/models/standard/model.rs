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
            .expect("Set default image uuid failed!");
}

#[derive(Identifiable, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "standard_ref"]
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

#[derive(Debug, SimpleObject)]
pub(crate) struct StandardAndRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) parent_standard_uuid: Uuid,
    pub(crate) classifier: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) specified_tolerance: String,
    pub(crate) technical_committee: String,
    pub(crate) publication_at: NaiveDateTime,
    pub(crate) image_file: DownloadFile,
    pub(crate) owner_user: ShowUserShort,
    pub(crate) owner_company: ShowCompanyShort,
    pub(crate) type_access: TypeAccessTranslateList,
    pub(crate) standard_status: StandardStatusTranslateList,
    pub(crate) region: RegionTranslateList,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
    // related data
    pub(crate) standard_files: Vec<ShowFileRelatedData>, // <-- documentation files, etc.
    pub(crate) standard_specs: Vec<SpecTranslateList>,
    pub(crate) standard_keywords: Vec<Keyword>,
    // count users to folloded the standard
    pub(crate) subscribers: i32,
    // for display the checkbox "favorites"
    pub(crate) is_followed: bool,
}

#[derive(Debug, SimpleObject)]
pub(crate) struct ShowStandardShort {
    pub(crate) uuid: Uuid,
    pub(crate) classifier: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) specified_tolerance: String,
    pub(crate) publication_at: NaiveDateTime,
    // for display main image
    pub(crate) image_file: DownloadFile,
    pub(crate) owner_company: ShowCompanyShort,
    pub(crate) standard_status: StandardStatusTranslateList,
    pub(crate) updated_at: NaiveDateTime,
    // for display the checkbox "favorites"
    pub(crate) is_followed: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_ref"]
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptStandardData {
    pub(crate) parent_standard_uuid: Option<Uuid>,
    pub(crate) classifier: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) specified_tolerance: String,
    pub(crate) technical_committee: String,
    pub(crate) publication_at: NaiveDateTime,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) standard_status_id: i32,
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateStandardData {
    pub(crate) classifier: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) specified_tolerance: Option<String>,
    pub(crate) technical_committee: Option<String>,
    pub(crate) publication_at: Option<NaiveDateTime>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) standard_status_id: Option<i32>,
    pub(crate) region_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardsArg {
    pub(crate) standards_uuids:  Option<Vec<Uuid>>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) favorite: Option<bool>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct StandardsArg {
    pub(crate) filter_standards_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) favorite: bool,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for StandardsArg {
    fn default() -> Self {
        Self {
            filter_standards_uuids: Vec::new(),
            company_uuid: None,
            favorite: false,
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptStandardsArg> for StandardsArg {
    fn from(data: IptStandardsArg) -> Self {
        let IptStandardsArg {
            standards_uuids,
            company_uuid,
            favorite,
            limit,
            offset,
        } = data;

        Self {
            filter_standards_uuids: standards_uuids.unwrap_or_default(),
            company_uuid,
            favorite: favorite.unwrap_or(false),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptStandardFilesArg {
    pub(crate) standard_uuid:  Uuid,
    pub(crate) files_uuids: Option<Vec<Uuid>>,
}

#[derive(Debug)]
pub(crate) struct StandardFilesArg {
    pub(crate) standard_uuid:  Uuid,
    pub(crate) files_uuids: Vec<Uuid>,
}

impl From<IptStandardFilesArg> for StandardFilesArg {
    fn from(data: IptStandardFilesArg) -> Self {
        let IptStandardFilesArg {
            standard_uuid,
            files_uuids,
        } = data;

        Self {
            standard_uuid,
            files_uuids: files_uuids.unwrap_or_default(),
        }
    }
}
