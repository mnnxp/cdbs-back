use crate::errors::ServiceResult;
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::company::company_represent::model::IptUpdateCompanyRepresentData;
use crate::models::company::access::util::check_is_owner_with_err;
use crate::schema::company_represent_ref::dsl as company_represent_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновление информации о представительстве компании.
/// Возвращает количество успешных изменений.
/// И будет возвращена ошибка, если все отправленные данные уже отправлены.
pub(crate) fn update_company_represent_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    target_company_represent_uuid: &Uuid,
    data: &IptUpdateCompanyRepresentData,
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

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::region_id.ne(value))))
            .set(company_represent_ref::region_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column representation_type_id
    if let Some(value) = &data.representation_type_id {
        count_update_columns += diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::representation_type_id.ne(value))))
            .set(company_represent_ref::representation_type_id.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column name
    if let Some(value) = &data.name {
        count_update_columns += diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::name.ne(value))))
            .set(company_represent_ref::name.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column address
    if let Some(value) = &data.address {
        count_update_columns += diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::address.ne(value))))
            .set(company_represent_ref::address.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    // update column phone
    if let Some(value) = &data.phone {
        count_update_columns += diesel::update(company_represent_ref::company_represent_ref
            .filter(company_represent_ref::uuid.eq(target_company_represent_uuid)
            .and(company_represent_ref::phone.ne(value))))
            .set(company_represent_ref::phone.eq(value))
            .execute(conn)
            .map_err(|err| {
                debug!("Failed update data: {:?}", err);
                get_err_msg(ErrorMessage::FailedUpdateData)
            })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(get_err_msg(ErrorMessage::DataHasAlready))
    }

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
