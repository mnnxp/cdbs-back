use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::user::model::{InsertableUser, IptUserData, SlimUser};
use crate::models::user::util::check_use_username;
use crate::schema::user_ref::dsl as user_ref;
use diesel::prelude::*;

/// Добавляет нового пользователя.
pub(crate) fn create_user(data: IptUserData, conn: &mut PgConnection) -> ServiceResult<SlimUser> {
    if check_use_username(&data.username, conn)? {
        return Err(get_err_msg(ErrorMessage::UsernameIsAlreadyUsed));
    }

    let insert_values = InsertableUser::by_arg(data)?;

    diesel::insert_into(user_ref::user_ref)
        .values(&insert_values)
        .returning((user_ref::uuid, user_ref::username, user_ref::program_id))
        .get_result::<SlimUser>(conn)
        .map_err(|err| {
            debug!("Failed create new user: {:?}", err);
            ServiceError::InternalServerError
        })
}
