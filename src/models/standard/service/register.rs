use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::standard::model::{InsertableStandard, SlimStandard, Standard, StandardData};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn create_standard(
    new_standard_data: StandardData,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    use crate::schema::company_ref::dsl::*;
    use crate::schema::company_ref::dsl::uuid as uuid_company;
    use crate::schema::company_ref::dsl::uuid_user as uuid_user_owner_company;
    use crate::schema::standard_ref::dsl::*;
    use diesel::dsl::count;

    let flag_found_company: i64 = company_ref
        .filter(uuid_user_owner_company.eq(new_standard_data.uuid_user))
        .filter(uuid_company.eq(new_standard_data.uuid_company))
        .select(count(uuid_company))
        .first(conn).unwrap();

    // debug!("fn create_standard START SEARCH ={:?}", flag_found_company);

    match flag_found_company {
        0 => Err(ServiceError::BadRequest("Not found this company of you.".to_string())),
        1 => {
            let new_standard_data: InsertableStandard = new_standard_data.into();
            let inserted_standard_data: Standard = diesel::insert_into(
                standard_ref)
                .values(&new_standard_data)
                .get_result(conn)?;
            Ok(inserted_standard_data.into())
        }
        _ => Err(ServiceError::BadRequest("Wow what? Found several companys.".to_string())),
    }
}
