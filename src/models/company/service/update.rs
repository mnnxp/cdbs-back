use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::model::IptUpdateCompanyData;
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Update company main data
pub(crate) fn update_company_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    data: &IptUpdateCompanyData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::company_ref::dsl as company_ref;
    
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
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::orgname.ne(&value))))
            .set(company_ref::orgname.eq(value))
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

    // update column shortname
    if let Some(value) = &data.shortname {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::shortname.ne(&value))))
            .set(company_ref::shortname.eq(value))
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

    // update column inn
    if let Some(value) = &data.inn {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::inn.ne(&value))))
            .set(company_ref::inn.eq(value))
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

    // update column phone
    if let Some(value) = &data.phone {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::phone.ne(&value))))
            .set(company_ref::phone.eq(value))
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

    // update column email
    if let Some(value) = &data.email {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::email.ne(&value))))
            .set(company_ref::email.eq(value))
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
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::description.ne(&value))))
            .set(company_ref::description.eq(value))
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

    // update column address
    if let Some(value) = &data.address {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::address.ne(&value))))
            .set(company_ref::address.eq(value))
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

    // update column site_url
    if let Some(value) = &data.site_url {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::site_url.ne(&value))))
            .set(company_ref::site_url.eq(value))
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

    // update column time_zone
    if let Some(value) = &data.time_zone {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::time_zone.ne(&value))))
            .set(company_ref::time_zone.eq(value))
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
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::region_id.ne(&value))))
            .set(company_ref::region_id.eq(value))
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

    // update column company_type_id
    if let Some(value) = &data.company_type_id {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)
            .and(company_ref::company_type_id.ne(&value))))
            .set(company_ref::company_type_id.eq(value))
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

    // new date for updated_at in company_ref table if update more one column
    if count_update_columns > 0 {
        let res = diesel::update(company_ref::company_ref
            .filter(company_ref::uuid.eq(target_company_uuid)))
            .set(company_ref::updated_at.eq(chrono::Local::now().naive_local()))
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
