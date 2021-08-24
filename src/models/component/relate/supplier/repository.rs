use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::supplier::model::{SupplierComponent, ComponentSupplierRelatedData};
use crate::models::company::model::SlimCompany;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentSupplierRelatedData {
    pub fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let supplier_component: Vec<SupplierComponent> = SupplierComponent::belonging_to(component)
            .load::<SupplierComponent>(conn)
            .expect("Error loading supplier_component");

        let mut uuid_supplier_list: Vec<Uuid> = Vec::new();
        for supplier in supplier_component.iter() {
            uuid_supplier_list.push(supplier.uuid_company);
        }

        let slim_company_supplier: Vec<SlimCompany> = company_ref::company_ref
            .filter(company_ref::uuid.eq_any(uuid_supplier_list))
            .select((
                company_ref::uuid,
                company_ref::shortname,
                company_ref::is_supplier,
            ))
            .load::<SlimCompany>(conn)
            .expect("Error loading slim_company_supplier");

        let mut supplier_component_with_relate: Vec<ComponentSupplierRelatedData> = Vec::new();
        for x in supplier_component.iter() {
            for y in slim_company_supplier.iter() {
                if x.uuid_company == y.uuid {
                    let res: ComponentSupplierRelatedData = (x.clone(),y.clone()).into();
                    supplier_component_with_relate.push(res)
                }
            }
        }

        Ok(supplier_component_with_relate)
    }

    /// Get the first company associated with target component
    pub fn get_first_supplier(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentSupplierRelatedData> {
        let supplier_component: SupplierComponent = SupplierComponent::belonging_to(component)
            .first::<SupplierComponent>(conn)?;

        let slim_company_supplier: SlimCompany = company_ref::company_ref
            .filter(company_ref::uuid.eq(&supplier_component.uuid_company))
            .select((
                company_ref::uuid,
                company_ref::shortname,
                company_ref::is_supplier,
            ))
            .first::<SlimCompany>(conn)
            .expect("Error loading slim_company_supplier");
        Ok(ComponentSupplierRelatedData::from((
            supplier_component,
            slim_company_supplier
        )))
    }
}
