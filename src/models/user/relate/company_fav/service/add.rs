use crate::errors::ServiceResult;
use crate::models::user::company_fav::model::{
    CompanyFav,
    IptCompanyFavData,
    InsertableCompanyFav,
};
use crate::schema::company_fav::dsl::*;
use diesel::prelude::*;

pub(crate) fn add_company_fav(
    data: IptCompanyFavData,
    conn: &PgConnection,
) -> ServiceResult<CompanyFav> {
    // if have need row, just update is_enabled to true
    let check_fav = diesel::update(company_fav)
        .filter(company_uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(&data.user_uuid)))
        .set(is_enabled.eq(true))
        .get_result(conn);

    let user_company_fav: CompanyFav = match check_fav {
        Ok(fav) => fav,
        Err(_) => {
            // add flag and date created
            let insertable_fav: InsertableCompanyFav = data.into();

            diesel::insert_into(company_fav)
                .values(insertable_fav)
                .get_result(conn)?
        },
    };

    debug!("User favorite company: {:#?}", user_company_fav);

    Ok(user_company_fav)
}
