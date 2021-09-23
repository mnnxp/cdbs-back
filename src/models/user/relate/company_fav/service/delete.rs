use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::user::company_fav::model::{
    CompanyFav,
    IptCompanyFavData,
};
use crate::schema::company_fav::dsl::*;
use diesel::prelude::*;

// Remove a company from user favorites company list
pub(crate) fn delete_company_fav(
    data: IptCompanyFavData,
    conn: &PgConnection,
) -> ServiceResult<CompanyFav> {
    // if have need row, just update is_enabled to false
    let check_fav = diesel::update(company_fav)
        .filter(company_uuid.eq(&data.company_uuid)
        .and(user_uuid.eq(&data.user_uuid))
        .and(is_enabled.eq(true))) // <-- active favorite
        .set(is_enabled.eq(false)) // <-- off favorite company
        .get_result(conn);

    let user_company_fav: CompanyFav = match check_fav {
        Ok(fav) => fav, // <-- turned flag to false
        Err(err) => {
            debug!("Err with delete company fav: {:#?}", err);
            // company not found in favorite list
            return Err(ServiceError::BadRequest("Company not found in favotite list".to_string()))
        },
    };

    debug!("Company delete from favorute: {:#?}", user_company_fav);

    Ok(user_company_fav)
}
