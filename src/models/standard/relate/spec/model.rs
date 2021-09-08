use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::standard::model::Standard;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec standard models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(uuid_standard, id_spec)]
#[belongs_to(Standard, foreign_key = "uuid_standard")]
#[belongs_to(Spec, foreign_key = "id_spec")]
#[table_name = "spec_to_standard"]
pub struct SpecStandard {
    pub id_spec: i32,
    pub uuid_standard: Uuid,
}

#[Object]
impl SpecStandard {
    async fn id_spec(&self) -> &i32 {
        &self.id_spec
    }
    async fn uuid_standard(&self) -> ID {
        self.uuid_standard.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct StandardSpecWithTranslation {
    pub spec: SpecTranslateList,
    pub uuid_standard: Uuid,
}

impl From<(SpecStandard, SpecTranslateList)> for StandardSpecWithTranslation {
    fn from(data: (SpecStandard, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            uuid_standard: data.0.uuid_standard,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptSpecStandardData {
    pub uuid_standard: ID,
    pub id_spec: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_standard"]
pub struct InsertableSpecStandard {
    pub uuid_standard: Uuid,
    pub id_spec: i32,
}

impl From<IptSpecStandardData> for InsertableSpecStandard {
    fn from(ipt_data: IptSpecStandardData) -> Self {
        let IptSpecStandardData {
            uuid_standard,
            id_spec,
            ..
        } = ipt_data;

        Self {
            uuid_standard: Uuid::parse_str(&uuid_standard.to_string()).unwrap(),
            id_spec,
        }
    }
}
