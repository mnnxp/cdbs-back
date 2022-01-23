use async_graphql::InputObject;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanyOfSuppliersData {
    pub(crate) company_uuid: Uuid,
    pub(crate) component_uuid: Uuid,
}
