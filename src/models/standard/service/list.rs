use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::model::{ShowStandardShort, StandardAndRelatedData};
use diesel::{PgConnection, prelude::*};
use uuid::Uuid;

/// Gets standard short data with filter by:
/// uuids, company_uuid, favorite (for self, for other user)
pub(crate) fn get_standard(
    logged_user_uuid: &Uuid,
    filter_standards_uuids: &[Uuid],
    company_uuid: &Option<Uuid>,
    favorite: &bool,
    limit_offset: (&i32, &i32),
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<ShowStandardShort>> {
    // tuple for reduce the number of function arguments
    let (limit, offset) = limit_offset;

    // collect standards uuids for check access
    let target_standards_uuids: Vec<Uuid> = match (company_uuid, favorite) {
        // gets standards of user list with/without filter
        (Some(company_u), false) => {
            get_standards_by_user(
                filter_standards_uuids,
                company_u,
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
        _ => {
            return Err(ServiceError::BadRequest(
                "Failed match arguments".to_string()
            ))
        },
    };

    // return not found if set search favorite and no favorite standards
    if (*favorite || company_uuid.is_some()) &&
            target_standards_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowStandardShort::get_standards(
        logged_user_uuid,
        &target_standards_uuids,
        limit, offset,
        set_lang_id,
        conn
    ).map_err(|err| {
        debug!("Error loading list standards and collect short data: {:?}", err);
        ServiceError::BadRequest("Access denied".to_string())
    })
}

/// Gets list with uuids standards by owner user
/// with/without filter
fn get_standards_by_user(
    filter_standards_uuids: &[Uuid],
    company_uuid: &Uuid,
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
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
    conn: &PgConnection,
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

pub(crate) fn find_by_uuid(
    logged_user_uuid: &Uuid,
    target_standard_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &PgConnection,
) -> ServiceResult<StandardAndRelatedData> {
    // collect data for standard
    let result: StandardAndRelatedData = StandardAndRelatedData::collect_related_data(
        target_standard_uuid,
        logged_user_uuid,
        set_lang_id,
        conn
    ).expect("Error loading standard and collect related data");

    debug!("Standard data: {:#?}", result);

    Ok(result)
}
