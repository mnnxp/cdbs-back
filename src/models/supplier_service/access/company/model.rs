use crate::auth::permission::PermissionTranslateList;
use crate::schema::*;
use async_graphql::InputObject;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(service_uuid, company_uuid))]
#[diesel(table_name = company_access_to_service)]
pub(crate) struct CompanyAccessService {
    pub(crate) service_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data on the availability of access to the service for the company (members of the company)
#[derive(Debug, Deserialize)]
pub(crate) struct CompanyAccessServiceAndRelatedData {
    /// UUID of the service
    pub(crate) service_uuid: Uuid,
    /// UUID of accessing company
    pub(crate) company_uuid: Uuid,
    /// Access level with localization
    pub(crate) permission: PermissionTranslateList,
    /// Access activity flag
    pub(crate) is_enabled: bool,
    /// Date of first issuance of access to the company
    pub(crate) created_at: NaiveDateTime,
    /// Date of access update
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_access_to_service)]
pub(crate) struct InsertableCompanyAccessService {
    pub(crate) service_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data for a request to add access to a service for company members
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyAccessServiceData {
    /// UUID of the service
    pub(crate) service_uuid: Uuid,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Access type (level) identifier
    pub(crate) type_access_id: i32,
}

impl From<&IptCompanyAccessServiceData> for InsertableCompanyAccessService {
    fn from(data_service: &IptCompanyAccessServiceData) -> Self {
        let IptCompanyAccessServiceData {
            service_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_service;

        Self {
            service_uuid: *service_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Data to request removal of company access to the service
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelCompanyAccessServiceData {
    /// UUID of the service
    pub(crate) service_uuid: Uuid,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
}
