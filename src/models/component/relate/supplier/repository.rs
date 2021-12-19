use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::supplier::model::{SupplierComponent, ComponentSupplierRelatedData};
use crate::models::company::model::SlimCompany;
use crate::schema::supplier_to_component::dsl as supplier_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentSupplierRelatedData {
    /// Get suppliers list by component uuid
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let component_suppliers = supplier_to_component::supplier_to_component
            .filter(supplier_to_component::component_uuid.eq(component_uuid))
            .load::<SupplierComponent>(conn)
            .map_err(|err| {
                debug!("Failed get supplier_component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut suppliers_list: Vec<ComponentSupplierRelatedData> = Vec::new();
        for x in component_suppliers.iter() {
            let mut data = ComponentSupplierRelatedData::new(x);
            data.put_supplier(SlimCompany::get_by_uuid(&x.company_uuid, conn)?);
            // debug!("get supplier: {:?}", data);
            suppliers_list.push(data);
        }

        Ok(suppliers_list)
    }

    /// Get the first company associated with target component
    pub(crate) fn get_first_supplier(
        component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let supplier_component = supplier_to_component::supplier_to_component
            .filter(supplier_to_component::component_uuid.eq(component_uuid))
            .limit(1)
            .load::<SupplierComponent>(conn)
            .map_err(|err| {
                debug!("Failed get supplier_component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match supplier_component.first() {
            Some(x) => {
                let mut data = ComponentSupplierRelatedData::new(x);
                data.put_supplier(SlimCompany::get_by_uuid(&x.company_uuid, conn)?);
                // debug!("get first supplier: {:?}", data);
                Ok(vec![data])
            },
            None => Ok(Vec::new()),
        }
    }
}
