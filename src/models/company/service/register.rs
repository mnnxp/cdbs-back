use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::model::{
    IptCompanyData, InsertableCompany,
};
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Создает компанию, возвращает UUID новой компании.
pub(crate) fn create_company(
    logged_user_uuid: &Uuid,
    data: &IptCompanyData,
    conn: &mut PgConnection
) -> ServiceResult<Uuid> {
    let mut insert_data: InsertableCompany = data.into();

    // set logged user as owner company
    insert_data.set_user_uuid(logged_user_uuid);
    // set default company favicon
    insert_data.set_image_uuid();

    diesel::insert_into(company_ref::company_ref)
        .values(&insert_data)
        .returning(company_ref::uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed insert company data: {:?}", err);
            ServiceError::InternalServerError
        })
}
