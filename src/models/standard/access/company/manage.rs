use crate::auth::access::invalidate_access;
use crate::auth::AccessEntity;
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::standard::access::company::model::{
    CompanyAccessStandard, CompanyAccessStandardAndRelatedData, DelCompanyAccessStandardData,
    InsertableCompanyAccessStandard, IptCompanyAccessStandardData,
};
use crate::models::standard::access::util::check_is_owner_with_err;
use crate::schema::company_access_to_standard::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Возвращает список компаний, имеющих доступ к стандарту.
pub(crate) fn get_companies_list_access_standard(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyAccessStandardAndRelatedData>> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, target_standard_uuid, conn)?;

    // 2. получить список компаний с доступом к стандарту
    let list_companies_with_access = CompanyAccessStandardAndRelatedData::from_standard_by_uuid(
        target_standard_uuid,
        set_lang_id,
        conn,
    );

    match list_companies_with_access {
        Ok(res) => {
            debug!("Get companies have access: {:?}", res);
            Ok(res)
        }
        Err(err) => {
            debug!(
                "Failed get companies list have access to standard: {:?}",
                err
            );
            Err(get_err_msg(
                ErrorMessage::FailedGetCompaniesWithAccessStandard,
            ))
        }
    }
}

/// Устанавливает доступ к стандарту для компании.
/// Этот доступ распространяется на всех членов компании в соответствии с их ролями.
pub(crate) fn set_company_access_standard(
    logged_user_uuid: &Uuid,
    data: &IptCompanyAccessStandardData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    invalidate_access(
        logged_user_uuid,
        AccessEntity::Standard,
        &data.standard_uuid,
    );

    // 2. изменить или добавить доступ для указанной компании
    let set_access = diesel::update(
        company_access_to_standard.filter(
            standard_uuid
                .eq(&data.standard_uuid)
                .and(company_uuid.eq(&data.company_uuid)),
        ),
    )
    .set((
        type_access_id.eq(data.type_access_id),
        is_enabled.eq(true),
        updated_at.eq(chrono::Utc::now().naive_utc()),
    ))
    .execute(conn);

    match set_access {
        Ok(0) => {
            // доступ не найден, добавить новую запись
            if add_company_access_standard(data, conn)? {
                return Ok(true);
            }
            Err(get_err_msg(ErrorMessage::FailedSetAccessCompany))
        }
        Ok(x) => {
            debug!("Set access for target company: {:?}", x);
            Ok(true)
        }
        Err(err) => {
            debug!("Failed set access for target company: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedSetAccessCompany))
        }
    }
}

/// Add new access standard for company
/// Warning: this function without "check is owner company"
fn add_company_access_standard(
    data: &IptCompanyAccessStandardData,
    conn: &mut PgConnection,
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
        }
        Err(err) => {
            debug!("Failed add access for target company: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedAddAccess))
        }
    }
}

/// Удаляет доступ к стандарту для компании.
pub(crate) fn del_company_access_standard(
    logged_user_uuid: &Uuid,
    data: &DelCompanyAccessStandardData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. проверить пользователя на владение стандартом
    check_is_owner_with_err(logged_user_uuid, &data.standard_uuid, conn)?;

    // 2. деактивировать доступ для указанной компании
    let del_access = diesel::delete(company_access_to_standard)
        .filter(
            standard_uuid
                .eq(&data.standard_uuid)
                .and(company_uuid.eq(&data.company_uuid)),
        )
        .execute(conn);

    match del_access {
        Ok(0) => Err(get_err_msg(ErrorMessage::AccessNotFoundCompany)),
        Ok(x) => {
            debug!("Delete access for target company: {:?}", x);
            Ok(true)
        }
        Err(err) => {
            debug!("Failed delete access for target company: {:?}", err);
            Err(get_err_msg(ErrorMessage::FailedDeleteAccessForCompany))
        }
    }
}
