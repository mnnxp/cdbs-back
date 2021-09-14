use crate::errors::ServiceResult;
use crate::models::user::model::UserShort;
use crate::models::relate_ref::file::model::{
    ListObject, IptPreliminaryFileData, PreliminaryFileData
};
use crate::models::relate_ref::file as file;
use diesel::PgConnection;
use uuid::Uuid;

pub(crate) fn update_favicon(
    target_user: &Uuid,
    file_data: &IptPreliminaryFileData,
    conn: &PgConnection,
) -> ServiceResult<String> {

    let content_sha1: String = file_data.sha1.clone();

    let user_short = UserShort::get_by_uuid(
        target_user,
        conn
    )?;

    let preliminary_file_data = PreliminaryFileData::from_ipt_preliminary_file_data(
        target_user.to_owned(),
        user_short.image_file_uuid,
        ListObject::User(user_short.uuid),
        file_data.to_owned(),
        conn
    )?;

    let slim_file = file::service::register::register(
        preliminary_file_data,
        conn
    )?;

    let upload_url_data = crate::storage::wrapper::upload::get_url_upload_file(
        target_user,
        conn
    );

    match upload_url_data {
        Ok(upload_url_data) => Ok(
            format!("file_name: {:?}, content_type: {:?}, content_sha1: {:?}, server_side_encryption: {:?}, url: {:?}",
                slim_file.path_file,
                "b2/x-auto".to_string(),
                content_sha1,
                "AES256".to_string(),
                upload_url_data,
            )
        ),
        Err(e) => Err(e),
    }
}
