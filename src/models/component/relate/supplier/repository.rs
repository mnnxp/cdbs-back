use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::supplier::model::{SupplierComponent, ComponentSupplierRelatedData};
use crate::models::company::model::SlimCompany;
use crate::models::search::order::Paginate;
use crate::schema::supplier_to_component::dsl as supplier_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentSupplierRelatedData {
    /// Get suppliers list by component uuid
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let component_suppliers = supplier_to_component::supplier_to_component
            .filter(supplier_to_component::component_uuid.eq(component_uuid))
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<SupplierComponent>(conn)
            .map_err(|err| {
                debug!("Failed get supplier_component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut suppliers_list: Vec<ComponentSupplierRelatedData> = Vec::new();
        for supplier in component_suppliers.iter() {
            let mut data = ComponentSupplierRelatedData::new(supplier);
            data.put_supplier(SlimCompany::get_by_uuid(&supplier.company_uuid, conn)?);
            // debug!("get supplier: {:?}", data);
            suppliers_list.push(data);
        }

        Ok(suppliers_list)
    }
}
