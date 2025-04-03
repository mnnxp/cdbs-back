use crate::schema::*;
use crate::models::relate_ref::spec::model::Spec;
use crate::models::supplier_service::model::Service;
use async_graphql::*;
use uuid::Uuid;

// Spec service models
#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations, Clone, Debug)]
#[diesel(primary_key(service_uuid, spec_id))]
#[diesel(belongs_to(Service, foreign_key = service_uuid))]
#[diesel(belongs_to(Spec, foreign_key = spec_id))]
#[diesel(table_name = spec_to_service)]
pub(crate) struct ServiceSpec {
    pub(crate) spec_id: i32,
    pub(crate) service_uuid: Uuid,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = spec_to_service)]
pub(crate) struct InsertableServiceSpec {
    pub(crate) service_uuid: Uuid,
    pub(crate) spec_id: i32,
}

/// Data for requesting to add/remove a service's association with catalogs
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptServiceSpecsData {
    /// Service UUID
    pub(crate) service_uuid: Uuid,
    /// Catalog identifiers (list)
    pub(crate) spec_ids: Vec<i32>,
}

impl From<&IptServiceSpecsData> for Vec<InsertableServiceSpec> {
    fn from(ipt_data: &IptServiceSpecsData) -> Vec<InsertableServiceSpec> {
        let IptServiceSpecsData {
            service_uuid,
            spec_ids,
            ..
        } = ipt_data;

        let mut res = Vec::new();
        // create struct for each spec
        for spec_id in spec_ids {
            if spec_id > &0 { // <-- additionally we check the correctness of the key
                res.push(InsertableServiceSpec {
                    service_uuid: *service_uuid,
                    spec_id: *spec_id,
                })
            }
        }

        res
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DeleteServiceSpecs {
    pub(crate) service_uuid: Uuid,
    pub(crate) spec_ids: Vec<i32>,
}

impl From<&IptServiceSpecsData> for DeleteServiceSpecs {
    fn from(ipt_data: &IptServiceSpecsData) -> Self {
        let IptServiceSpecsData {
            service_uuid,
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
            service_uuid: *service_uuid,
            spec_ids: good_spec_ids,
        }
    }
}