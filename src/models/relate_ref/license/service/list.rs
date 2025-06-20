use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::license::model::License;
use crate::models::search::order::Paginate;
use crate::schema::license_ref::dsl as license_ref;
use diesel::{prelude::*, PgConnection};

/// Returns a list of available licenses.
/// If no license filter is specified, all existing licenses are aggregated.
pub(crate) fn get_licenses(
    license_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<License>> {
    match license_ids.is_empty() {
        true => find_all_license(paginate, conn),
        false => find_license_id(license_ids, paginate, conn),
    }
}

fn find_all_license(paginate: &Paginate, conn: &mut PgConnection) -> ServiceResult<Vec<License>> {
    license_ref::license_ref
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<License>(conn)
        .map_err(|err| {
            debug!("Failed get license: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_license_id(
    license_ids: &[i32],
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<License>> {
    license_ref::license_ref
        .filter(license_ref::id.eq_any(license_ids))
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<License>(conn)
        .map_err(|err| {
            debug!("Failed get licenses: {:?}", err);
            ServiceError::InternalServerError
        })
}
