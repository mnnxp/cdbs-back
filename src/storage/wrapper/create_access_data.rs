use crate::errors::{ServiceResult, ServiceError};
use crate::database::PgConn;
use crate::cli_args;
use crate::models::user::model::TargetUser;
use crate::storage::model::UserStorageAccess;
use crate::storage::backblaze::b2_create_key::b2_create_key;
use crate::storage::backblaze::b2_authorize_account::b2_authorize_account;
use crate::storage::backblaze::b2_types::CreateKeyRequest;

/// Gets new storage account and access for user
pub(crate) async fn get_new_storage_access(
    target_user: TargetUser,
    pool: PgConn,
) -> ServiceResult<UserStorageAccess> {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    // Gets enviroment variables from `.env`
    dotenv::dotenv().ok();

    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let get_main_access = UserStorageAccess::get(
        &opt.uuid_main_access_b2,
        &conn
    );

    if let Err(access_err) = get_main_access {
        let update = update_main_access(pool.clone()).await;
        debug!("Update main access after failed fet: {:#?}", update);

        return Err(access_err)
    };

    let main_access = match get_main_access {
        Ok(access) => access,
        Err(e) => return Err(e),
    };

    let api_url = main_access.api_url;

    let authorization_token = main_access.authorization_token;

    // todo!(make parameters parsing by means of StructOpt)
    let mut capabilities: Vec<String> = Vec::new();
    // parsing B2_CAPABILITIES from .env
    for capability in opt.b2_capabilities.rsplit(',') {
        capabilities.push(capability.to_string())
    }

    debug!("Capabilities: {:#?}", capabilities);

    let create_key_req = CreateKeyRequest::new(
        opt.b2_account_id, // account_id
        capabilities, // capabilities
        "autogenerate".to_string(), // key_name
        Some(86400_u64), // valid_duration_in_seconds
        Some(main_access.bucket_id.to_string()), // bucket_id
        None, // name_prefix
        None, // options
    );

    debug!("Create key request: {:#?}", create_key_req);

    // CreatedKeyData
    let created_key_data = b2_create_key(
        &api_url,
        &authorization_token,
        &create_key_req,
    ).await;

    debug!("Create key data: {:#?}", created_key_data);

    if let Err(e) = created_key_data {
        let update = update_main_access(pool.clone()).await;
        debug!("Update main access after failed fet: {:#?}", update);

        return Err(ServiceError::BadRequest(e.to_string()))
    };

    let new_auth_data = match created_key_data {
        Ok(ref key_data) => {
            // AuthorizeAccountData
            let auth_account_data = b2_authorize_account(
                &api_url,
                &key_data.application_key_id,
                &key_data.application_key,
            ).await;

            match auth_account_data {
                Ok(auth_data) => Ok(auth_data),
                Err(e) => Err(ServiceError::BadRequest(e.to_string())),
            }
        },
        Err(e) => {
            return Err(ServiceError::BadRequest(e.to_string()))
        },
    };

    debug!("New auth data: {:#?}", new_auth_data);

    match created_key_data  {
        Ok(key_data) => match new_auth_data {
            Ok(auth_data) => {
                let key_expiration_timestamp = key_data.expiration_timestamp.unwrap_or(43200_i64);

                let bucket_id = match key_data.bucket_id {
                    Some(x) => x,
                    None => main_access.bucket_id,
                };

                debug!("Bucket id: {:#?}", bucket_id);

                let naive_add_one_day = chrono::Local::now().naive_local()+chrono::Duration::days(1);

                let new_storage_access = UserStorageAccess{
                    uuid_user: target_user.0,
                    application_key_id: key_data.application_key_id,
                    application_key: key_data.application_key,
                    key_expiration_at: chrono::NaiveDateTime::from_timestamp(key_expiration_timestamp, 0),
                    bucket_id,
                    api_url: auth_data.api_url,
                    authorization_token: auth_data.authorization_token,
                    token_expiration_at: naive_add_one_day,
                };

                // save new user storage access data in database
                UserStorageAccess::new(&new_storage_access, &conn)?;

                Ok(new_storage_access)
            },
            // Failed with new_auth_data (auth_data)
            Err(e) => Err(e),
        },
        // Failed with created_key_data (key_data)
        Err(e) => Err(ServiceError::BadRequest(e.to_string())),
    }
}

// Updates access storage for main user
pub(crate) async fn update_main_access(
    pool: PgConn,
) -> ServiceResult<UserStorageAccess> {
    use crate::storage::wrapper::authorize_account::update_user_token;

    // Gets enviroment variables from `.env`
    dotenv::dotenv().ok();

    // Sets options to enviroment variables
    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    // get user uuid with main access to B2
    let main_user = TargetUser::from(&opt.uuid_main_access_b2);

    // generate new token for main user
    let new_token = update_user_token(main_user, pool).await;
    debug!("Result update main access for B2: {:#?}", new_token);

    Ok(new_token?)
}
