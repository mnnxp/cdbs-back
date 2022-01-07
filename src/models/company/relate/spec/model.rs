use crate::schema::*;
use crate::models::relate_ref::spec::model::Spec;
use crate::models::company::model::Company;
use async_graphql::*;
use uuid::Uuid;

// Spec company models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug, SimpleObject)]
#[primary_key(company_uuid, spec_id)]
#[belongs_to(Company, foreign_key = "company_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_company"]
pub struct CompanySpec {
    pub spec_id: i32,
    pub company_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_company"]
pub struct InsertableCompanySpec {
    pub company_uuid: Uuid,
    pub spec_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptCompanySpecsData {
    pub company_uuid: Uuid,
    pub spec_ids: Vec<i32>,
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

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct DelCompanySpec {
    pub company_uuid: Uuid,
    pub spec_ids: Vec<i32>,
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

#[derive(InputObject, Deserialize, Debug)]
pub struct IptCompanySpecsArg {
    pub company_uuid:  Uuid,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct CompanySpecsArg {
    pub company_uuid:  Uuid,
    pub limit: i32,
    pub offset: i32,
}

impl From<IptCompanySpecsArg> for CompanySpecsArg {
    fn from(data: IptCompanySpecsArg) -> Self {
        let IptCompanySpecsArg {
            company_uuid,
            limit,
            offset,
        } = data;

        Self {
            company_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
