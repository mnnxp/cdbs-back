use crate::errors::{
    ServiceError,
    ServiceResult
};
use crate::models::standard::model::{
    IptStandardData,
    InsertableStandard,
    SlimStandard,
    Standard,
    StandardData
};
use crate::schema::company_ref::dsl as company_ref;
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::{dsl::count, prelude::*};
use uuid::Uuid;

pub(crate) fn create_standard(
    logged_uuid_user: Uuid,
    data: IptStandardData,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    let target_uuid_company = Uuid::parse_str(&data.uuid_company.0)?;

    let uuid_standard_parent = match &data.uuid_standard_parent {
        Some(parent) => Uuid::parse_str(&parent.0)?,
        None => Uuid::parse_str("303ec2aa-2066-42e3-93fb-de4fb9344bcb")?, // <-- todo!(get uuid root standard)
    };

    let uuid_image_file = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    crate::models::company::util::check_company_access(
        &logged_uuid_user,
        &target_uuid_company,
        3,
        conn,
    )?;

    crate::models::company::util::check_is_supplier(
        &target_uuid_company,
        conn
    )?;

    let new_standard_data = StandardData {
        uuid_standard_parent,
        classifier: data.classifier,
        name: data.name,
        description: data.description,
        specified_tolerance: data.specified_tolerance,
        technical_committee: data.technical_committee,
        publication_at: data.publication_at,
        uuid_image_file,
        uuid_user: logged_uuid_user,
        uuid_company: target_uuid_company,
        id_type_access: data.id_type_access,
        id_standard_status: data.id_standard_status,
        id_region: data.id_region,
    };

    let flag_found_company: i64 = company_ref::company_ref
        .filter(company_ref::uuid_user.eq(&new_standard_data.uuid_user))
        .filter(company_ref::uuid.eq(&new_standard_data.uuid_company))
        .select(count(company_ref::uuid))
        .first(conn).unwrap();

    // debug!("fn create_standard START SEARCH ={:?}", flag_found_company);

    match flag_found_company {
        0 => Err(ServiceError::BadRequest("Not found this company of you.".to_string())),
        1 => {
            let data: InsertableStandard = new_standard_data.into();
            let inserted_standard_data: Standard = diesel::insert_into(standard_ref::standard_ref)
                .values(&data)
                .get_result(conn)?;
            Ok(inserted_standard_data.into())
        }
        _ => Err(ServiceError::BadRequest("Wow what? Found several companys.".to_string())),
    }
}
