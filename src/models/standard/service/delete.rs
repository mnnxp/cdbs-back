use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_is_owner_with_err;
use crate::models::relate_ref::file::service::delete::{delete_file_by_uuid, delete_file_by_uuids};
use crate::models::standard::access::util::check_is_owner;
use crate::schema::{file_to_standard::dsl as file_to_standard, standard_ref::dsl as standard_ref};
use diesel::prelude::*;
use uuid::Uuid;

/// Удаляет стандарт и связанные с ним данные.
/// Возвращает UUID удалённого стандарта.
pub(crate) fn del_standard_data(
    logged_user_uuid: &Uuid,
    del_standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // check ownership standard
    if !check_is_owner(logged_user_uuid, del_standard_uuid, conn)? {
        // if user not ownership standard
        let owner_company_uuid = standard_ref::standard_ref
            .filter(standard_ref::uuid.eq(del_standard_uuid))
            .select(standard_ref::company_uuid)
            .first::<Uuid>(conn)
            .map_err(|err| {
                debug!("Not found standard: {:?}", err);
                get_err_msg(ErrorMessage::NotFoundStandard)
            })?;

        // check ownership company of standard
        check_is_owner_with_err(logged_user_uuid, &owner_company_uuid, conn)?;
    }

    delete_standard(del_standard_uuid, conn)
}

/// Delete standard and related data
pub(crate) fn delete_standard(
    del_standard_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Uuid> {
    // set flags for standard files
    delete_standard_files(del_standard_uuid, conn)?;

    // Delere row about standard in database
    let image_file_uuid =
        diesel::delete(standard_ref::standard_ref.filter(standard_ref::uuid.eq(del_standard_uuid)))
            .returning(standard_ref::image_file_uuid)
            .get_result::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed delete standard: {:?}", err);
                ServiceError::InternalServerError
            })?;

    // set delete flag for main image deleted a standard
    delete_file_by_uuid(&image_file_uuid, conn)?;

    Ok(*del_standard_uuid)
}

/// Set the delete flags for all files associated with the standard
fn delete_standard_files(standard_uuid: &Uuid, conn: &mut PgConnection) -> ServiceResult<bool> {
    let del_file_uuids = file_to_standard::file_to_standard
        .filter(file_to_standard::standard_uuid.eq(standard_uuid))
        .select(file_to_standard::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed gets file of standard: {:?}", err);
            ServiceError::InternalServerError
        })?;

    delete_file_by_uuids(&del_file_uuids, conn)
}
