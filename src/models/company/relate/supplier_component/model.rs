use async_graphql::InputObject;
use uuid::Uuid;

/// Data for a request to remove a company from the list of component suppliers (details)
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyOfSuppliersData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// UUID of the component (part)
    pub(crate) component_uuid: Uuid,
}
