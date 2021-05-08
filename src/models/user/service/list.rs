use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::models::user::model::ShowUser;
use diesel::prelude::*;

pub(crate) fn find_all_users(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<ShowUser>> {
    use crate::schema::user_ref::dsl::*;
    use crate::schema::type_user_ref::dsl::*;
    use crate::schema::name_cad_ref::dsl::*;
    use crate::schema::region_ref::dsl::*;
    let conn: &PooledConnection = &context.db;

    // joinable!(type_user_ref -> user_ref (id));
    // joinable!(name_cad_ref -> user_ref (id));
    // joinable!(region_ref -> user_ref (id));
    // allow_tables_to_appear_in_same_query!(user_ref, type_user_ref, name_cad_ref, region_ref);

    Ok(user_ref
        .inner_join(type_user_ref)
        .inner_join(name_cad_ref)
        .inner_join(region_ref)
        .select((
            uuid, email, email_verified, id_type_user,
            typeusershort, is_supplier, firstname, lastname,
            secondname, username, orgname, shortname,
            inn, phone, id_name_cad, name_cad,
            comment, address, time_zone, position,
            site_url, uuid_file_info_icon, id_region,
            region, created_at
        ))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<ShowUser>(conn)?)
}

// SELECT * FROM user_ref INNER JOIN type_user_ref
// ON (user_ref.id_type_user = type_user_ref.typeusershort);
// id_type_user
// id_name_cad
// id_region
