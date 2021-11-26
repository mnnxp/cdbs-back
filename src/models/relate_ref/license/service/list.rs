use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::license::model::License;
use crate::schema::license_ref::dsl as license_ref;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_licenses(
    license_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<License>> {
    match license_id_search.is_empty()  {
        true => find_all_license(limit, offset, conn),
        false => find_license_id(license_id_search, limit, offset, conn)
    }
}

fn find_all_license(
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<License>> {
    license_ref::license_ref
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<License>(conn)
        .map_err(|err| {
            debug!("Failed get license: {:?}", err);
            ServiceError::InternalServerError
        })
}

fn find_license_id(
    license_id_search: &[i32],
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<License>> {
    license_ref::license_ref
        .filter(license_ref::id.eq_any(license_id_search))
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<License>(conn)
        .map_err(|err| {
            debug!("Failed get licenses: {:?}", err);
            ServiceError::InternalServerError
        })
}
