use crate::models::company::company_represent::representation_type::model::RepresentationTypeTranslateList;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

/// Data on the company's representative office.
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = company_represent_ref)]
pub(crate) struct CompanyRepresent {
    /// Company representative office UUID
    pub(crate) uuid: Uuid,
    /// Company's UUID
    pub(crate) company_uuid: Uuid,
    /// Identifier of the region to which the representative office belongs
    pub(crate) region_id: i32,
    /// Identifier of the type of the company's representative office
    pub(crate) representation_type_id: i32,
    /// Name of the representative office
    pub(crate) name: String,
    /// Address of the company's representative office
    pub(crate) address: String,
    /// Phone number of the company's representative office
    pub(crate) phone: String,
}

/// Complete data on the company's representative office
/// with localization for the selected language (or English by default).
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyRepresentAndRelatedData {
    /// UUID of the company's representative office
    pub(crate) uuid: Uuid,
    /// Company's UUID
    pub(crate) company_uuid: Uuid,
    /// Data of the region to which the representative office belongs
    pub(crate) region: RegionTranslateList,
    /// Data on the type of the representative office
    pub(crate) representation_type: RepresentationTypeTranslateList,
    /// Name of the representative office
    pub(crate) name: String,
    /// Address of the company's representative office
    pub(crate) address: String,
    /// Phone number of the company's representative office
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

/// Data for adding a new company representative office.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanyRepresentData {
    /// Company's UUID
    pub(crate) company_uuid: Uuid,
    /// Identifier of the region to which the representative office belongs
    pub(crate) region_id: i32,
    /// Identifier of the type of the company's representative office
    pub(crate) representation_type_id: i32,
    /// Name of the representative office
    pub(crate) name: String,
    /// Address of the representative office
    pub(crate) address: String,
    /// Phone number of the company's representative office
    pub(crate) phone: String,
}

/// Data for updating the company representation card.
/// The data is updated only for the specified values.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateCompanyRepresentData {
    /// Identifier of the region to which the representative office belongs
    pub(crate) region_id: Option<i32>,
    /// Identifier of the type of the company's representative office
    pub(crate) representation_type_id: Option<i32>,
    /// Name of the representative office
    pub(crate) name: Option<String>,
    /// Address of the representative office
    pub(crate) address: Option<String>,
    /// Phone number of the company's representative office
    pub(crate) phone: Option<String>,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptCompanyRepresentsArg {
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) represents_uuids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone)]
pub(crate) struct CompanyRepresentsArg {
    pub(crate) company_uuid: Uuid,
    pub(crate) represents_uuids: Vec<Uuid>,
    pub(crate) set_lang_id: i32,
}

impl CompanyRepresentsArg {
    /// Returns a CompanyRepresentsArg with the given arguments and language
    pub(crate) fn by_arg(data: IptCompanyRepresentsArg, set_lang_id: i32) -> Self {
        let IptCompanyRepresentsArg {
            company_uuid,
            represents_uuids,
        } = data;

        Self {
            company_uuid: company_uuid.unwrap_or_default(),
            represents_uuids: represents_uuids.unwrap_or_default(),
            set_lang_id,
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
