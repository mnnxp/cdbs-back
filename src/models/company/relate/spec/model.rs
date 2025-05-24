use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

/// Related catalog and company identifiers
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = spec_to_company)]
pub(crate) struct CompanySpec {
    /// Catalog Identifier
    pub(crate) spec_id: i32,
    /// Company UUID
    pub(crate) company_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_to_company)]
pub(crate) struct InsertableCompanySpec {
    pub(crate) company_uuid: Uuid,
    pub(crate) spec_id: i32,
}

/// Data for linking catalogs to the company
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptCompanySpecsData {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Catalog identifiers (list)
    pub(crate) spec_ids: Vec<i32>,
}

impl From<&IptCompanySpecsData> for Vec<InsertableCompanySpec> {
    fn from(ipt_data: &IptCompanySpecsData) -> Vec<InsertableCompanySpec> {
        let IptCompanySpecsData {
            company_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut res: Vec<InsertableCompanySpec> = Vec::new();
        // create struct for each spec
        for sc_id in spec_ids {
            if sc_id > &0 {
                res.push(InsertableCompanySpec {
                    company_uuid: *company_uuid,
                    spec_id: *sc_id,
                })
            }
        }

        res
    }
}

/// Data for removing the company's association with the specified catalogs
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct DelCompanySpec {
    /// Company UUID
    pub(crate) company_uuid: Uuid,
    /// Catalog identifiers (list)
    pub(crate) spec_ids: Vec<i32>,
}

impl From<&IptCompanySpecsData> for DelCompanySpec {
    fn from(ipt_data: &IptCompanySpecsData) -> Self {
        let IptCompanySpecsData {
            company_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut good_spec_ids: Vec<i32> = Vec::new();
        // filter bad keyword id
        for spec_id in spec_ids {
            if spec_id > &0 {
                good_spec_ids.push(*spec_id)
            }
        }

        Self{
            company_uuid: *company_uuid,
            spec_ids: good_spec_ids,
        }
    }
}