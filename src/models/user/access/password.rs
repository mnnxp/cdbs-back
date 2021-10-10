use crate::errors::{ServiceResult, ServiceError};
use crate::models::user::access::util::{
    make_hash_salt, make_salt, verify,
};
use crate::schema::user_ref::dsl as user_ref;
use async_graphql::InputObject;
use diesel::prelude::*;
use uuid::Uuid;
// use regex::Regex;

// todo!(make without: look-around, including look-ahead and look-behind, is not supported)
// ^(?=.*\d)(?=.*[A-Z])(?=.*[a-z])(?=.*[^\w\d\s:])([^\s]){8,}$
// lazy_static::lazy_static! {
//     static ref PASSWORD_REGEXP : Regex =
//         Regex::new(r"^.{6,}$")
//             .expect("Password regexp failed!");
// }

#[derive(Deserialize, InputObject)]
pub struct IptUpdatePassword {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Queryable)]
struct HashPassword {
    psw_hash: Vec<u8>,
    psw_salt: Vec<u8>,
}

impl HashPassword {
    /// Gets password hash
    fn get_psw_hash(&self) -> &[u8] {
        &self.psw_hash
    }

    /// Gets password salt
    fn get_psw_salt(&self) -> &[u8] {
        &self.psw_salt
    }
}

/// Change password for user
pub(crate) fn change_password(
    logged_user_uuid: &Uuid,
    data: &IptUpdatePassword,
    conn: &PgConnection,
) -> ServiceResult<bool> {
    if data.old_password == data.new_password {
        // return Err(ServiceError::BadRequest(
        //     "Need different passwords".to_string()
        // ))
        return Ok(false)
    }

    let hash_pass = user_ref::user_ref
        .filter(user_ref::uuid.eq(logged_user_uuid))
        .select((
            user_ref::psw_hash,
            user_ref::psw_salt,
        ))
        .first::<HashPassword>(conn)
        .map_err(|_| ServiceError::Unauthorized)?;

    // debug!("HashPassword: {:?}", user);

    if !verify(
        hash_pass.get_psw_hash(),
        hash_pass.get_psw_salt(),
        data.old_password.as_bytes()
    ) {
        return Err(ServiceError::BadRequest(
            "Old password does not valid.".to_string()
        ))
    }

    // match PASSWORD_REGEXP.is_match(&data.new_password) {
    //     true => {},
    //     false => {
    //         // Err(ServiceError::BadRequest(
    //         //     "Password not strong".to_string()
    //         // ))
    //         Ok(false)
    //     },
    // }

    update_password(
        logged_user_uuid,
        data.new_password.as_bytes(),
        conn,
    )
}

/// Generate new hash and salt for password
/// and update data in database
fn update_password(
    logged_user_uuid: &Uuid,
    new_password: &[u8],
    conn: &PgConnection,
) -> ServiceResult<bool> {
    // get salt for hashed password
    let psw_salt = make_salt();

    // make hash with salt for save password in database
    let psw_hash = make_hash_salt(
        new_password,
        &psw_salt,
    );

    // update hash and salt in database
    let res = diesel::update(user_ref::user_ref
        .filter(user_ref::uuid.eq(logged_user_uuid)))
        .set((
            user_ref::psw_hash.eq(psw_hash),
            user_ref::psw_salt.eq(psw_salt.to_vec()),
            user_ref::updated_at.eq(chrono::Local::now().naive_local()),
        ))
        .execute(conn);

    match res {
        Ok(x) => {
            debug!("Password updated: {:?}", x);
            Ok(true)
        },
        Err(err) => {
            debug!("Failed updated password: {:?}", err);
            Err(ServiceError::InternalServerError)
        }
    }
}
