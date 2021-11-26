use crate::schema::*;
use async_graphql::*;
use chrono::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Debug)]
#[primary_key(id)]
#[table_name = "license_ref"]
pub struct License {
    pub id: i32,
    pub name: String,
    pub keyword: String,
    pub publication_at: NaiveDateTime,
}

#[Object]
impl License {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn name(&self) -> &String {
        &self.name
    }
    async fn keyword(&self) -> &String {
        &self.keyword
    }
    async fn publication_at(&self) -> &NaiveDateTime {
        &self.publication_at
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlimLicense {
    pub id: i32,
    pub keyword: String,
}

#[Object]
impl SlimLicense {
    async fn id(&self) -> &i32 {
        &self.id
    }
    async fn keyword(&self) -> &String {
        &self.keyword
    }
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
