use super::company_represent::model::CompanyRepresentAndRelatedData;
use super::certificate::model::CertificateWithSlimFile;
use super::company_type::model::CompanyTypeTranslateList;
use super::spec::model::CompanySpecWithTranslation;
use crate::models::user::model::ShowUserShort;
use crate::models::relate_ref::file::model::SlimFile;
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
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_company_type: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, SimpleObject)]
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
    pub image_file: SlimFile,
    pub region: RegionTranslateList,
    pub company_represents: Vec<CompanyRepresentAndRelatedData>,
    pub company_type: CompanyTypeTranslateList,
    // show certificates company
    pub company_certificates: Vec<CertificateWithSlimFile>,
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

#[derive(Debug, Deserialize, SimpleObject)]
pub struct ShowCompanyShort {
    pub uuid: Uuid,
    pub shortname: String,
    pub inn: String,
    pub description: String,
    pub image_file: SlimFile,
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
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_company_type: i32,
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
    pub uuid_image_file: ID,
    pub id_region: i32,
    pub id_company_type: i32,
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
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_company_type: i32,
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
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn uuid_image_file(&self) -> ID {
        self.uuid_image_file.into()
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
    async fn id_company_type(&self) -> &i32 {
        &self.id_company_type
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
            uuid_user,
            uuid_image_file,
            id_region,
            id_company_type,
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
            uuid_user,
            uuid_image_file,
            id_region,
            id_company_type,
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
