use crate::errors::ServiceResult;
use crate::models::user::model::{
    IptUserData,
    InsertableUser,
    SlimUser,
    User,
    UserData,
};
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn create_user(
    data: IptUserData,
    conn: &PgConnection
) -> ServiceResult<SlimUser> {
    use crate::schema::user_ref::dsl::user_ref;

    let uuid_image_file = Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?; // <-- todo!(get uuid default favicon)

    let new_user = UserData {
        email: data.email,
        password: data.password,
        firstname: data.firstname,
        lastname: data.lastname,
        secondname: data.secondname,
        username: data.username,
        phone: data.phone,
        description: data.description,
        address: data.address,
        position: data.position,
        time_zone: data.time_zone,
        uuid_image_file,
        id_region: data.id_region,
        id_program: data.id_program,
    };

    let user: InsertableUser = new_user.into();
    let inserted_user: User = diesel::insert_into(user_ref).values(&user).get_result(conn)?;
    Ok(inserted_user.into())
}
