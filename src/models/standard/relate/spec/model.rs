use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::standard::model::Standard;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(standard_uuid, spec_id)]
#[belongs_to(Standard, foreign_key = "standard_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_standard"]
pub struct StandardSpec {
    pub spec_id: i32,
    pub standard_uuid: Uuid,
}

#[Object]
impl StandardSpec {
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }
    async fn standard_uuid(&self) -> ID {
        self.standard_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct StandardSpecWithTranslation {
    pub spec: SpecTranslateList,
    pub standard_uuid: Uuid,
}

impl From<(StandardSpec, SpecTranslateList)> for StandardSpecWithTranslation {
    fn from(data: (StandardSpec, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            standard_uuid: data.0.standard_uuid,
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_standard"]
pub struct InsertableStandardSpec {
    pub standard_uuid: Uuid,
    pub spec_id: i32,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptStandardSpecsData {
    pub standard_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

impl From<&IptStandardSpecsData> for Vec<InsertableStandardSpec> {
    fn from(ipt_data: &IptStandardSpecsData) -> Vec<InsertableStandardSpec> {
        let IptStandardSpecsData {
            standard_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each spec
        for spec_id in spec_ids {
            if spec_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableStandardSpec {
                    standard_uuid: standard_uuid.to_owned(),
                    spec_id: *spec_id,
                })
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub struct DeleteStandardSpecs {
    pub standard_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

impl From<&IptStandardSpecsData> for DeleteStandardSpecs {
    fn from(ipt_data: &IptStandardSpecsData) -> Self {
        let IptStandardSpecsData {
            standard_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut good_spec_ids: Vec<i32> = Vec::new();
        // filter bad specs id
        for spec_id in spec_ids {
            if spec_id > &0 {
                good_spec_ids.push(*spec_id)
            }
        }

        Self{
            standard_uuid: standard_uuid.to_owned(),
            spec_ids: good_spec_ids,
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptStandardSpecsArg {
    pub standard_uuid:  Uuid,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct StandardSpecsArg {
    pub standard_uuid:  Uuid,
    pub limit: i32,
    pub offset: i32,
}

impl From<IptStandardSpecsArg> for StandardSpecsArg {
    fn from(data: IptStandardSpecsArg) -> Self {
        let IptStandardSpecsArg {
            standard_uuid,
            limit,
            offset,
        } = data;

        Self {
            standard_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
