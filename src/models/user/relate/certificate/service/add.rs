use crate::errors::{
    // ServiceError,
    ServiceResult
};
use crate::database::PooledConnection;
use crate::models::user::certificate::model::{
    UserCertificate,
    IptUserCertificateData,
    InsertableUserCertificate,
};
// use crate::models::user::model::TargetUser;
use crate::models::relate_ref::file::model::{
    ListObject, IptPreliminaryFileData, PreliminaryFileData
};
use crate::models::relate_ref::file as file;
use crate::schema::user_certificate_ref::dsl::*;
use crate::storage::model::UserStorageAccess;
use crate::storage::wrapper::presigned_url::upload_presigned_url;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn add_certificate(
    target_user_uuid: Uuid,
    cert_data: IptUserCertificateData,
    file_data: IptPreliminaryFileData,
    conn: &PooledConnection,
) -> ServiceResult<String> {
    // let pool = pool.clone();
    // let conn = pool.get().unwrap();

    // let content_sha1: String = file_data.sha1.clone();

    let preliminary_file_data = PreliminaryFileData::from_ipt_preliminary_file_data(
        target_user_uuid,
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")?, // <-- todo!(get uuid default file)
        ListObject::UserCertificate(target_user_uuid),
        file_data,
        conn
    )?;

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    let new_user_certificate = InsertableUserCertificate{
        file_uuid: slim_file.uuid,
        user_uuid: target_user_uuid,
        description: cert_data.description,
    };

    // debug!("fn create_favorite START SEARCH ={:?}", flag_found_favorite);

    let user_inserted_certificate: UserCertificate = diesel::insert_into(user_certificate_ref)
        .values(new_user_certificate)
        .get_result(conn)?;

    debug!("User inserted certificate: {:?}", user_inserted_certificate);

    // todo!(presigned_url)
    upload_presigned_url(
        &UserStorageAccess::get(
            &target_user_uuid,
            conn
        )?,
        &slim_file.path_file,
        // &content_sha1,
    )
}
