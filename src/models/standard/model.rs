use super::standard_status::model::StandardStatusTranslateList;
use super::spec::model::StandardSpecWithTranslation;
use crate::models::company::model::ShowCompanyShort;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::file::model::{ShowFile, SlimFile};
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
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl Standard {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_standard_parent(&self) -> ID {
        self.uuid_standard_parent.into()
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
    async fn uuid_image_file(&self) -> ID {
        self.uuid_image_file.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn id_type_access(&self) -> &i32 {
        &self.id_type_access
    }
    async fn id_standard_status(&self) -> &i32 {
        &self.id_standard_status
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
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

#[derive(Debug, Deserialize, SimpleObject)]
pub struct StandardAndRelatedData {
    pub uuid: Uuid,
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub image_file: SlimFile,
    pub owner_user: ShowUserShort,
    pub owner_company: ShowCompanyShort,
    pub id_type_access: i32,
    pub standard_status: StandardStatusTranslateList,
    pub region: RegionTranslateList,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    // related data
    pub standard_files: Vec<ShowFile>, // <-- documentation files, etc.
    pub standard_specs: Vec<StandardSpecWithTranslation>,
    pub standard_keywords: Vec<Keyword>,
    // count users to folloded the standard
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
}

#[derive(Debug, Deserialize, SimpleObject)]
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
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardData {
    pub uuid_standard_parent: ID,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: ID,
    pub uuid_company: ID,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StandardData {
    pub uuid_standard_parent: Uuid,
    pub classifier: String,
    pub name: String,
    pub description: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub uuid_image_file: Uuid,
    pub uuid_user: Uuid,
    pub uuid_company: Uuid,
    pub id_type_access: i32,
    pub id_standard_status: i32,
    pub id_region: i32,
}

#[Object]
impl StandardData {
    async fn uuid_standard_parent(&self) -> ID {
        self.uuid_standard_parent.into()
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
    async fn uuid_image_file(&self) -> ID {
        self.uuid_image_file.into()
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn id_type_access(&self) -> &i32 {
        &self.id_type_access
    }
    async fn id_standard_status(&self) -> &i32 {
        &self.id_standard_status
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimStandard {
    pub uuid: Uuid,
    pub classifier: String,
    pub name: String,
    pub specified_tolerance: String,
    pub technical_committee: String,
    pub publication_at: NaiveDateTime,
    pub id_standard_status: i32,
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
    async fn id_standard_status(&self) -> &i32 {
        &self.id_standard_status
    }
}

impl From<StandardData> for InsertableStandard {
    fn from(company_data: StandardData) -> Self {
        let StandardData {
            uuid_standard_parent,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            uuid_image_file,
            uuid_user,
            uuid_company,
            id_type_access,
            id_standard_status,
            id_region,
            ..
        } = company_data;

        // let uuid_standard_parent = Uuid::parse_str(&uuid_standard_parent).unwrap();
        // let uuid_image_file = Uuid::parse_str(&uuid_image_file).unwrap();
        // let uuid_user = Uuid::parse_str(&uuid_user).unwrap();
        // let uuid_company = Uuid::parse_str(&uuid_company).unwrap();

        Self {
            uuid: Uuid::new_v4(),
            uuid_standard_parent,
            classifier,
            name,
            description,
            specified_tolerance,
            technical_committee,
            publication_at,
            uuid_image_file,
            uuid_user,
            uuid_company,
            id_type_access,
            id_standard_status,
            id_region,
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
            id_standard_status,
            ..
        } = company;

        Self {
            uuid,
            classifier,
            name,
            specified_tolerance,
            technical_committee,
            publication_at,
            id_standard_status,
        }
    }
}
