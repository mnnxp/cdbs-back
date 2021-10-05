use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::model::{
    IptStandardData,
    InsertableStandard,
    SlimStandard,
    StandardData
};
use crate::models::company::{
    access::util::check_company_access,
    util::check_is_supplier,
};
use crate::schema::standard_ref::dsl as standard_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Created standard
pub(crate) fn create_standard(
    logged_user_uuid: &Uuid,
    data: &IptStandardData,
    conn: &PgConnection
) -> ServiceResult<SlimStandard> {
    let parent_standard_uuid = match data.parent_standard_uuid {
        Some(parent) => parent,
        None => Uuid::parse_str("303ec2aa-2066-42e3-93fb-de4fb9344bcb")?, // <-- todo!(get uuid root standard)
    };

    let image_file_uuid = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_company_access(
        logged_user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn,
    )?;

    check_is_supplier(
        &data.company_uuid,
        conn
    )?;

    let new_standard_data = StandardData {
        parent_standard_uuid,
        classifier: data.classifier.to_string(),
        name: data.name.to_string(),
        description: data.description.to_string(),
        specified_tolerance: data.specified_tolerance.to_string(),
        technical_committee: data.technical_committee.to_string(),
        publication_at: data.publication_at,
        image_file_uuid,
        user_uuid: *logged_user_uuid,
        company_uuid: data.company_uuid,
        type_access_id: data.type_access_id,
        standard_status_id: data.standard_status_id,
        region_id: data.region_id,
    };

    let data: InsertableStandard = new_standard_data.into();

    let inserted_data = diesel::insert_into(standard_ref::standard_ref)
        .values(&data)
        .returning((
            standard_ref::uuid,
            standard_ref::classifier,
            standard_ref::name,
            standard_ref::specified_tolerance,
            standard_ref::technical_committee,
            standard_ref::publication_at,
            standard_ref::standard_status_id,
        ))
        .get_result::<SlimStandard>(conn);

    match inserted_data {
        Ok(x) => Ok(x),
        Err(err) => {
            debug!("Failed created standard: {:?}", err);

            Err(ServiceError::BadRequest(
                "Failed created standard".to_string()
            ))
        },
    }
}
