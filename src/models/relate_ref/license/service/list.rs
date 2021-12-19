use crate::errors::{ServiceResult, ServiceError};
use crate::models::relate_ref::license::model::{
    License, LicenseArg
};
use crate::schema::license_ref::dsl as license_ref;
use diesel::{PgConnection, prelude::*};

pub(crate) fn get_licenses(
    args: &LicenseArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<License>> {
    match args.license_ids.is_empty()  {
        true => find_all_license(&args.limit, &args.offset, conn),
        false => find_license_id(args, conn)
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
    args: &LicenseArg,
    conn: &PgConnection,
) -> ServiceResult<Vec<License>> {
    license_ref::license_ref
        .filter(license_ref::id.eq_any(&args.license_ids))
        .limit(args.limit as i64)
        .offset(args.offset as i64)
        .load::<License>(conn)
        .map_err(|err| {
            debug!("Failed get licenses: {:?}", err);
            ServiceError::InternalServerError
        })
}
