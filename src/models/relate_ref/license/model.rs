use crate::schema::*;
use async_graphql::*;
use chrono::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[primary_key(id)]
#[table_name = "license_ref"]
pub struct License {
    pub id: i32,
    pub name: String,
    pub keyword: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "license_ref"]
pub struct InsertableLicense {
    pub name: String,
    pub keyword: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct LicenseData {
    pub name: String,
    pub keyword: String,
    pub publication_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct SlimLicense {
    pub id: i32,
    pub keyword: String,
}

impl From<&LicenseData> for InsertableLicense {
    fn from(data: &LicenseData) -> Self {
        Self {
            name: data.name.clone(),
            keyword: data.keyword.clone(),
            publication_at: data.publication_at,
        }
    }
}
