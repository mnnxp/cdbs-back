use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct Company {
    pub id: i32,
    pub uuid: Uuid,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: i32,
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_type_org: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowCompany {
    pub uuid: Uuid,
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: i32,
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_type_org: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[Object]
impl ShowCompany {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn orgname(&self) -> String {
        self.orgname.clone()
    }
    async fn shortname(&self) -> String {
        self.shortname.clone()
    }
    async fn inn(&self) -> String {
        self.inn.clone()
    }
    async fn phone(&self) -> String {
        self.phone.clone()
    }
    async fn email(&self) -> String {
        self.email.clone()
    }
    async fn description(&self) -> String {
        self.description.clone()
    }
    async fn address(&self) -> String {
        self.address.clone()
    }
    async fn site_url(&self) -> String {
        self.site_url.clone()
    }
    async fn time_zone(&self) -> i32 {
        self.time_zone.into()
    }
    async fn uuid_user(&self) -> ID {
        self.uuid_user.into()
    }
    async fn uuid_image_file(&self) -> ID {
        self.uuid_image_file.into()
    }
    async fn id_region(&self) -> i32 {
        self.id_region.into()
    }
    async fn id_type_org(&self) -> i32 {
        self.id_type_org.into()
    }
    async fn is_supplier(&self) -> bool {
        self.is_supplier.into()
    }
    async fn is_email_verified(&self) -> bool {
        self.is_email_verified.into()
    }
    async fn is_enabled(&self) -> bool {
        self.is_enabled.into()
    }
    async fn is_delete(&self) -> bool {
        self.is_delete.into()
    }
    async fn created_at(&self) -> NaiveDateTime {
        self.created_at.into()
    }
    async fn updated_at(&self) -> NaiveDateTime {
        self.updated_at.into()
    }
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
    pub time_zone: i32,
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_type_org: i32,
    pub is_supplier: bool,
    pub is_email_verified: bool,
    pub is_enabled: bool,
    pub is_delete: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CompanyData {
    pub orgname: String,
    pub shortname: String,
    pub inn: String,
    pub phone: String,
    pub email: String,
    pub description: String,
    pub address: String,
    pub site_url: String,
    pub time_zone: i32,
    pub uuid_user: Uuid,
    pub uuid_image_file: Uuid,
    pub id_region: i32,
    pub id_type_org: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimCompany {
    pub uuid: Uuid,
    pub shortname: String,
    pub is_supplier: bool,
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
            id_type_org,
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
            id_type_org,
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
