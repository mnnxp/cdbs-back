use crate::errors::{ServiceResult, ServiceError};
use crate::models::company::company_represent::model::IptUpdateCompanyRepresentData;
use crate::models::company::access::util::check_is_owner_with_err;
use diesel::prelude::*;
use uuid::Uuid;

/// Update company representative data
pub(crate) fn update_company_represent_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    target_company_represent_uuid: &Uuid,
    data: &IptUpdateCompanyRepresentData,
    conn: &PgConnection
) -> ServiceResult<i32> {
    use crate::schema::company_represent_ref::dsl as company_represent_ref;

    // check access user for company
    check_is_owner_with_err(
        logged_user_uuid,
        target_company_uuid,
        conn
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column region_id
    if let Some(value) = &data.region_id {
        let res = diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::region_id.ne(value))))
            .set(company_represent_ref::region_id.eq(value))
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

    // update column representation_type_id
    if let Some(value) = &data.representation_type_id {
        let res = diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::representation_type_id.ne(value))))
            .set(company_represent_ref::representation_type_id.eq(value))
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
        let res = diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::name.ne(value))))
            .set(company_represent_ref::name.eq(value))
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
        let res = diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::address.ne(value))))
            .set(company_represent_ref::address.eq(value))
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
        let res = diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::phone.ne(value))))
            .set(company_represent_ref::phone.eq(value))
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

    // new date for updated_at in company_represent_ref table if update more one column
    if count_update_columns > 0 {
        debug!("Count update columns: {:?}", count_update_columns);

        return Ok(count_update_columns as i32) // <- return count of updates if there are more than 0
    }

    // return error if new data not different with old data
    Err(ServiceError::BadRequest("The data has already".to_string()))
}
