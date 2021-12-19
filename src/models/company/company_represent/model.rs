use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptUpdateCompanyRepresentData {
    pub region_id: Option<i32>,
    pub representation_type_id: Option<i32>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Clone, SimpleObject)]
pub struct CompanyRepresentData {
    pub company_uuid: Uuid,
    pub region_id: i32,
    pub representation_type_id: i32,
    pub name: String,
    pub address: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct SlimCompanyRepresent {
    pub uuid: Uuid,
    pub company_uuid: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<&IptCompanyRepresentData> for InsertableCompanyRepresent {
    fn from(company_represent_data: &IptCompanyRepresentData) -> Self {
        let IptCompanyRepresentData {
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
            company_uuid: *company_uuid,
            region_id: *region_id,
            representation_type_id: *representation_type_id,
            name: name.clone(),
            address: address.clone(),
            phone: phone.clone(),
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


#[derive(InputObject, Deserialize, Debug)]
pub struct IptCompanyRepresentsArg {
    pub company_uuid: Option<Uuid>,
    pub represents_uuids: Option<Vec<Uuid>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct CompanyRepresentsArg {
    pub company_uuid: Uuid,
    pub represents_uuids: Vec<Uuid>,
    pub limit: i32,
    pub offset: i32,
}

impl From<IptCompanyRepresentsArg> for CompanyRepresentsArg {
    fn from(data: IptCompanyRepresentsArg) -> Self {
        let IptCompanyRepresentsArg {
            company_uuid,
            represents_uuids,
            limit,
            offset,
        } = data;

        Self {
            company_uuid: company_uuid.unwrap_or_default(),
            represents_uuids: represents_uuids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
