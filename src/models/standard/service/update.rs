use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::model::IptUpdateStandardData;
use crate::models::company::{
    access::util::check_company_access,
    util::check_is_supplier,
};
use crate::models::standard::access::util::check_access_standard_for_user;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Update main standard data
pub(crate) fn update_standard_data(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    data: &IptUpdateStandardData,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_access_standard_for_user(
        logged_user_uuid,
        target_standard_uuid,
        &need_access_level,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column company_uuid
    if let Some(value) = &data.company_uuid {
        check_company_access(
            logged_user_uuid,
            value,
            &need_access_level,
            conn,
        )?;

        check_is_supplier(
            value,
            conn
        )?;

        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::company_uuid.ne(value))))
            .set(standard_ref::company_uuid.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column classifier
    if let Some(value) = &data.classifier {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::classifier.ne(value))))
            .set(standard_ref::classifier.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column name
    if let Some(value) = &data.name {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::name.ne(value))))
            .set(standard_ref::name.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::description.ne(value))))
            .set(standard_ref::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column specified_tolerance
    if let Some(value) = &data.specified_tolerance {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::specified_tolerance.ne(value))))
            .set(standard_ref::specified_tolerance.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column technical_committee
    if let Some(value) = &data.technical_committee {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::technical_committee.ne(value))))
            .set(standard_ref::technical_committee.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column publication_at
    if let Some(value) = &data.publication_at {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::publication_at.ne(value))))
            .set(standard_ref::publication_at.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column standard_status_id
    if let Some(value) = &data.standard_status_id {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::standard_status_id.ne(value))))
            .set(standard_ref::standard_status_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::region_id.ne(value))))
            .set(standard_ref::region_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                ServiceError::BadRequest("Failed update data".to_string())
            })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(ServiceError::BadRequest("The data has already".to_string()));
    }

    diesel::update(standard_ref::standard_ref
        .filter(standard_ref::uuid.eq(target_standard_uuid)))
        .set(standard_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            ServiceError::BadRequest("Failed update data".to_string())
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
