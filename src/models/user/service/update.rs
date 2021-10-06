use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::IptUpdateUserData;
use diesel::prelude::*;
use uuid::Uuid;

/// Update user main data
pub(crate) fn update_user(
    logged_user_uuid: &Uuid,
    data: &IptUpdateUserData,
    conn: &PgConnection,
) -> ServiceResult<i32> {
    use crate::schema::user_ref::dsl as user_ref;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column email
    if let Some(value) = &data.email {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::email.ne(value))))
            .set(user_ref::email.eq(value))
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

    // update column firstname
    if let Some(value) = &data.firstname {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::firstname.ne(value))))
            .set(user_ref::firstname.eq(value))
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

    // update column lastname
    if let Some(value) = &data.lastname {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::lastname.ne(value))))
            .set(user_ref::lastname.eq(value))
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

    // update column secondname
    if let Some(value) = &data.secondname {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::secondname.ne(value))))
            .set(user_ref::secondname.eq(value))
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

    // update column username
    if let Some(value) = &data.username {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::username.ne(value))))
            .set(user_ref::username.eq(value))
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
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::phone.ne(value))))
            .set(user_ref::phone.eq(value))
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
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::description.ne(value))))
            .set(user_ref::description.eq(value))
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
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::address.ne(value))))
            .set(user_ref::address.eq(value))
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

    // update column position
    if let Some(value) = &data.position {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::position.ne(value))))
            .set(user_ref::position.eq(value))
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
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::time_zone.ne(value))))
            .set(user_ref::time_zone.eq(value))
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
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::region_id.ne(value))))
            .set(user_ref::region_id.eq(value))
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

    // update column program_id
    if let Some(value) = &data.program_id {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::program_id.ne(value))))
            .set(user_ref::program_id.eq(value))
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

    // new date for updated_at in user_ref table if update more one column
    if count_update_columns > 0 {
        let res = diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)))
            .set(user_ref::updated_at.eq(chrono::Local::now().naive_local()))
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
