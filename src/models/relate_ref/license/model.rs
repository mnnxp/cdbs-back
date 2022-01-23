use crate::schema::*;
use async_graphql::*;
use chrono::*;

#[derive(Identifiable, Serialize, Deserialize, Queryable, SimpleObject, Debug)]
#[primary_key(id)]
#[table_name = "license_ref"]
pub(crate) struct License {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) keyword: String,
    pub(crate) publication_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "license_ref"]
pub(crate) struct InsertableLicense {
    pub(crate) name: String,
    pub(crate) keyword: String,
    pub(crate) publication_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct LicenseData {
    pub(crate) name: String,
    pub(crate) keyword: String,
    pub(crate) publication_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub(crate) struct SlimLicense {
    pub(crate) id: i32,
    pub(crate) keyword: String,
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

#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptLicenseArg {
    pub(crate) license_ids:  Option<Vec<i32>>,
    pub(crate) limit: Option<i32>,
    pub(crate) offset: Option<i32>,
}

#[derive(Debug)]
pub(crate) struct LicenseArg {
    pub(crate) license_ids: Vec<i32>,
    pub(crate) limit: i32,
    pub(crate) offset: i32,
}

impl Default for LicenseArg {
    fn default() -> Self {
        Self {
            license_ids: Vec::new(),
            limit: 100,
            offset: 0,
        }
    }
}

impl From<IptLicenseArg> for LicenseArg {
    fn from(data: IptLicenseArg) -> Self {
        let IptLicenseArg {
            license_ids,
            limit,
            offset,
        } = data;

        Self {
            license_ids: license_ids.unwrap_or_default(),
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
