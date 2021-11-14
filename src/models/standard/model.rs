use super::standard_status::model::StandardStatusTranslateList;
use super::spec::model::StandardSpecWithTranslation;
use crate::models::company::model::ShowCompanyShort;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::file::model::{ShowFileForDownload, DownloadFile};
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
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

#[Object]
impl Standard {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn parent_standard_uuid(&self) -> ID {
        self.parent_standard_uuid.into()
    }
    async fn classifier(&self) -> &String {
        &self.classifier
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn specified_tolerance(&self) -> &String {
        &self.specified_tolerance
    }
    async fn technical_committee(&self) -> &String {
        &self.technical_committee
    }
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }
    async fn image_file_uuid(&self) -> ID {
        self.image_file_uuid.into()
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn type_access_id(&self) -> &i32 {
        &self.type_access_id
    }
    async fn standard_status_id(&self) -> &i32 {
        &self.standard_status_id
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn is_delete(&self) -> &bool {
        &self.is_delete
    }
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
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
    pub type_access_id: i32,
    pub standard_status: StandardStatusTranslateList,
    pub region: RegionTranslateList,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub standard_files: Vec<ShowFileForDownload>, // <-- documentation files, etc.
    pub standard_specs: Vec<StandardSpecWithTranslation>,
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

#[derive(Debug, Deserialize, Clone)]
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

#[Object]
impl StandardData {
    async fn parent_standard_uuid(&self) -> ID {
        self.parent_standard_uuid.into()
    }
    async fn classifier(&self) -> &String {
        &self.classifier
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn specified_tolerance(&self) -> &String {
        &self.specified_tolerance
    }
    async fn technical_committee(&self) -> &String {
        &self.technical_committee
    }
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }
    async fn image_file_uuid(&self) -> ID {
        self.image_file_uuid.into()
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn type_access_id(&self) -> &i32 {
        &self.type_access_id
    }
    async fn standard_status_id(&self) -> &i32 {
        &self.standard_status_id
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct SlimStandard {
    pub uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub standard_status_id: i32,
}

#[Object]
impl SlimStandard {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn classifier(&self) -> &String {
        &self.classifier
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn specified_tolerance(&self) -> &String {
        &self.specified_tolerance
    }
    async fn technical_committee(&self) -> &String {
        &self.technical_committee
    }
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }
    async fn standard_status_id(&self) -> &i32 {
        &self.standard_status_id
    }
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

impl From<Standard> for SlimStandard {
    fn from(company: Standard) -> Self {
        let Standard {
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            standard_status_id,
            ..
        } = company;

        Self {
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            standard_status_id,
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
