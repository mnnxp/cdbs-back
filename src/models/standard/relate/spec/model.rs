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
pub struct SpecStandard {
    pub spec_id: i32,
    pub standard_uuid: Uuid,
}

#[Object]
impl SpecStandard {
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

impl From<(SpecStandard, SpecTranslateList)> for StandardSpecWithTranslation {
    fn from(data: (SpecStandard, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            standard_uuid: data.0.standard_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecStandardData {
    pub standard_uuid: ID,
    pub spec_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_standard"]
pub struct InsertableSpecStandard {
    pub standard_uuid: Uuid,
    pub spec_id: i32,
}

impl From<IptSpecStandardData> for InsertableSpecStandard {
    fn from(ipt_data: IptSpecStandardData) -> Self {
        let IptSpecStandardData {
            standard_uuid,
            spec_id,
            ..
        } = ipt_data;

        Self {
            standard_uuid: Uuid::parse_str(&standard_uuid.to_string()).unwrap(),
            spec_id,
        }
    }
}
