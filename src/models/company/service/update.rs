use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::ServiceResult;
use crate::models::company::model::IptUpdateCompanyData;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Обновляет основные данные компании. Возвращает количество успешных изменений.
pub(crate) fn update_company_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    data: &IptUpdateCompanyData,
    conn: &mut PgConnection,
) -> ServiceResult<usize> {
    // update data validation
    if data
        .description
        .as_ref()
        .map(|d| d.chars().count())
        .unwrap_or_default()
        > 50000
    {
        return Err(get_err_msg(ErrorMessage::TextMustLess(50000)));
    }

    require_permission(
        logged_user_uuid,
        AccessEntity::Company,
        target_company_uuid,
        AccessOperation::Manage,
        conn,
    )?;

    // for returning change count
    let mut count_update_columns = 0_usize;

    // update column orgname
    if let Some(value) = &data.orgname {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::orgname.ne(value)),
            ),
        )
        .set(company_ref::orgname.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column shortname
    if let Some(value) = &data.shortname {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::shortname.ne(value)),
            ),
        )
        .set(company_ref::shortname.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column inn
    if let Some(value) = &data.inn {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::inn.ne(value)),
            ),
        )
        .set(company_ref::inn.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column phone
    if let Some(value) = &data.phone {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::phone.ne(value)),
            ),
        )
        .set(company_ref::phone.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column email
    if let Some(value) = &data.email {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::email.ne(value)),
            ),
        )
        .set(company_ref::email.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column description
    if let Some(value) = &data.description {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::description.ne(value)),
            ),
        )
        .set(company_ref::description.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column address
    if let Some(value) = &data.address {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::address.ne(value)),
            ),
        )
        .set(company_ref::address.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column site_url
    if let Some(value) = &data.site_url {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::site_url.ne(value)),
            ),
        )
        .set(company_ref::site_url.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column time_zone
    if let Some(value) = &data.time_zone {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::time_zone.ne(value)),
            ),
        )
        .set(company_ref::time_zone.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column region_id
    if let Some(value) = &data.region_id {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::region_id.ne(value)),
            ),
        )
        .set(company_ref::region_id.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    // update column company_type_id
    if let Some(value) = &data.company_type_id {
        count_update_columns += diesel::update(
            company_ref::company_ref.filter(
                company_ref::uuid
                    .eq(target_company_uuid)
                    .and(company_ref::company_type_id.ne(value)),
            ),
        )
        .set(company_ref::company_type_id.eq(value))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;
    }

    if count_update_columns == 0 {
        // return error if new data not different with old data
        return Err(get_err_msg(ErrorMessage::DataHasAlready));
    }

    diesel::update(company_ref::company_ref.filter(company_ref::uuid.eq(target_company_uuid)))
        .set(company_ref::updated_at.eq(chrono::Utc::now().naive_utc()))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed update data: {:?}", err);
            get_err_msg(ErrorMessage::FailedUpdateData)
        })?;

    debug!("Count update columns: {:?}", count_update_columns);

    Ok(count_update_columns)
}
