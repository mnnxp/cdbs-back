use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::service::delete::delete_company;
use crate::models::component::service::delete::delete_component;
use crate::models::relate_ref::file::service::delete::delete_file_by_uuid;
use crate::models::standard::service::delete::delete_standard;
use crate::models::user::access::password::check_password;
use crate::models::user::relate::certificate::service::delete::delete_user_certificates;
use crate::schema::{
    company_ref::dsl as company_ref, component_ref::dsl as component_ref,
    standard_ref::dsl as standard_ref, user_ref::dsl as user_ref,
};
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет пользователя и связанные с ним данные.
pub(crate) fn delete_user(
    logged_user_uuid: &Uuid,
    user_password: &[u8],
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // compare password with password in database
    check_password(logged_user_uuid, user_password, conn)?;

    // delete user certificates and set flags for certificates files
    delete_user_certificates(logged_user_uuid, conn)?;

    // delete user components and set flags for files
    delete_user_components(logged_user_uuid, conn)?;

    // delete user companies and set flags for files
    delete_user_companies(logged_user_uuid, conn)?;

    // delete user standards and set flags for files
    delete_user_standards(logged_user_uuid, conn)?;

    let image_file_uuid = diesel::update(user_ref::user_ref)
        .filter(user_ref::uuid.eq(logged_user_uuid))
        .set((user_ref::is_enabled.eq(false), user_ref::is_delete.eq(true)))
        .returning(user_ref::image_file_uuid)
        .get_result::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed delete user data: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuid(&image_file_uuid, conn)?;

    Ok(true)
}

/// Delete all components ownership user
fn delete_user_components(user_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let component_uuids = component_ref::component_ref
        .filter(component_ref::user_uuid.eq(user_uuid))
        .select(component_ref::uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get components by user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    for del_component_uuid in &component_uuids {
        delete_component(del_component_uuid, conn)?;
    }

    Ok(true)
}

/// Delete all companies ownership user
fn delete_user_companies(user_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let company_uuids = company_ref::company_ref
        .filter(company_ref::user_uuid.eq(user_uuid))
        .select(company_ref::uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get companies by user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    for del_company_uuid in &company_uuids {
        delete_company(del_company_uuid, conn)?;
    }

    Ok(true)
}

/// Delete all standards ownership user
fn delete_user_standards(user_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let standard_uuids = standard_ref::standard_ref
        .filter(standard_ref::user_uuid.eq(user_uuid))
        .select(standard_ref::uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get standards by user: {:?}", err);
            ServiceError::InternalServerError
        })?;

    for del_standard_uuid in &standard_uuids {
        delete_standard(del_standard_uuid, conn)?;
    }

    Ok(true)
}
