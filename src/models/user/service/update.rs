use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::user::model::IptUpdateUserData;
use crate::models::user::util::check_use_username;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет базовые данные пользователя по UUID.
/// Возвращает количество успешных изменений или ошибку, если все указанные данные уже существуют.
pub(crate) fn update_user(
    logged_user_uuid: &Uuid,
    data: &IptUpdateUserData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // update data validation
    if let Some(username) = &data.username {
        if check_use_username(username, conn)? {
            return Err(get_err_msg(ErrorMessage::UsernameIsAlreadyUsed))
        }
    }
    if data.description.as_ref().map(|d| d.len()).unwrap_or_default() > 2000 {
        return Err(get_err_msg(ErrorMessage::TextMustLess(2000)));
    }

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column email
    if let Some(value) = &data.email {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::email.ne(value))))
            .set(user_ref::email.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column firstname
    if let Some(value) = &data.firstname {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::firstname.ne(value))))
            .set(user_ref::firstname.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column lastname
    if let Some(value) = &data.lastname {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::lastname.ne(value))))
            .set(user_ref::lastname.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column secondname
    if let Some(value) = &data.secondname {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::secondname.ne(value))))
            .set(user_ref::secondname.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column username
    if let Some(value) = &data.username {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::username.ne(value))))
            .set(user_ref::username.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column phone
    if let Some(value) = &data.phone {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::phone.ne(value))))
            .set(user_ref::phone.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::description.ne(value))))
            .set(user_ref::description.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column address
    if let Some(value) = &data.address {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::address.ne(value))))
            .set(user_ref::address.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column position
    if let Some(value) = &data.position {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::position.ne(value))))
            .set(user_ref::position.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column time_zone
    if let Some(value) = &data.time_zone {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::time_zone.ne(value))))
            .set(user_ref::time_zone.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::region_id.ne(value))))
            .set(user_ref::region_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column program_id
    if let Some(value) = &data.program_id {
        count_update_columns += diesel::update(user_ref::user_ref
            .filter(user_ref::uuid.eq(logged_user_uuid)
            .and(user_ref::program_id.ne(value))))
            .set(user_ref::program_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // new date for updated_at in user_ref table if update more one column
    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(get_err_msg(ErrorMessage::DataHasAlready))
    }

    diesel::update(user_ref::user_ref
        .filter(user_ref::uuid.eq(logged_user_uuid)))
        .set(user_ref::updated_at.eq(chrono::Local::now().naive_local()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
