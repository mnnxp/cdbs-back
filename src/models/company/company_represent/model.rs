use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

#[derive(Debug, Queryable)]
pub struct CompanyRepresent {
    // pub id: i32,
    pub uuid: Uuid,
    pub uuid_company: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct ShowCompanyRepresent {
    pub uuid: Uuid,
    pub uuid_company: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[Object]
impl ShowCompanyRepresent {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
    async fn id_representation_type(&self) -> &i32 {
        &self.id_representation_type
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn address(&self) -> &String {
        &self.address
    }
    async fn phone(&self) -> &String {
        &self.phone
    }
}

#[derive(Debug, Insertable)]
#[table_name = "company_represent_ref"]
pub struct InsertableCompanyRepresent {
    pub uuid: Uuid,
    pub uuid_company: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyRepresentData {
    pub uuid_company: ID,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompanyRepresentData {
    pub uuid_company: Uuid,
    pub id_region: i32,
    pub id_representation_type: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<IptCompanyRepresentData> for CompanyRepresentData {
    fn from(ipt_data: IptCompanyRepresentData) -> Self {
        let IptCompanyRepresentData {
            uuid_company,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        } = ipt_data;
        CompanyRepresentData {
            uuid_company: Uuid::parse_str(&uuid_company.to_string()).unwrap(),
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        }
    }
}

#[Object]
impl CompanyRepresentData {
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn id_region(&self) -> &i32 {
        &self.id_region
    }
    async fn id_representation_type(&self) -> &i32 {
        &self.id_representation_type
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn address(&self) -> &String {
        &self.address
    }
    async fn phone(&self) -> &String {
        &self.phone
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimCompanyRepresent {
    pub uuid: Uuid,
    pub uuid_company: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[Object]
impl SlimCompanyRepresent {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn uuid_company(&self) -> ID {
        self.uuid_company.into()
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn address(&self) -> &String {
        &self.address
    }
    async fn phone(&self) -> &String {
        &self.phone
    }
}

impl From<CompanyRepresentData> for InsertableCompanyRepresent {
    fn from(company_represent_data: CompanyRepresentData) -> Self {
        let CompanyRepresentData {
            uuid_company,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
            ..
        } = company_represent_data;

        // let uuid_company = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b".parse().unwrap();

        Self {
            uuid: Uuid::new_v4(),
            uuid_company,
            id_region,
            id_representation_type,
            name,
            address,
            phone,
        }
    }
}

impl From<CompanyRepresent> for SlimCompanyRepresent {
    fn from(company_represent: CompanyRepresent) -> Self {
        let CompanyRepresent {
            uuid,
            uuid_company,
            name,
            address,
            phone,
            ..
        } = company_represent;

        Self {
            uuid,
            uuid_company,
            name,
            address,
            phone,
        }
    }
}
