use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_company_access;
use crate::models::company::spec::model::{InsertableCompanySpec, IptCompanySpecsData};
use crate::schema::spec_to_company::dsl as spec_to_company;
use diesel::prelude::*;
use uuid::Uuid;

/// Добавляет подключения компании в указанные разделы каталога.
/// Возвращает количество успешных подключений.
/// И будет возвращена ошибка, если все соединения уже добавлены.
pub(crate) fn add_company_specs(
    logged_user_uuid: &Uuid,
    data: &IptCompanySpecsData,
    conn: &mut PgConnection,
) -> ServiceResult<i32> {
    let need_access_level = 1; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    let mut count_insert_rows = 0; // <-- for accumulated count inserted rows
    let mut error_sc_has: Vec<i32> = Vec::new(); // <-- for accumulated spec duplicates

    // creating structures for inserting records into a table
    let new_company_specs: Vec<InsertableCompanySpec> = data.into();

    if new_company_specs.is_empty() {
        // return error if not found correct specs
        return Err(get_err_msg(ErrorMessage::NotFoundSpecs));
    }

    let mut insert_data: Vec<InsertableCompanySpec> = Vec::new();

    for company_sc in new_company_specs {
        // check new row on non duplicate
        let flag_found_spec = spec_to_company::spec_to_company
            .filter(
                spec_to_company::company_uuid
                    .eq(&company_sc.company_uuid)
                    .and(spec_to_company::spec_id.eq(&company_sc.spec_id)),
            )
            .execute(conn)
            .map_err(|err| {
                debug!("Fail check spec: {:?}", err);
                ServiceError::InternalServerError
            })?;

        match flag_found_spec {
            0 => {
                debug!("Inserted company spec: {:?}", &company_sc.spec_id);
                insert_data.push(company_sc);
                count_insert_rows += 1;
            }
            _ => error_sc_has.push(company_sc.spec_id),
        }
    }

    if insert_data.is_empty() {
        // return error if all spec duplicate
        return Err(get_err_msg(ErrorMessage::IdsAlreadyHas(error_sc_has)));
    }

    diesel::insert_into(spec_to_company::spec_to_company)
        .values(&insert_data)
        .returning(spec_to_company::spec_id)
        .get_result::<i32>(conn)
        .map_err(|err| {
            debug!("Fail inserted spec: {:?}", err);
            ServiceError::InternalServerError
        })?;

    Ok(count_insert_rows)
}
