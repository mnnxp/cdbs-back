use crate::schema::*;
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
