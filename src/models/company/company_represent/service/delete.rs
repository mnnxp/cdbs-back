use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::company_represent::model::{CompanyRepresent, SlimCompanyRepresent};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn delete_company_represent(
    input_uuid_company: Uuid,
    uuid_represent_delete: Uuid,
    conn: &PgConnection,
) -> ServiceResult<SlimCompanyRepresent> {
    use crate::schema::company_represent_ref::dsl::*;

    // debug!("fn input_uuid_company = {}", &input_uuid_company);
    // debug!("fn uuid_represent_delete = {}", &uuid_represent_delete);

    // find represent and check privileges for delete
    let find_represent: i32 = company_represent_ref
        .filter(uuid_company.eq(input_uuid_company))
        .filter(uuid.eq(uuid_represent_delete))
        .select(id)
        .first(conn)
        .unwrap_or(0);

    match find_represent {
        1..=i32::MAX => {
            // delete represent and save delete data for send response
            let delete_company_represent: CompanyRepresent =
                diesel::delete(company_represent_ref.filter(id.eq(find_represent)))
                    .get_result(conn)?;
            // debug!("fn delete_company_represent ={:?}", &delete_company_represent);
            Ok(delete_company_represent.into())
        }
        _ => Err(ServiceError::BadRequest(
            "The representative not you or not found.".to_string(),
        )),
    }
}
