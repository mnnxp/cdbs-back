use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::supplier_service::access::company::model::{
    CompanyAccessService, CompanyAccessServiceAndRelatedData,
};
use crate::schema::company_access_to_service::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

impl CompanyAccessServiceAndRelatedData {
    /// Collect related data for companies lits access service
    pub(crate) fn from_service_by_uuid(
        target_service_uuid: &Uuid,
        set_lang_id: i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<CompanyAccessServiceAndRelatedData>> {
        let list_companies_with_access = company_access_to_service
            .filter(service_uuid.eq(target_service_uuid))
            .load::<CompanyAccessService>(conn)
            .map_err(|err| {
                debug!("Failed get company_access_to_service: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut res: Vec<CompanyAccessServiceAndRelatedData> = Vec::new();
        for x in list_companies_with_access {
            let type_access = TypeAccessTranslateList::get_type_access_by_id(
                x.type_access_id,
                set_lang_id,
                conn,
            )?;

            res.push(CompanyAccessServiceAndRelatedData {
                service_uuid: x.service_uuid,
                company_uuid: x.company_uuid,
                permission: type_access.into(),
                is_enabled: x.is_enabled,
                created_at: x.created_at,
                updated_at: x.updated_at,
            });
        }

        Ok(res)
    }
}
