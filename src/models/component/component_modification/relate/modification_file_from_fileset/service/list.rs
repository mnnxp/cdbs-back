use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::modification_file_from_fileset::model::{
    ModificationFileFromFileset, FileOfFileset
};
use crate::models::relate_ref::file::model::ShowFileForDownload;
use crate::models::component::component_modification::fileset_for_program::util::get_component_by_fileset;
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn get_files_of_fileset(
    logged_user_uuid: &Uuid,
    target_fileset_uuid: &Uuid,
    target_file_uuids: &Option<Vec<Uuid>>,
    limit: &i32,
    offset: &i32,
    conn: &PgConnection,
) -> ServiceResult<Vec<FileOfFileset>> {
    use crate::schema::modification_file_from_fileset::dsl::*;

    let need_access_level = 2; // todo!(create enum for manage access level)

    check_access_component_for_user(
        logged_user_uuid,
        &get_component_by_fileset(target_fileset_uuid, conn)?,
        &need_access_level,
        conn
    )?;

    let mut query = modification_file_from_fileset.into_boxed();

    match target_file_uuids {
        Some(file_uuids) if !file_uuids.is_empty() => {
            // add filter for target files
            query = query.filter(fileset_uuid.eq(target_fileset_uuid)
                .and(file_uuid.eq_any(file_uuids)));
        },
        _ => {
            // get all files for fileset
            query = query.filter(fileset_uuid.eq(target_fileset_uuid))
        },
    }

    let files_of_fileset = match query
        .limit(*limit as i64)
        .offset(*offset as i64)
        .load::<ModificationFileFromFileset>(conn) {
        Ok(files_of_fileset) => files_of_fileset,
        Err(err) => {
            debug!("Error get files of fileset: {:?}", err);
            return Err(ServiceError::BadRequest(
                "Error get files of fileset".to_string()
            ))
        },
    };

    let mut show_files_of_fileset: Vec<FileOfFileset> = Vec::new();
    for file_of_set in files_of_fileset {
        show_files_of_fileset.push(
            FileOfFileset {
                fileset_uuid: file_of_set.fileset_uuid,
                file: ShowFileForDownload::get_file_by_uuid(
                    &file_of_set.file_uuid,
                    conn
                )?,
            }
        )
    }

    Ok(show_files_of_fileset)
}
