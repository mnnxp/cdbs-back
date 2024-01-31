use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::model::IptUpdateCompanyData;
use crate::models::company::access::util::check_is_owner_with_err;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет основные данные компании. Возвращает количество успешных изменений.
pub(crate) fn update_company_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    data: &IptUpdateCompanyData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    // check access user for company
    check_is_owner_with_err(
        logged_user_uuid,
        target_company_uuid,
        conn
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column orgname
    if let Some(value) = &data.orgname {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::orgname.ne(value))))
            .set(company_ref::orgname.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column shortname
    if let Some(value) = &data.shortname {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::shortname.ne(value))))
            .set(company_ref::shortname.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column inn
    if let Some(value) = &data.inn {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::inn.ne(value))))
            .set(company_ref::inn.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column phone
    if let Some(value) = &data.phone {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::phone.ne(value))))
            .set(company_ref::phone.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column email
    if let Some(value) = &data.email {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::email.ne(value))))
            .set(company_ref::email.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::description.ne(value))))
            .set(company_ref::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column address
    if let Some(value) = &data.address {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::address.ne(value))))
            .set(company_ref::address.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column site_url
    if let Some(value) = &data.site_url {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::site_url.ne(value))))
            .set(company_ref::site_url.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column time_zone
    if let Some(value) = &data.time_zone {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::time_zone.ne(value))))
            .set(company_ref::time_zone.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::region_id.ne(value))))
            .set(company_ref::region_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column company_type_id
    if let Some(value) = &data.company_type_id {
        count_update_columns += diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::company_type_id.ne(value))))
            .set(company_ref::company_type_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(ServiceError::BadRequest("The data has already".to_string()))
    }

    diesel::update(company_ref::company_ref
        .filter(company_ref::uuid.eq(target_company_uuid)))
        .set(company_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            ServiceError::BadRequest("Failed update data".to_string())
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
