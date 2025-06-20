use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::company::model::{CompaniesArg, CompanyAndRelatedData, ShowCompanyShort};
use crate::models::search::order::Paginate;
use diesel::{prelude::*, PgConnection};
use uuid::Uuid;

/// Возвращает агрегированные данные о компаниях с фильтрами по UUID, избранному, поставщикам.
/// Получает краткие данные о компаниях с фильтрацией по:
/// UUID, UUID пользователя, избранному (для себя или другого пользователя).
pub(crate) fn get_companies(
    logged_user_uuid: &Uuid,
    arguments: &CompaniesArg,
    // sort: &Sort,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<ShowCompanyShort>> {
    // structure for reduce the number of function arguments
    let CompaniesArg {
        filter_companies_uuids,
        user_uuid,
        favorite,
        supplier,
        set_lang_id,
    } = arguments;

    // collect companies uuids for check access
    let target_companies_uuids: Vec<Uuid> = match (user_uuid, favorite) {
        // gets companies of user list with/without filter
        (Some(ur_uuid), false) => {
            get_companies_by_user(
                filter_companies_uuids,
                ur_uuid, // user_uuid
                paginate,
                conn,
            )?
        }
        // gets companies of other user favorite list with/without filter
        (Some(ur_uuid), true) => {
            get_companies_followed_by_user(
                filter_companies_uuids,
                ur_uuid, // user_uuid
                paginate,
                conn,
            )?
        }
        // gets companies of self favorite list with/without filter
        (None, true) => get_companies_followed_by_user(
            filter_companies_uuids,
            logged_user_uuid,
            paginate,
            conn,
        )?,
        // get all public companies
        (None, false) => filter_companies_uuids.to_vec(),
    };

    // return not found if set search favorite and no favorite companies
    if (*favorite || user_uuid.is_some()) && target_companies_uuids.is_empty() {
        return Ok(Vec::new());
    }

    ShowCompanyShort::get_companies(
        logged_user_uuid,
        &target_companies_uuids,
        supplier,
        paginate,
        set_lang_id,
        conn,
    )
    .map_err(|err| {
        debug!("Failed get companies data: {:?}", err);
        get_err_msg(ErrorMessage::AccessDenied)
    })
}

/// Gets list with uuids companies by owner user
/// with/without filter
fn get_companies_by_user(
    filter_companies_uuids: &[Uuid],
    user_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_ref::dsl as company_ref;

    let mut query = company_ref::company_ref.into_boxed();

    query = match filter_companies_uuids.is_empty() {
        true => query.filter(company_ref::user_uuid.eq(user_uuid)),
        false => query.filter(
            company_ref::user_uuid
                .eq(user_uuid)
                .and(company_ref::uuid.eq_any(filter_companies_uuids)),
        ),
    };

    query
        .select(company_ref::uuid)
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Gets list with uuids companies by followed user
/// with/without filter
fn get_companies_followed_by_user(
    filter_companies_uuids: &[Uuid],
    user_uuid: &Uuid,
    paginate: &Paginate,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    use crate::schema::company_fav::dsl as company_fav;

    let mut query = company_fav::company_fav.into_boxed();

    query = match filter_companies_uuids.is_empty() {
        true => query.filter(
            company_fav::user_uuid
                .eq(user_uuid)
                .and(company_fav::is_enabled.eq(true)),
        ),
        false => query.filter(
            company_fav::user_uuid
                .eq(user_uuid)
                .and(company_fav::is_enabled.eq(true))
                .and(company_fav::company_uuid.eq_any(filter_companies_uuids)),
        ),
    };

    query
        .select(company_fav::company_uuid)
        .limit(paginate.limit)
        .offset(paginate.offset)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get company fav: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Возвращает основные и связанные данные компании по UUID.
pub(crate) fn find_by_uuid(
    logged_user_uuid: &Uuid,
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {
    // collect data for company
    CompanyAndRelatedData::get_by_uuid(target_company_uuid, logged_user_uuid, set_lang_id, conn)
        .map_err(|err| {
            debug!("Error loading company and collect related data: {:?}", err);
            get_err_msg(ErrorMessage::AccessDenied)
        })
}

/// Возвращает основные и связанные данные компании со статусом поставщика.
/// Без проверки авторизации и прав пользователя.
/// Фильтр на статус поставщика и открытый доступ.
pub(crate) fn get_supplier_by_uuid(
    target_company_uuid: &Uuid,
    set_lang_id: &i32,
    conn: &mut PgConnection,
) -> ServiceResult<CompanyAndRelatedData> {
    // collect data for company
    CompanyAndRelatedData::get_supplier_by_uuid(target_company_uuid, set_lang_id, conn).map_err(
        |err| {
            debug!(
                "Error loading supplier company and collect related data: {:?}",
                err
            );
            ServiceError::InternalServerError
        },
    )
}
