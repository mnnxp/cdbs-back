use argon2::{self, Config};

const SALT_LEN: usize = 128;

pub(crate) fn make_salt() -> [u8; SALT_LEN] {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let mut psw_salt: [u8; SALT_LEN] = [0_u8; SALT_LEN];

    let mut count = 0;

    while count < SALT_LEN {
        let x = rng.gen_range(0..CHARSET.len());
        psw_salt[count] = x as u8;
        count += 1;
    };

    psw_salt
}

pub(crate) fn make_hash_salt(
    password: &[u8],
    psw_salt: &[u8],
) -> Vec<u8> {
    argon2::hash_encoded(
        password,
        psw_salt,
        &Config::default()
    ).unwrap()
    .into_bytes()
}

pub(crate) fn verify(
    psw_hash: &[u8],
    psw_salt: &[u8],
    password: &[u8],
) -> bool {
    make_hash_salt(password, psw_salt) == psw_hash
}

// comparison of the received user_uuid with the user_uuid of the authorized user
// pub(crate) fn compare_user_uuid(target_auth_user_uuid: Uuid, cxt: &Context<'_>) -> ServiceResult<bool> {
//     Ok(get_auth_user_uuid(cxt, false)? == target_auth_user_uuid)
// }
