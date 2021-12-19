use crate::schema::*;
use crate::models::relate_ref::spec::model::Spec;
use crate::models::component::model::Component;
use async_graphql::*;
use uuid::Uuid;

// Spec component models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug, SimpleObject)]
#[primary_key(component_uuid, spec_id)]
#[belongs_to(Component, foreign_key = "component_uuid")]
#[belongs_to(Spec, foreign_key = "spec_id")]
#[table_name = "spec_to_component"]
pub struct ComponentSpec {
    pub spec_id: i32,
    pub component_uuid: Uuid,
}

#[derive(Debug, Deserialize, Clone, InputObject)]
pub struct IptComponentSpecsData {
    pub component_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "spec_to_component"]
pub struct InsertableComponentSpec {
    pub component_uuid: Uuid,
    pub spec_id: i32,
}

impl From<&IptComponentSpecsData> for Vec<InsertableComponentSpec> {
    fn from(ipt_data: &IptComponentSpecsData) -> Vec<InsertableComponentSpec> {
        let IptComponentSpecsData {
            component_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each keyword
        for spec_id in spec_ids {
            if spec_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableComponentSpec {
                    component_uuid: *component_uuid,
                    spec_id: *spec_id,
                })
            }
        }
        res
    }
}

#[derive(Debug, Clone)]
pub struct DeleteComponentSpecs {
    pub component_uuid: Uuid,
    pub spec_ids: Vec<i32>,
}

impl From<&IptComponentSpecsData> for DeleteComponentSpecs {
    fn from(ipt_data: &IptComponentSpecsData) -> Self {
        let IptComponentSpecsData {
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
            component_uuid: *component_uuid,
            spec_ids: good_spec_ids,
        }
    }
}

#[derive(InputObject, Deserialize, Debug)]
pub struct IptComponentSpecsArg {
    pub component_uuid:  Uuid,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug)]
pub struct ComponentSpecsArg {
    pub component_uuid:  Uuid,
    pub limit: i32,
    pub offset: i32,
}

impl From<IptComponentSpecsArg> for ComponentSpecsArg {
    fn from(data: IptComponentSpecsArg) -> Self {
        let IptComponentSpecsArg {
            component_uuid,
            limit,
            offset,
        } = data;

        Self {
            component_uuid,
            limit: limit.unwrap_or(100),
            offset: offset.unwrap_or(0),
        }
    }
}
