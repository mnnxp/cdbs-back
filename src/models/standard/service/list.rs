use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::search::{model::ExtraOptions, order::Paginate};
use crate::models::standard::model::{
    ShowStandardShort, StandardAndRelatedData, StandardsArg,
};
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Возвращает агрегированные данные о стандартах.
/// Получает краткие данные о стандартах с фильтром по: UUID, компании, пользователю, избранному (для себя или другого пользователя).
pub(crate) fn get_standard(
    logged_user_uuid: &Uuid,
    arguments: &StandardsArg,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowStandardShort>> {
    // structure for reduce the number of function arguments
    let StandardsArg {
        filter_standards_uuids,
        company_uuid,
        favorite,
        limit,
        offset,
    } = arguments;

    // collect standards uuids for check access
    let target_standards_uuids: Vec<Uuid> = match (company_uuid, favorite) {
        // gets standards of user list with/without filter
        (Some(cy_uuid), false) => {
            get_standards_by_user(
                filter_standards_uuids,
                cy_uuid, // company_uuid
                limit,
                offset,
                conn
            )?
        },
        // gets standards of self favorite list with/without filter
        (None, true) => {
            get_standards_followed_by_user(
                filter_standards_uuids,
                logged_user_uuid,
                limit,
                offset,
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
        &ExtraOptions {
            logged_user_uuid: *logged_user_uuid,
            set_lang_id: *set_lang_id,
        },
        &Paginate::parsing(*limit, *offset),
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
    limit: &i32,
    offset: &i32,
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
        .limit(*limit as i64)
        .offset(*offset as i64)
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
    limit: &i32,
    offset: &i32,
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
        .limit(*limit as i64)
        .offset(*offset as i64)
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
    limit: i32,
    offset: i32,
    conn: &mut PgConnection,
) -> ServiceResult<StandardAndRelatedData> {
    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_standard_uuid,
        options,
        &Paginate::parsing(limit, offset),
        conn
    ).expect("Error loading standard and collect related data");
    debug!("Standard data: {:#?}", result);
    Ok(result)
}
