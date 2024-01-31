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

/// Abbreviated data about the distribution license
#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub(crate) struct SlimLicense {
    /// License Identifier
    pub(crate) id: i32,
    /// License abbreviation or acronym
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

/// Arguments for querying existing distribution licenses on the platform
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptLicenseArg {
    /// Filter on license identifiers
    pub(crate) license_ids: Option<Vec<i32>>,
    /// Restriction of data sampling (maximum number of records)
    pub(crate) limit: Option<i32>,
    /// Number of skipping records at the beginning (offset)
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
