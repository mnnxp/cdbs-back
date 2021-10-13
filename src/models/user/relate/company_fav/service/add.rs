use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_company_access;
use crate::models::user::company_fav::model::{
    IptCompanyFavData, InsertableCompanyFav
};
use crate::schema::company_fav::dsl::*;
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
    let check_fav = diesel::update(company_fav)
        .filter(company_uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .execute(conn)
        .expect("Failed check fav data");

    match check_fav {
        1_usize => Ok(true),
        0_usize => {
            // add flag and date created
            let insertable_fav: InsertableCompanyFav = data.into();

            diesel::insert_into(company_fav)
                .values(insertable_fav)
                .execute(conn)
                .expect("Failed add fav data");

            Ok(true)
        },
        _ => Err(ServiceError::InternalServerError),
    }
}
