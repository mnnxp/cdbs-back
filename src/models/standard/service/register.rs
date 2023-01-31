use crate::errors::{ServiceError, ServiceResult};
use crate::models::standard::{
    model::{IptStandardData, InsertableStandard},
    access::util::check_access_standard_for_user,
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
    conn: &mut PgConnection
) -> ServiceResult<Uuid> {
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

    if let Some(parent_standard_uuid) = &data.parent_standard_uuid {
        check_access_standard_for_user(
            logged_user_uuid,
            parent_standard_uuid,
            &3, // need_access_level
            conn
        )?;
    }

    let mut insert_data: InsertableStandard = data.into();

    // set logged user as owner company
    insert_data.set_user_uuid(logged_user_uuid);
    // set default company favicon
    insert_data.set_image_uuid();

    match insert_data.parent_uuid_is_nil() {
        true => {
            // set parent standard uuid to base
            insert_data.parent_uuid_to_base();

            let new_uuid = diesel::insert_into(standard_ref::standard_ref)
                .values(&insert_data)
                .returning(standard_ref::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Failed created standard: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            diesel::update(standard_ref::standard_ref)
                .filter(standard_ref::uuid.eq(&new_uuid))
                .set(standard_ref::parent_standard_uuid.eq(&new_uuid))
                .returning(standard_ref::uuid)
                .get_result::<Uuid>(conn)
                .map_err(|err| {
                    debug!("Error change parent standard uuid: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        false => diesel::insert_into(standard_ref::standard_ref)
            .values(&insert_data)
            .returning(standard_ref::uuid)
            .get_result::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed created standard: {:?}", err);
                ServiceError::InternalServerError
            }),
    }
}
