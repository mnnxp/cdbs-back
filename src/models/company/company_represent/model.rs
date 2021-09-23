use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use async_graphql::types::ID;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[table_name = "company_represent_ref"]
pub struct CompanyRepresent {
    pub uuid: Uuid,
    pub company_uuid: Uuid,
    pub region_id: i32,
    pub representation_type_id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[Object]
impl CompanyRepresent {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn representation_type_id(&self) -> &i32 {
        &self.representation_type_id
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

#[derive(Debug, Deserialize, SimpleObject)]
pub struct CompanyRepresentAndRelatedData {
    pub uuid: Uuid,
    pub company_uuid: Uuid,
    pub region: RegionTranslateList,
    pub representation_type: RepresentationTypeTranslateList,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Insertable)]
#[table_name = "company_represent_ref"]
pub struct InsertableCompanyRepresent {
    pub uuid: Uuid,
    pub company_uuid: Uuid,
    pub region_id: i32,
    pub representation_type_id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanyRepresentData {
    pub company_uuid: Uuid,
    pub region_id: i32,
    pub representation_type_id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompanyRepresentData {
    pub company_uuid: Uuid,
    pub region_id: i32,
    pub representation_type_id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<IptCompanyRepresentData> for CompanyRepresentData {
    fn from(ipt_data: IptCompanyRepresentData) -> Self {
        let IptCompanyRepresentData {
            company_uuid,
            region_id,
            representation_type_id,
            name,
            address,
            phone,
        } = ipt_data;
        CompanyRepresentData {
            company_uuid: Uuid::parse_str(&company_uuid.to_string()).unwrap(),
            region_id,
            representation_type_id,
            name,
            address,
            phone,
        }
    }
}

#[Object]
impl CompanyRepresentData {
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
    }
    async fn region_id(&self) -> &i32 {
        &self.region_id
    }
    async fn representation_type_id(&self) -> &i32 {
        &self.representation_type_id
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
    pub company_uuid: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[Object]
impl SlimCompanyRepresent {
    async fn uuid(&self) -> ID {
        self.uuid.into()
    }
    async fn company_uuid(&self) -> ID {
        self.company_uuid.into()
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
            company_uuid,
            region_id,
            representation_type_id,
            name,
            address,
            phone,
            ..
        } = company_represent_data;

        // let company_uuid = "31ecc6f8-0c09-4a59-a2d5-34b5b833e59b".parse().unwrap();

        Self {
            uuid: Uuid::new_v4(),
            company_uuid,
            region_id,
            representation_type_id,
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
            company_uuid,
            name,
            address,
            phone,
            ..
        } = company_represent;

        Self {
            uuid,
            company_uuid,
            name,
            address,
            phone,
        }
    }
}
