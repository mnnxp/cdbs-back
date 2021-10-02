use crate::schema::*;
use crate::models::relate_ref::spec::model::{Spec, SpecTranslateList};
use crate::models::component::model::Component;
use async_graphql::types::ID;
use async_graphql::*;
// use chrono::*;
use uuid::Uuid;

// Spec component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[primary_key(component_uuid, spec_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_component"]
pub struct ComponentSpec {
    pub spec_id: i32,
    pub component_uuid: Uuid,
}

#[Object]
impl ComponentSpec {
    async fn spec_id(&self) -> &i32 {
        &self.spec_id
    }
    async fn component_uuid(&self) -> ID {
        self.component_uuid.into()
    }
}

#[derive(Deserialize, SimpleObject, Clone, Debug)]
pub struct ComponentSpecWithTranslation {
    pub spec: SpecTranslateList,
    pub component_uuid: Uuid,
}

impl From<(ComponentSpec, SpecTranslateList)> for ComponentSpecWithTranslation {
    fn from(data: (ComponentSpec, SpecTranslateList)) -> Self {
        Self {
            spec: data.1,
            component_uuid: data.0.component_uuid,
        }
    }
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentSpecData {
    pub component_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_component"]
pub struct InsertableComponentSpec {
    pub component_uuid: Uuid,
    pub spec_id: i32,
}

impl From<&IptComponentSpecData> for Vec<InsertableComponentSpec> {
    fn from(ipt_data: &IptComponentSpecData) -> Vec<InsertableComponentSpec> {
        let IptComponentSpecData {
            component_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for spec_id in spec_ids {
            if spec_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableComponentSpec {
                    component_uuid: component_uuid.to_owned(),
                    spec_id: *spec_id,
                })
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct DeleteComponentSpec {
    pub component_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

impl From<&IptComponentSpecData> for DeleteComponentSpec {
    fn from(ipt_data: &IptComponentSpecData) -> Self {
        let IptComponentSpecData {
            component_uuid,
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
            component_uuid: component_uuid.to_owned(),
            spec_ids: good_spec_ids,
        }
    }
}
