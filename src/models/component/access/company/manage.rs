use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::access::company::model::{
    CompanyAccessComponent,
    CompanyAccessComponentAndRelatedData,
    IptCompanyAccessComponentData,
    InsertableCompanyAccessComponent,
    DelCompanyAccessComponentData,
};
use crate::models::component::access::util::check_is_owner;
use crate::schema::company_access_to_component::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Get companies list have access to component
pub(crate) fn get_companies_list_access_component(
    logged_company_uuid: &Uuid,
    target_component_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_company_uuid, target_component_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. получить список пользователей с доступом к компоненту
    let list_companies_with_access = CompanyAccessComponentAndRelatedData::from_component_by_uuid(
        target_component_uuid,
        set_lang_id,
        conn
    );

    match list_companies_with_access {
        Ok(res) => {
            debug!("Get companies have access: {:?}", res);
            Ok(res)
        },
        Err(err) => {
            debug!("Failed get companies list have access to component: {:?}", err);
            Err(ServiceError::BadRequest(
                "Failed get companies list have access to component".to_string()
            ))
        },
    }
}

/// Manage component access for company
pub(crate) fn set_company_access_component(
    logged_company_uuid: &Uuid,
    data: &IptCompanyAccessComponentData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_company_uuid, &data.component_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. изменить или добавить доступ для указанного пользователя
    let set_access = diesel::update(company_access_to_component
        .filter(component_uuid.eq(&data.component_uuid)
        .and(company_uuid.eq(&data.company_uuid))))
        .set((
            type_access_id.eq(data.type_access_id),
            is_enabled.eq(true),
            updated_at.eq(chrono::Local::now().naive_local())
        )).execute(conn);

    match set_access {
        Ok(0) => {
            // доступ не найден, добавить новую запись
            if add_company_access_component(
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

/// Add new access component for company
/// Warning: this function without "check is owner company"
fn add_company_access_component(
    data: &IptCompanyAccessComponentData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableCompanyAccessComponent = data.into();

    let add_new_access: Result<CompanyAccessComponent, diesel::result::Error> =
        diesel::insert_into(company_access_to_component)
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

/// Remove access component for company
pub(crate) fn del_company_access_component(
    logged_company_uuid: &Uuid,
    data: &DelCompanyAccessComponentData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение компонентом
    if !check_is_owner(logged_company_uuid, &data.component_uuid, conn) {
        return Err(ServiceError::BadRequest("Access denied".to_string()))
    }

    // 2. деактивировать доступ для указанного пользователя
    let del_access = diesel::delete(company_access_to_component)
        .filter(component_uuid.eq(&data.component_uuid)
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
