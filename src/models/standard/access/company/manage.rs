use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::access::company::model::{
    CompanyAccessStandard,
    CompanyAccessStandardAndRelatedData,
    IptCompanyAccessStandardData,
    InsertableCompanyAccessStandard,
    DelCompanyAccessStandardData,
};
use crate::models::standard::access::util::check_is_owner_with_err;
use crate::schema::company_access_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Get companies list have access to standard
pub(crate) fn get_companies_list_access_standard(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<CompanyAccessStandardAndRelatedData>> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, target_standard_uuid, conn)?;

    // 2. получить список компаний с доступом к стандарту
    let list_companies_with_access = CompanyAccessStandardAndRelatedData::from_standard_by_uuid(
        target_standard_uuid,
        set_lang_id,
        conn
    );

    match list_companies_with_access {
        Ok(res) => {
            debug!("Get companies have access: {:?}", res);
            Ok(res)
        },
        Err(err) => {
            debug!("Failed get companies list have access to standard: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed get companies list have access to standard".to_string()
            ))
        },
    }
}

/// Manage standard access for company
pub(crate) fn set_company_access_standard(
    logged_user_uuid: &Uuid,
    data: &IptCompanyAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. изменить или добавить доступ для указанной компании
    let set_access = diesel::update(company_access_to_standard
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(company_uuid.eq(&data.company_uuid))))
        .set((
            type_access_id.eq(data.type_access_id),
            is_enabled.eq(true),
            updated_at.eq(chrono::Local::now().naive_local())
        )).execute(conn);

    match set_access {
        Ok(0) => {
            // доступ не найден, добавить новую запись
            if add_company_access_standard(
                data,
                conn
            )? { return Ok(true) }

            Err(ServiceError::BadRequest(
                "Failed set access for target company".to_string()
            ))
        },
        Ok(x) => {
            debug!("Set access for target company: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed set access for target company: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed set access for target company".to_string()
            ))
        },
    }
}

/// Add new access standard for company
/// Warning: this function without "check is owner company"
fn add_company_access_standard(
    data: &IptCompanyAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableCompanyAccessStandard = data.into();

    let add_new_access: Result<CompanyAccessStandard, diesel::result::Error> =
        diesel::insert_into(company_access_to_standard)
            .values(insert_data)
            .get_result(conn);

    match add_new_access {
        Ok(x) => {
            debug!("Completed add new access for target company: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed add access for target company: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed add access for target company".to_string()
            ))
        },
    }
}

/// Remove access standard for company
pub(crate) fn del_company_access_standard(
    logged_user_uuid: &Uuid,
    data: &DelCompanyAccessStandardData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. деактивировать доступ для указанной компании
    let del_access = diesel::delete(company_access_to_standard)
        .filter(standard_uuid.eq(&data.standard_uuid)
        .and(company_uuid.eq(&data.company_uuid)))
        .execute(conn);

    match del_access {
        Ok(0) => {
            // доступ не найден
            Err(ServiceError::BadRequest(
                "Access not found for company".to_string()
            ))
        },
        Ok(x) => {
            debug!("Delete access for target company: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed delete access for target company: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed delete access for target company".to_string()
            ))
        },
    }
}
