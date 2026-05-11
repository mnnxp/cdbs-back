use crate::auth::permission::PermissionTranslateList;
use crate::schema::*;
use async_graphql::InputObject;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Deserialize, Queryable, Debug)]
#[diesel(primary_key(component_uuid, company_uuid))]
#[diesel(table_name = company_access_to_component)]
pub(crate) struct CompanyAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Company access data to the component (part) with additional information
#[derive(Debug, Deserialize)]
pub(crate) struct CompanyAccessComponentAndRelatedData {
    /// UUID of the component
    pub(crate) component_uuid: Uuid,
    /// UUID of the company
    pub(crate) company_uuid: Uuid,
    /// Access level with localization
    pub(crate) permission: PermissionTranslateList,
    /// Access activity flag
    pub(crate) is_enabled: bool,
    /// Date of first access assignment
    pub(crate) created_at: NaiveDateTime,
    /// Date of access modification
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = company_access_to_component)]
pub(crate) struct InsertableCompanyAccessComponent {
    pub(crate) component_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) type_access_id: i32,
    pub(crate) is_enabled: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

/// Data for requesting to create or change company access to the component
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct IptCompanyAccessComponentData {
    /// Identifier (UUID) of the component (part)
    pub(crate) component_uuid: Uuid,
    /// Identifier (UUID) of the company
    pub(crate) company_uuid: Uuid,
    /// Identifier of the type (level) of access
    pub(crate) type_access_id: i32,
}

impl From<&IptCompanyAccessComponentData> for InsertableCompanyAccessComponent {
    fn from(data_component: &IptCompanyAccessComponentData) -> Self {
        let IptCompanyAccessComponentData {
            component_uuid,
            company_uuid,
            type_access_id,
            ..
        } = data_component;

        Self {
            component_uuid: *component_uuid,
            company_uuid: *company_uuid,
            type_access_id: *type_access_id,
            is_enabled: true,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

/// Data for requesting deletion (deactivation) of company members' access to the component (part)
#[derive(Debug, Deserialize, InputObject)]
pub(crate) struct DelCompanyAccessComponentData {
    /// Identifier (UUID) of the component (part)
    pub(crate) component_uuid: Uuid,
    /// Identifier (UUID) of the company
    pub(crate) company_uuid: Uuid,
}
