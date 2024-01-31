use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::schema::*;
use async_graphql::{InputObject, SimpleObject};
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(standard_uuid, company_uuid))]
#[diesel(table_name = company_access_to_standard)]
pub(crate) struct CompanyAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data on the availability of access to the standard for the company (members of the company)
#[derive(Debug, Deserialize, SimpleObject)]
pub(crate) struct CompanyAccessStandardAndRelatedData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// UUID of accessing company
    pub(crate) company_uuid: Uuid,
    /// Type of access with localization
    pub(crate) type_access: TypeAccessTranslateList,
    /// Access activity flag
    pub(crate) is_enabled: bool,
    /// Date of first issuance of access to the company
    pub(crate) created_at: NaiveDateTime,
    /// Date of access update
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_access_to_standard)]
pub(crate) struct InsertableCompanyAccessStandard {
    pub(crate) standard_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data for a request to add access to a standard for company members
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyAccessStandardData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Access type (level) identifier
    pub(crate) type_access_id: i32,
}

impl From<&IptCompanyAccessStandardData> for InsertableCompanyAccessStandard {
    fn from(data_standard: &IptCompanyAccessStandardData) -> Self {
        let IptCompanyAccessStandardData {
            standard_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_standard;

        Self {
            standard_uuid: *standard_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Data to request removal of company access to the standard
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelCompanyAccessStandardData {
    /// UUID of the standard
    pub(crate) standard_uuid: Uuid,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
}
