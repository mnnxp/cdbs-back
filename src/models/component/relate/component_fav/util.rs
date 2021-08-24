use crate::errors::ServiceResult;
use crate::schema::component_fav::dsl as component_fav;
use diesel::prelude::*;
use uuid::Uuid;

/// Checking the subscribe for the component
pub(crate) fn check_subscriber_by_uuid(
    target_uuid_component: &Uuid,
    target_uuid_user: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let check_subscriber = component_fav::component_fav
        .filter(component_fav::uuid_component.eq(target_uuid_component)
        .and(component_fav::uuid_user.eq(target_uuid_user)))
        .execute(conn)
        .expect("Fail load uuid list target user");

    match check_subscriber {
        0 => Ok(false),
        _ => Ok(true),
    }
}
