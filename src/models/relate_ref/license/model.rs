use crate::schema::*;
use async_graphql::*;
use chrono::*;

/// Distribution license data
#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = license_ref)]
pub(crate) struct License {
    /// License ID
    pub(crate) id: i32,
    /// License name
    pub(crate) name: String,
    /// Abbreviation or abbreviation of the license
    pub(crate) keyword: String,
    /// Date of publication of the main text of the license
    pub(crate) publication_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = license_ref)]
pub(crate) struct InsertableLicense {
    pub(crate) name: String,
    pub(crate) keyword: String,
    pub(crate) publication_at: NaiveDateTime,
}

/// Data for the request to add a distribution license to the database
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct LicenseData {
    /// License name
    pub(crate) name: String,
    /// Abbreviation or abbreviation of the license
    pub(crate) keyword: String,
    /// Date of publication of the main text of the license
    pub(crate) publication_at: NaiveDateTime,
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
