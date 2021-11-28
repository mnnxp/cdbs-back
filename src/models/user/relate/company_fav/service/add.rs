use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::access::util::check_company_access;
use crate::models::user::company_fav::model::{
    IptCompanyFavData, InsertableCompanyFav
};
use crate::models::user::notification::{
    model::{NotificationType, NotificationData},
    service::register::create_notification,
};
use crate::schema::company_fav::dsl as company_fav;
use crate::schema::company_ref::dsl as company_ref;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_company_fav(
    data: &IptCompanyFavData,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    // check access user for company
    check_company_access(
        &data.user_uuid,
        &data.company_uuid,
        &need_access_level,
        conn
    )?;

    // if have need row, just update is_enabled to true
    let check_fav = company_fav::company_fav
        .filter(company_fav::company_uuid.eq(&data.company_uuid)
        .and(company_fav::user_uuid.eq(&data.user_uuid)))
        .select(company_fav::is_enabled)
        .first(conn);

    match check_fav {
        Ok(true) => Ok(false), // <-- if data already has
        Ok(false) => {
            // if have need row, just update is_enabled to true
            diesel::update(company_fav::company_fav)
                .filter(company_fav::company_uuid.eq(&data.company_uuid)
                .and(company_fav::user_uuid.eq(&data.user_uuid)))
                .set(company_fav::is_enabled.eq(true))
                .returning(company_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav company: {:?}", err);
                    ServiceError::InternalServerError
                })
        },
        Err(err) => {
            debug!("Err check is_enabled: {:?}", err);

            // add flag and date created
            let insertable_fav: InsertableCompanyFav = data.into();

            diesel::insert_into(company_fav::company_fav)
                .values(insertable_fav)
                .returning(company_fav::is_enabled)
                .get_result::<bool>(conn)
                .map_err(|err| {
                    debug!("Failed add fav company: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            new_notification(&data.company_uuid, conn)
        },
    }
}

fn new_notification(
    object_uuid: &Uuid,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    let user_uuid = company_ref::company_ref
        .filter(company_ref::uuid.eq(object_uuid))
        .select(company_ref::user_uuid)
        .first::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get owner company: {:?}", err);
            ServiceError::InternalServerError
        })?;

    // add notification for user
    create_notification(
        &user_uuid,
        &NotificationData {
            notification: "New follower you company".to_string(),
            degree_importance: NotificationType::Info,
        },
        conn,
    )
}
