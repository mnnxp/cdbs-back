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
    logged_user_uuid: Uuid,
    data: IptStandardData,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    let parent_standard_uuid = match data.parent_standard_uuid {
        Some(parent) => parent,
        None => Uuid::parse_str("303ec2aa-2066-42e3-93fb-de4fb9344bcb")?, // <-- todo!(get uuid root standard)
    };

    let image_file_uuid = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    crate::models::company::util::check_company_access(
        &logged_user_uuid,
        &data.company_uuid,
        3,
        conn,
    )?;

    crate::models::company::util::check_is_supplier(
        &data.company_uuid,
        conn
    )?;

    let new_standard_data = StandardData {
        parent_standard_uuid,
        classifier: data.classifier,
        name: data.name,
        description: data.description,
        specified_tolerance: data.specified_tolerance,
        technical_committee: data.technical_committee,
        publication_at: data.publication_at,
        image_file_uuid,
        user_uuid: logged_user_uuid,
        company_uuid: data.company_uuid,
        type_access_id: data.type_access_id,
        standard_status_id: data.standard_status_id,
        region_id: data.region_id,
    };

    let flag_found_company: i64 = company_ref::company_ref
        .filter(company_ref::user_uuid.eq(&new_standard_data.user_uuid))
        .filter(company_ref::uuid.eq(&new_standard_data.company_uuid))
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
