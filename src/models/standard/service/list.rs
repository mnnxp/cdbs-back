use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::search::{model::ExtraOptions, order::Paginate};
use crate::models::standard::model::{
    ShowStandardShort, StandardAndRelatedData, StandardsArg,
};
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Returns aggregated standards data. Gets a summary of standards filtered by:
/// UUID, company, user, favorites (for yourself or another user).
pub(crate) fn get_standard(
    arguments: &StandardsArg,
    options: &ExtraOptions,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowStandardShort>> {
    // structure for reduce the number of function arguments
    let StandardsArg {
        filter_standards_uuids,
        company_uuid,
        favorite,
    } = arguments;

    // collect standards uuids for check access
    let target_standards_uuids: Vec<Uuid> = match (company_uuid, favorite) {
        // gets standards of user list with/without filter
        (Some(cy_uuid), false) => {
            get_standards_by_user(
                filter_standards_uuids,
                cy_uuid, // company_uuid
                paginate,
                conn
            )?
        },
        // gets standards of self favorite list with/without filter
        (None, true) => {
            get_standards_followed_by_user(
                filter_standards_uuids,
                &options.logged_user_uuid,
                paginate,
                conn
            )?
        },
        // get all public standards
        (None, false) => {
            filter_standards_uuids.to_vec()
        },
        _ => return Err(get_err_msg(ErrorMessage::FailedMatchArguments)),
    };

    // return not found if set search favorite and no favorite standards
    if (*favorite || company_uuid.is_some()) && target_standards_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowStandardShort::get_standards(
        &target_standards_uuids,
        options,
        paginate,
        conn,
    ).map_err(|err| {
        debug!("Error loading list standards and collect short data: {:?}", err);
        get_err_msg(ErrorMessage::AccessDenied)
    })
}

/// Gets list with uuids standards by owner user
/// with/without filter
fn get_standards_by_user(
    filter_standards_uuids: &[Uuid],
    company_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::standard_ref::dsl as standard_ref;

    let mut query = standard_ref::standard_ref.into_boxed();

    query = match filter_standards_uuids.is_empty() {
        true => {
            query.filter(standard_ref::company_uuid.eq(company_uuid))
        },
        false => {
            query.filter(standard_ref::company_uuid.eq(company_uuid)
                .and(standard_ref::uuid.eq_any(filter_standards_uuids)))
        },
    };

    query.select(standard_ref::uuid)
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets list with uuids standards by followed user
/// with/without filter
fn get_standards_followed_by_user(
    filter_standards_uuids: &[Uuid],
    user_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::standard_fav::dsl as standard_fav;

    let mut query = standard_fav::standard_fav.into_boxed();

    query = match filter_standards_uuids.is_empty() {
        true => {
            query.filter(standard_fav::user_uuid.eq(user_uuid)
                .and(standard_fav::is_enabled.eq(true)))
        },
        false => {
            query.filter(standard_fav::user_uuid.eq(user_uuid)
                .and(standard_fav::is_enabled.eq(true))
                .and(standard_fav::standard_uuid.eq_any(filter_standards_uuids)))
        },
    };

    query.select(standard_fav::standard_uuid)
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get standards fav: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Возвращает полную информацию о стандарте по UUID.
pub(crate) fn find_by_uuid(
    target_standard_uuid: &Uuid,
    options: &ExtraOptions,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<StandardAndRelatedData> {
    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_standard_uuid,
        options,
        paginate,
        conn
    ).expect("Error loading standard and collect related data");
    debug!("Standard data: {:#?}", result);
    Ok(result)
}
