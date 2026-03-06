use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::search::model::ExtraOptions;
use crate::models::supplier_service::access::company::model::{
    CompanyAccessService, CompanyAccessServiceAndRelatedData, DelCompanyAccessServiceData,
    InsertableCompanyAccessService, IptCompanyAccessServiceData,
};
use crate::models::supplier_service::access::util::check_is_owner_with_err;
use crate::schema::company_access_to_service::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns the list of companies that have access to the service
pub(crate) fn get_companies_list_access_service(
    target_service_uuid: &Uuid,
    options: &ExtraOptions,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<CompanyAccessServiceAndRelatedData>> {
    // 1. check the user for service ownership
    check_is_owner_with_err(&options.logged_user_uuid, target_service_uuid, conn)?;

    // 2. get a list of companies with access to the service
    let list_companies_with_access = CompanyAccessServiceAndRelatedData::from_service_by_uuid(
        target_service_uuid,
        options.set_lang_id,
        conn,
    );

    match list_companies_with_access {
        Ok(res) => {
            debug!("Get companies have access: {:?}", res);
            Ok(res)
        }
        Err(err) => {
            debug!(
                "Failed get companies list have access to service: {:?}",
                err
            );
            Err(get_err_msg(
                ErrorMessage::FailedGetCompaniesWithAccessService,
            ))
        }
    }
}

/// Sets the access to the service for the company.
/// This access applies to all members of the company according to their roles.
pub(crate) fn set_company_access_service(
    data: &IptCompanyAccessServiceData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. check the user for service ownership
    check_is_owner_with_err(logged_user_uuid, &data.service_uuid, conn)?;

    // 2. change or add access for the specified company
    let set_access = diesel::update(
        company_access_to_service.filter(
            service_uuid
                .eq(&data.service_uuid)
                .and(company_uuid.eq(&data.company_uuid)),
        ),
    )
    .set((
        type_access_id.eq(data.type_access_id),
        is_enabled.eq(true),
        updated_at.eq(chrono::Local::now().naive_local()),
    ))
    .execute(conn);

    match set_access {
        Ok(0) => {
            // access not found, add a new entry
            if add_company_access_service(data, conn)? {
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

/// Add new access service for company
/// Warning: this function without "check is owner company"
fn add_company_access_service(
    data: &IptCompanyAccessServiceData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let insert_data: InsertableCompanyAccessService = data.into();

    let add_new_access: Result<CompanyAccessService, diesel::result::Error> =
        diesel::insert_into(company_access_to_service)
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

/// Disable the access service for the company
pub(crate) fn del_company_access_service(
    data: &DelCompanyAccessServiceData,
    logged_user_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // 1. verify the user's ownership of the service
    check_is_owner_with_err(logged_user_uuid, &data.service_uuid, conn)?;

    // 2. deactivate access for the specified company
    let del_access = diesel::delete(company_access_to_service)
        .filter(
            service_uuid
                .eq(&data.service_uuid)
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
