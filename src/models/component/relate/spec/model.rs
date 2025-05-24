use crate::schema::*;
use async_graphql::*;
use uuid::Uuid;

// Spec component models
#[derive(Serialize, Deserialize, Queryable, SimpleObject, Clone, Debug)]
#[diesel(table_name = spec_to_component)]
pub(crate) struct ComponentSpec {
    pub(crate) spec_id: i32,
    pub(crate) component_uuid: Uuid,
}

/// Data for requests to add and remove directory links to the component
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptComponentSpecsData {
    /// Component UUID
    pub(crate) component_uuid: Uuid,
    /// Catalog identifiers (list)
    pub(crate) spec_ids: Vec<i32>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_to_component)]
pub(crate) struct InsertableComponentSpec {
    pub(crate) component_uuid: Uuid,
    pub(crate) spec_id: i32,
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
pub(crate) struct DeleteComponentSpecs {
    pub(crate) component_uuid: Uuid,
    pub(crate) spec_ids: Vec<i32>,
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