use crate::schema::*;
use crate::models::company::model::Company;
use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[derive(SimpleObject, Clone, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(belongs_to(Company, foreign_key = company_uuid))]
#[diesel(table_name = company_represent_ref)]
pub(crate) struct CompanyRepresent {
    pub(crate) uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) representation_type_id: i32,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyRepresentAndRelatedData {
    pub(crate) uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) region: RegionTranslateList,
    pub(crate) representation_type: RepresentationTypeTranslateList,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_represent_ref)]
pub(crate) struct InsertableCompanyRepresent {
    pub(crate) uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) representation_type_id: i32,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyRepresentData {
    pub(crate) company_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) representation_type_id: i32,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyRepresentData {
    pub(crate) region_id: Option<i32>,
    pub(crate) representation_type_id: Option<i32>,
    pub(crate) name: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) phone: Option<String>,
}

#[derive(Debug, Deserialize, Clone, SimpleObject)]
pub(crate) struct CompanyRepresentData {
    pub(crate) company_uuid: Uuid,
    pub(crate) region_id: i32,
    pub(crate) representation_type_id: i32,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub(crate) struct SlimCompanyRepresent {
    pub(crate) uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
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
pub(crate) struct IptCompanyRepresentsArg {
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) represents_uuids: Option<Vec<Uuid>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug, Clone)]
pub(crate) struct CompanyRepresentsArg {
    pub(crate) company_uuid: Uuid,
    pub(crate) represents_uuids: Vec<Uuid>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
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

impl CompanyRepresentsArg {
    /// Change company uuid
    pub(crate) fn set_company_uuid(&mut self, company_uuid: &Uuid) {
        self.company_uuid = *company_uuid;
    }

    /// Change represents uuids
    pub(crate) fn set_represents_uuids(&mut self, represents_uuids: Vec<Uuid>) {
        self.represents_uuids = represents_uuids;
    }
}
