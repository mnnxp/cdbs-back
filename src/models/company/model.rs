use super::company_represent::model::CompanyRepresentAndRelatedData;
use super::certificate::model::CertificateAndFile;
use super::company_type::model::CompanyTypeTranslateList;
use super::spec::model::CompanySpecWithTranslation;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::file::model::ShowFileForDownload;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[primary_key(uuid)]
#[table_name = "company_ref"]
pub struct Company {
    pub uuid: Uuid,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub user_uuid: Uuid,
    pub image_file_uuid: Uuid,
    pub region_id: i32,
    pub company_type_id: i32,
    pub type_access_id: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub struct CompanyAndRelatedData {
    pub uuid: Uuid,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub owner_user: ShowUserShort,
    pub image_file: ShowFileForDownload,
    pub region: RegionTranslateList,
    pub company_represents: Vec<CompanyRepresentAndRelatedData>,
    pub company_type: CompanyTypeTranslateList,
    // show certificates company
    pub company_certificates: Vec<CertificateAndFile>,
    pub company_specs: Vec<CompanySpecWithTranslation>,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    // count users to folloded the company
    pub subscribers: i32,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, SimpleObject)]
pub struct ShowCompanyShort {
    pub uuid: Uuid,
    pub shortname: String,
    pub inn: String,
    pub description: String,
    pub image_file: ShowFileForDownload,
    pub region: RegionTranslateList,
    pub company_type: CompanyTypeTranslateList,
    pub is_supplier: bool,
    // for display the checkbox "favorites"
    pub is_followed: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "company_ref"]
pub struct InsertableCompany {
    pub uuid: Uuid,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub user_uuid: Uuid,
    pub image_file_uuid: Uuid,
    pub region_id: i32,
    pub company_type_id: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyData {
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub region_id: i32,
    pub company_type_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateCompanyData {
    pub orgname: Option<String>,
    pub shortname: Option<String>,
    pub inn: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub site_url: Option<String>,
    pub time_zone: Option<String>,
    pub region_id: Option<i32>,
    pub company_type_id: Option<i32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompanyData {
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: String,
    pub user_uuid: Uuid,
    pub image_file_uuid: Uuid,
    pub region_id: i32,
    pub company_type_id: i32,
}

#[Object]
impl CompanyData {
    async fn orgname(&self) -> &String {
        &self.orgname
    }
    async fn shortname(&self) -> &String {
        &self.shortname
    }
    async fn inn(&self) -> &String {
        &self.inn
    }
    async fn phone(&self) -> &String {
        &self.phone
    }
    async fn email(&self) -> &String {
        &self.email
    }
    async fn description(&self) -> &String {
        &self.description
    }
    async fn address(&self) -> &String {
        &self.address
    }
    async fn site_url(&self) -> &String {
        &self.site_url
    }
    async fn time_zone(&self) -> &String {
        &self.time_zone
    }
    async fn user_uuid(&self) -> ID {
        self.user_uuid.into()
    }
    async fn image_file_uuid(&self) -> ID {
        self.image_file_uuid.into()
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn company_type_id(&self) -> &i32 {
        &self.company_type_id
    }
}


#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct SlimCompany {
    pub uuid: Uuid,
    pub shortname: String,
    pub is_supplier: bool,
}

#[Object]
impl SlimCompany {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn shortname(&self) -> &String {
        &self.shortname
    }
    async fn is_supplier(&self) -> &bool {
        &self.is_supplier
    }
}

impl From<CompanyData> for InsertableCompany {
    fn from(company_data: CompanyData) -> Self {
        let CompanyData {
            orgname,
            shortname,
            inn,
            phone,
            email,
            description,
            address,
            site_url,
            time_zone,
            user_uuid,
            image_file_uuid,
            region_id,
            company_type_id,
            ..
        } = company_data;

        let is_supplier = false;
        let is_email_verified = false;
        let is_enabled = true;
        let is_delete = false;

        Self {
            uuid: Uuid::new_v4(),
            orgname,
            shortname,
            inn,
            phone,
            email,
            description,
            address,
            site_url,
            time_zone,
            user_uuid,
            image_file_uuid,
            region_id,
            company_type_id,
            is_supplier,
            is_email_verified,
            is_enabled,
            is_delete,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<Company> for SlimCompany {
    fn from(company: Company) -> Self {
        let Company {
            uuid,
            shortname,
            is_supplier,
            ..
        } = company;

        Self {
            uuid,
            shortname,
            is_supplier,
        }
    }
}
