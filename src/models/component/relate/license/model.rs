use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::license::model::License;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, SimpleObject, Debug)]
#[primary_key(component_uuid, license_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(License, foreign_key = "license_id")]
#[table_name = "license_to_component"]
pub(crate) struct ComponentLicense {
    pub(crate) component_uuid: Uuid,
    pub(crate) license_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentLicenseData {
    pub(crate) component_uuid: Uuid,
    pub(crate) license_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "license_to_component"]
pub(crate) struct InsertableComponentLicense {
    pub(crate) component_uuid: Uuid,
    pub(crate) license_id: i32,
}

impl From<&IptComponentLicenseData> for InsertableComponentLicense {
    fn from(ipt_data: &IptComponentLicenseData) -> Self {
        let IptComponentLicenseData {
            component_uuid,
            license_id,
        } = ipt_data;

        Self {
            component_uuid: *component_uuid,
            license_id: *license_id,
        }
    }
}
