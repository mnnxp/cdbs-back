use super::standard_status::model::StandardStatusTranslateList;
use crate::models::company::model::ShowCompanyShort;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    file::model::{ShowFileRelatedData, DownloadFile},
    region::model::RegionTranslateList,
    spec::model::SpecTranslateList,
    keyword::model::Keyword,
};
use crate::schema::*;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "standard_ref"]
pub struct Standard {
    pub uuid: Uuid,
    pub parent_standard_uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub standard_status_id: i32,
    pub region_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub struct StandardAndRelatedData {
    pub uuid: Uuid,
    pub parent_standard_uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file: DownloadFile,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    pub type_access: TypeAccessTranslateList,
    pub standard_status: StandardStatusTranslateList,
    pub region: RegionTranslateList,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub standard_files: Vec<ShowFileRelatedData>, // <-- documentation files, etc.
    pub standard_specs: Vec<SpecTranslateList>,
    pub standard_keywords: Vec<Keyword>,
    // count users to folloded the standard
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Debug, SimpleObject)]
pub struct ShowStandardShort {
    pub uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub publication_at: NaiveDateTime,
    pub owner_company: ShowCompanyShort,
    pub standard_status: StandardStatusTranslateList,
    pub updated_at: NaiveDateTime,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "standard_ref"]
pub struct InsertableStandard {
    pub uuid: Uuid,
    pub parent_standard_uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub standard_status_id: i32,
    pub region_id: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardData {
    pub parent_standard_uuid: Option<Uuid>,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub standard_status_id: i32,
    pub region_id: i32,
}

#[derive(Debug, Clone)]
pub struct StandardData {
    pub parent_standard_uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file_uuid: Uuid,
    pub user_uuid: Uuid,
    pub company_uuid: Uuid,
    pub type_access_id: i32,
    pub standard_status_id: i32,
    pub region_id: i32,
}

impl From<StandardData> for InsertableStandard {
    fn from(company_data: StandardData) -> Self {
        let StandardData {
            parent_standard_uuid,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            image_file_uuid,
            user_uuid,
            company_uuid,
            type_access_id,
            standard_status_id,
            region_id,
            ..
        } = company_data;

        // let parent_standard_uuid = Uuid::parse_str(&parent_standard_uuid).unwrap();
        // let image_file_uuid = Uuid::parse_str(&image_file_uuid).unwrap();
        // let user_uuid = Uuid::parse_str(&user_uuid).unwrap();
        // let company_uuid = Uuid::parse_str(&company_uuid).unwrap();

        Self {
            uuid: Uuid::new_v4(),
            parent_standard_uuid,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            image_file_uuid,
            user_uuid,
            company_uuid,
            type_access_id,
            standard_status_id,
            region_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateStandardData {
    pub classifier: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub specified_tolerance: Option<String>,
    pub technical_committee: Option<String>,
    pub publication_at: Option<NaiveDateTime>,
    pub company_uuid: Option<Uuid>,
    pub standard_status_id: Option<i32>,
    pub region_id: Option<i32>,
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptStandardsArg {
    pub standards_uuids:  Option<Vec<Uuid>>,
    pub company_uuid: Option<Uuid>,
    pub favorite: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct StandardsArg {
    pub filter_standards_uuids: Vec<Uuid>,
    pub company_uuid: Option<Uuid>,
    pub favorite: bool,
    pub limit: i32,
    pub offset: i32,
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
pub struct IptStandardFilesArg {
    pub standard_uuid:  Uuid,
    pub files_uuids: Option<Vec<Uuid>>,
}

#[derive(Debug)]
pub struct StandardFilesArg {
    pub standard_uuid:  Uuid,
    pub files_uuids: Vec<Uuid>,
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
