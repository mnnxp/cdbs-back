use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::company::company_fav::model::{
    CompanyFav,
    IptCompanyFavData,
    InsertableCompanyFav
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_company_favorite(
    data: IptCompanyFavData,
    conn: &PgConnection
) -> ServiceResult<CompanyFav> {
    use crate::schema::company_fav::dsl::*;

    let new_company_favorite: InsertableCompanyFav = data.into();

    let flag_found_favorite = company_fav
        .filter(company_uuid.eq(&new_company_favorite.company_uuid)
        .and(user_uuid.eq(&new_company_favorite.user_uuid)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    match flag_found_favorite as i32 {
        0 => {
            let inserted_company_favorite: CompanyFav = diesel::insert_into(company_fav)
                .values(&new_company_favorite)
                .get_result(conn)?;
            Ok(inserted_company_favorite)
        },
        _ => Err(ServiceError::BadRequest("This favorite name is already with the company.".to_string())),
    }
}
