use crate::database::PooledConnection;
use crate::errors::{ServiceError, ServiceResult};
// use crate::graphql::model::Context;
use crate::database::Pool;
use crate::models::user::model::ShowUser;
use async_graphql::Context;
use diesel::prelude::*;
// use r2d2::PooledConnection;
use uuid::Uuid;

fn get_conn(context: &Context<'_>) -> PooledConnection {
    context
        .data::<Pool>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}

pub(crate) fn get_users(
    context: &Context<'_>,
    uuid_user_search: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowUser>> {
    let mut variant_selection: u8 = 0;
    if uuid_user_search > Uuid::nil() {
        variant_selection += 1;
    }

    match variant_selection {
        0 => find_all_users(context, limit, offset),
        1 => find_user(context, uuid_user_search),
        // 10
        // 11
        // 100
        // 101
        // 110
        // 111
        _ => ServiceResult::Err(ServiceError::BadRequest("What?".to_string())),
    }
}

fn find_all_users(context: &Context<'_>, limit: i32, offset: i32) -> ServiceResult<Vec<ShowUser>> {
    use crate::schema::user_ref::dsl::*;
    // use crate::schema::type_user_ref::dsl::*;
    // use crate::schema::name_cad_ref::dsl::*;
    // use crate::schema::region_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context);

    // joinable!(type_user_ref -> user_ref (id));
    // joinable!(name_cad_ref -> user_ref (id));
    // joinable!(region_ref -> user_ref (id));
    // allow_tables_to_appear_in_same_query!(user_ref, type_user_ref, name_cad_ref, region_ref);

    Ok(user_ref
        // .inner_join(type_user_ref)
        // .inner_join(name_cad_ref)
        // .inner_join(region_ref)
        .select((
            uuid,
            email,
            firstname,
            lastname,
            secondname,
            username,
            phone,
            description,
            address,
            position,
            time_zone,
            uuid_image_file,
            id_region,
            id_program,
            is_email_verified,
            is_enabled,
            is_delete,
            created_at,
            updated_at,
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowUser>(conn)?)
}

fn find_user(context: &Context<'_>, uuid_user_search: Uuid) -> ServiceResult<Vec<ShowUser>> {
    use crate::schema::user_ref::dsl::*;
    // use crate::schema::type_user_ref::dsl::*;
    // use crate::schema::name_cad_ref::dsl::*;
    // use crate::schema::region_ref::dsl::*;
    let conn: &PooledConnection = &get_conn(context);

    // joinable!(type_user_ref -> user_ref (id));
    // joinable!(name_cad_ref -> user_ref (id));
    // joinable!(region_ref -> user_ref (id));
    // allow_tables_to_appear_in_same_query!(user_ref, type_user_ref, name_cad_ref, region_ref);

    Ok(user_ref
        // .inner_join(type_user_ref)
        // .inner_join(name_cad_ref)
        // .inner_join(region_ref)
        .filter(uuid.eq(uuid_user_search))
        .select((
            uuid,
            email,
            firstname,
            lastname,
            secondname,
            username,
            phone,
            description,
            address,
            position,
            time_zone,
            uuid_image_file,
            id_region,
            id_program,
            is_email_verified,
            is_enabled,
            is_delete,
            created_at,
            updated_at,
        ))
        .load::<ShowUser>(conn)?)
}

// SELECT * FROM user_ref INNER JOIN type_user_ref
// ON (user_ref.id_type_user = type_user_ref.typeusershort);
// id_type_user
// id_name_cad
// id_region
