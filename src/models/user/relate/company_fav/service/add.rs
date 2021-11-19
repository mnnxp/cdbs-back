use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_company_access;
use crate::models::user::company_fav::model::{
    IptCompanyFavData, InsertableCompanyFav
};
use crate::schema::company_fav::dsl as company_fav;
use diesel::prelude::*;

pub(crate) fn add_company_fav(
    data: &IptCompanyFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        &data.user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = company_fav::company_fav
        .filter(company_fav::company_uuid.eq(&data.company_uuid)
        .and(company_fav::user_uuid.eq(&data.user_uuid)))
        .select(company_fav::is_enabled)
        .first(conn);

    match check_fav {
        Ok(fav) => {
            if fav {
                // if data already has
                Ok(false)
            } else {
                // if have need row, just update is_enabled to true
                diesel::update(company_fav::company_fav)
                    .filter(company_fav::company_uuid.eq(&data.company_uuid)
                    .and(company_fav::user_uuid.eq(&data.user_uuid)))
                    .set(company_fav::is_enabled.eq(true))
                    .returning(company_fav::is_enabled)
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed add fav company: {:?}", err);
                        ServiceError::InternalServerError
                    })
            }
        },
        Err(err) => {
            debug!("Err check is_enabled: {:?}", err);

            // add flag and date created
            let insertable_fav: InsertableCompanyFav = data.into();

            diesel::insert_into(company_fav::company_fav)
                .values(insertable_fav)
                .returning(company_fav::is_enabled)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed add fav company: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
    }
}
