use crate::schema::*;
use crate::models::component::model::Component;
use crate::models::relate_ref::license::model::License;
use async_graphql::types::ID;
use async_graphql::*;
use uuid::Uuid;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Debug)]
#[primary_key(uuid_component, id_license)]
#[belongs_to(Component, foreign_key = "uuid_component")]
#[belongs_to(License, foreign_key = "id_license")]
#[table_name = "license_to_component"]
pub struct LicenseComponent {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

#[Object]
impl LicenseComponent {
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn id_license(&self) -> &i32 {
        &self.id_license
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptLicenseComponentData {
    pub uuid_component: ID,
    pub id_license: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "license_to_component"]
pub struct InsertableLicenseComponent {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

impl From<IptLicenseComponentData> for InsertableLicenseComponent {
    fn from(ipt_data: IptLicenseComponentData) -> Self {
        let IptLicenseComponentData {
            uuid_component,
            id_license,
        } = ipt_data;

        Self {
            uuid_component: Uuid::parse_str(&uuid_component.to_string()).unwrap(),
            id_license,
        }
    }
}
