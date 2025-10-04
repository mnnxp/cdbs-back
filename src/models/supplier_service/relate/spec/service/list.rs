use crate::errors::ServiceResult;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::search::{model::ExtraOptions, order::Paginate};
use crate::models::supplier_service::access::util::check_access_service_for_user;
use diesel::prelude::*;
use uuid::Uuid;

/// Returns an array of directory sections associated with the service
pub(crate) fn get_service_specs(
    service_uuid: &Uuid,
    options: &ExtraOptions,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<SpecTranslateList>> {
    let need_access_level = 3; // todo!(create enum for manage access level)

    check_access_service_for_user(
        &options.logged_user_uuid,
        service_uuid,
        &need_access_level,
        conn,
    )?;

    SpecTranslateList::for_service_by_uuid(service_uuid, &options.set_lang_id, paginate, conn)
}
