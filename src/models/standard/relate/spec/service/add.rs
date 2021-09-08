use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::standard::spec::model::{
    SpecStandard,
    IptSpecStandardData,
    InsertableSpecStandard
};
use diesel::prelude::*;
// use uuid::Uuid;

pub(crate) fn add_standard_spec(
    data: IptSpecStandardData,
    conn: &PgConnection
) -> ServiceResult<SpecStandard> {
    use crate::schema::spec_to_standard::dsl::*;

    let new_standard_spec: InsertableSpecStandard = data.into();

    let flag_found_spec = spec_to_standard
        .filter(uuid_standard.eq(&new_standard_spec.uuid_standard)
        .and(id_spec.eq(&new_standard_spec.id_spec)))
        .execute(conn).unwrap_or(0);

    // debug!("fn create_spec START SEARCH ={:?}", flag_found_spec);

    match flag_found_spec as i32 {
        0 => {
            let inserted_standard_spec: SpecStandard = diesel::insert_into(spec_to_standard)
                .values(&new_standard_spec)
                .get_result(conn)?;
            Ok(inserted_standard_spec)
        },
        _ => Err(ServiceError::BadRequest("This spec name is already with the standard.".to_string())),
    }
}
