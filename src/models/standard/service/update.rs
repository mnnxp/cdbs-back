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
    conn: &PgConnection
) -> ServiceResult<i32> {
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

        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::company_uuid.ne(value))))
            .set(standard_ref::company_uuid.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column classifier
    if let Some(value) = &data.classifier {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::classifier.ne(value))))
            .set(standard_ref::classifier.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column name
    if let Some(value) = &data.name {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::name.ne(value))))
            .set(standard_ref::name.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column description
    if let Some(value) = &data.description {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::description.ne(value))))
            .set(standard_ref::description.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column specified_tolerance
    if let Some(value) = &data.specified_tolerance {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::specified_tolerance.ne(value))))
            .set(standard_ref::specified_tolerance.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column technical_committee
    if let Some(value) = &data.technical_committee {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::technical_committee.ne(value))))
            .set(standard_ref::technical_committee.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column publication_at
    if let Some(value) = &data.publication_at {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::publication_at.ne(value))))
            .set(standard_ref::publication_at.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column type_access_id
    if let Some(value) = &data.type_access_id {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::type_access_id.ne(value))))
            .set(standard_ref::type_access_id.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column standard_status_id
    if let Some(value) = &data.standard_status_id {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::standard_status_id.ne(value))))
            .set(standard_ref::standard_status_id.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)
            .and(standard_ref::region_id.ne(value))))
            .set(standard_ref::region_id.eq(value))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }
    }

    // new date for updated_at in standard_ref table if update more one column
    if count_update_columns > 0 {
        let res = diesel::update(standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(target_standard_uuid)))
            .set(standard_ref::updated_at.eq(chrono::Local::now().naive_local()))
            .execute(conn);

        match res {
            Ok(up_item) => count_update_columns += up_item,
            Err(err) => {
                debug!("Failed update data: {:?}", err);

                return Err(ServiceError::BadRequest(
                    "Failed update data".to_string()
                ))
            },
        }

        debug!("Count update columns: {:?}", count_update_columns);

        return Ok(count_update_columns as i32) // <- return count of updates if there are more than 0
    }

    // return error if new data not different with old data
    Err(ServiceError::BadRequest("The data has already".to_string()))
}
