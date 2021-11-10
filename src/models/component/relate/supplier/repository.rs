use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::supplier::model::{SupplierComponent, ComponentSupplierRelatedData};
use crate::models::company::model::SlimCompany;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl ComponentSupplierRelatedData {
    pub(crate) fn for_component(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let supplier_component: Vec<SupplierComponent> = SupplierComponent::belonging_to(component)
            .load::<SupplierComponent>(conn)
            .expect("Error loading supplier_component");

        let mut uuid_supplier_list: Vec<Uuid> = Vec::new();
        for supplier in supplier_component.iter() {
            uuid_supplier_list.push(supplier.company_uuid);
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
                if x.company_uuid == y.uuid {
                    let res: ComponentSupplierRelatedData = (x.clone(),y.clone()).into();
                    supplier_component_with_relate.push(res)
                }
            }
        }

        Ok(supplier_component_with_relate)
    }

    /// Get the first company associated with target component
    pub(crate) fn get_first_supplier(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        let find_supplier_component: Result<SupplierComponent, _> = SupplierComponent::belonging_to(component)
            .first(conn);

        match find_supplier_component {
            Ok(supplier_component) => {
                let slim_company_supplier: SlimCompany = company_ref::company_ref
                    .filter(company_ref::uuid.eq(&supplier_component.company_uuid))
                    .select((
                        company_ref::uuid,
                        company_ref::shortname,
                        company_ref::is_supplier,
                    ))
                    .first::<SlimCompany>(conn)
                    .expect("Error loading slim_company_supplier");

                match slim_company_supplier.shortname.is_empty() {
                    false => Ok(vec![ComponentSupplierRelatedData::from((
                        supplier_component,
                        slim_company_supplier
                    ))]),
                    _ => Ok(Vec::new()),
                }
            },
            Err(e) => {
                debug!("Error loading supplier_component: {:#?}", e.to_string());
                Ok(Vec::new())
            },
        }


    }
}
