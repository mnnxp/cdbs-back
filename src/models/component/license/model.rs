use crate::schema::*;
use async_graphql::types::ID;
use async_graphql::*;
use chrono::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct License {
    pub id: i32,
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

#[Object]
impl License {
    async fn id(&self) -> i32 {
        self.id.into()
    }
    async fn name(&self) -> String {
        self.name.clone()
    }
    async fn keyword(&self) -> String {
        self.keyword.clone()
    }
    async fn description(&self) -> String {
        self.description.clone()
    }
    async fn publication_at(&self) -> NaiveDateTime {
        self.publication_at.into()
    }
}

#[derive(Debug, Insertable)]
#[table_name = "license_ref"]
pub struct InsertableLicense {
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct LicenseData {
    pub name: String,
    pub keyword: String,
    pub description: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimLicense {
    pub id: i32,
    pub keyword: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LicenseToComponent {
    pub id: i32,
    pub uuid_component: Uuid,
    pub id_license: i32,
}

#[Object]
impl LicenseToComponent {
    async fn id(&self) -> i32 {
        self.id.into()
    }
    async fn uuid_component(&self) -> ID {
        self.uuid_component.into()
    }
    async fn id_license(&self) -> i32 {
        self.id_license.into()
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct LicenseToComponentData {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "license_to_component"]
pub struct InsertableLicenseToComponent {
    pub uuid_component: Uuid,
    pub id_license: i32,
}

impl From<LicenseData> for InsertableLicense {
    fn from(data_license: LicenseData) -> Self {
        let LicenseData {
            name,
            keyword,
            description,
            publication_at,
            ..
        } = data_license;

        // let updated_at = chrono::Local::now().naive_local();

        Self {
            name,
            keyword,
            description,
            publication_at,
        }
    }
}

impl From<LicenseToComponentData> for InsertableLicenseToComponent {
    fn from(data_data_license_to_component: LicenseToComponentData) -> Self {
        let LicenseToComponentData {
            uuid_component,
            id_license,
            ..
        } = data_data_license_to_component;

        Self {
            uuid_component,
            id_license,
        }
    }
}
