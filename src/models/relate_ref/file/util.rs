use crate::errors::{ServiceResult, ServiceError};
use crate::errors::err_msg::{ErrorMessage, get_err_msg};
use crate::models::component::file::repository::{
    get_file_uuids_by_component_uuid, get_component_uuid_by_file_uuid
};
use crate::models::component::component_modification::{
    file::repository::{get_file_uuids_by_modification_uuid, get_modification_uuid_by_file_uuid},
    fileset_for_program::file::repository::{get_file_uuids_by_fileset_uuid, get_fileset_uuid_by_file_uuid},
};
use crate::models::standard::file::repository::{
    get_file_uuids_by_standard_uuid, get_standard_uuid_by_file_uuid
};
use crate::models::supplier_service::file::repository::{
    get_file_uuids_by_service_uuid, get_service_uuid_by_file_uuid
};
use regex::Regex;
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::file_ref::dsl as file_ref;
use super::model::{ListObject, PreliminaryFileData};

lazy_static::lazy_static! {
    static ref DEFAULT_IMAGE_UUID : Uuid =
        Uuid::parse_str("bc1c2151-86d0-4656-9c9d-d016dd584297")
            .expect("Set default image uuid failed!");
}

/// Retund default image uuid
pub(crate) fn get_default_image() -> Uuid {
    *DEFAULT_IMAGE_UUID
}

/// Check default file by uuid
pub(crate) fn check_default_file(file_uuid: &Uuid) -> bool {
    let defalt_uuid = *DEFAULT_IMAGE_UUID;
    &defalt_uuid == file_uuid
}

/// Find extension id on table for file extension
pub(crate) fn find_id_ext(
    filename: &str,
    conn: &mut PgConnection
) -> i32 {
    use crate::schema::extension_ref::dsl::*;
    // debug!("Filename_str {:?}", filename);
    let ext_str =
        Regex::new(r"\.\w+$")
            .unwrap()
            .find(filename)
            .map(|m| m.as_str())
            .unwrap_or_default();
    // debug!("Ext_str {:?}", ext_str);
    if ext_str.is_empty() {
        return 1
    }
    // find id extension or set not found id = 1
    extension_ref
        .filter(extension.eq(ext_str))
        .select(id)
        .first::<i32>(conn).unwrap_or(1)
}

/// Checking that the file name matches the image
pub(crate) fn check_image_filename(filename: &str) -> bool {
    let ext_str = Regex::new(r"\.\w+$").unwrap().find(filename).unwrap().as_str();

    matches!(
        ext_str.to_lowercase().as_str(),
        ".apng" | ".avif" | ".gif" |
        ".jpg" | ".jpeg" | ".jpe" |
        ".jif" | ".jfif" | ".png" |
        ".svg" | ".webp"
    )
}

/// Checking for a file with the same name for the same object.
/// Returns true if such a file was found and info about a new file has been updated.
pub(crate) fn parsing_old_file(
    preliminary_file_data: &mut PreliminaryFileData,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    let files_for_object_uuids = collect_file_uuids_of_object(&preliminary_file_data.object, conn)?;
    let old_revision_file =
        get_top_revision_by_name(files_for_object_uuids, &preliminary_file_data.filename, conn)?;

    // check if a previous version of the file is found
    if let Some((parent_uuid, parent_revision)) = old_revision_file {
        // set parent uuid and add next revision number
        preliminary_file_data.set_revision(
            parent_uuid,
            parent_revision + 1
        );
        return Ok(true)
    }
    Ok(false)
}

/// Collecting UUID from files associated with an object.
/// accepted objects: component, modification, modification fileset, standard.
fn collect_file_uuids_of_object(
    object: &ListObject,
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    match object {
        ListObject::Component(component_uuid) => {
            get_file_uuids_by_component_uuid(component_uuid, &[], conn)
        },
        ListObject::ComponentModification(modification_uuid) => {
            get_file_uuids_by_modification_uuid(modification_uuid, &[], conn)
        },
        ListObject::ComponentModificationSet(fileset_uuid) => {
            get_file_uuids_by_fileset_uuid(fileset_uuid, &[], conn)
        },
        ListObject::Standard(standard_uuid) => {
            get_file_uuids_by_standard_uuid(standard_uuid, &[], conn)
        },
        ListObject::Service(service_uuid) => {
            get_file_uuids_by_service_uuid(service_uuid, &[], conn)
        },
        _not_match => {
            debug!("This file does not require versioning");
            Ok(Vec::new())
        },
    }
}

/// Finding of file with same name among the target files,
/// returns one pair with UUID and version number of upper version file.
fn get_top_revision_by_name(
    files_for_object_uuids: Vec<Uuid>,
    filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Option<(Uuid, i32)>> {
    file_ref::file_ref
        .select((
            file_ref::uuid,
            file_ref::revision,
        ))
        .filter(file_ref::uuid.eq_any(files_for_object_uuids)
            .and(file_ref::filename.eq(filename)
            // .and(file_ref::is_checked.eq(true)
            .and(file_ref::is_delete.eq(false))))
        .order(file_ref::revision.desc())
        .first::<(Uuid, i32)>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed to get old file version: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Returns UUID and revision number of the latest version of a file
pub(crate) fn get_active_file_revision(
    object: &ListObject,
    filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Option<(Uuid, i32)>> {
    let files_for_object_uuids = collect_file_uuids_of_object(object, conn)?;
    get_top_active_revision_by_name(files_for_object_uuids, filename, conn)
}

/// Finding of file with same name among the target files, with check no hidden,
/// returns one pair with UUID and version number of upper version file.
fn get_top_active_revision_by_name(
    files_for_object_uuids: Vec<Uuid>,
    filename: &str,
    conn: &mut PgConnection,
) -> ServiceResult<Option<(Uuid, i32)>> {
    file_ref::file_ref
        .select((
            file_ref::uuid,
            file_ref::revision,
        ))
        .filter(file_ref::uuid.eq_any(files_for_object_uuids)
            .and(file_ref::filename.eq(filename)
            .and(file_ref::is_hidden.eq(false)
            .and(file_ref::is_delete.eq(false)))))
        .order(file_ref::revision.desc())
        .first::<(Uuid, i32)>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed to get old file version: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Determines the relationship of a file to an object by file UUID
pub(crate) fn detect_relation_to_object(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<ListObject> {

    if let Some(component_uuid) = get_component_uuid_by_file_uuid(file_uuid, conn)? {
        return Ok(ListObject::Component(component_uuid))
    };
    if let Some(modification_uuid) = get_modification_uuid_by_file_uuid(file_uuid, conn)? {
        return Ok(ListObject::ComponentModification(modification_uuid))
    };
    if let Some(fileset_uuid) = get_fileset_uuid_by_file_uuid(file_uuid, conn)? {
        return Ok(ListObject::ComponentModificationSet(fileset_uuid))
    };
    if let Some(standard_uuid) = get_standard_uuid_by_file_uuid(file_uuid, conn)? {
        return Ok(ListObject::Standard(standard_uuid))
    };
    if let Some(service_uuid) = get_service_uuid_by_file_uuid(file_uuid, conn)? {
        return Ok(ListObject::Service(service_uuid))
    };

    debug!("This file does not require versioning");
    Err(get_err_msg(ErrorMessage::FileDoedNotSupportVersioning))
}

/// Sets is_hidden value for file by UUID
pub(crate) fn set_hidden_flag(
    file_uuid: &Uuid,
    set_flag: bool,
    conn: &mut PgConnection
) -> ServiceResult<bool> {
    diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(file_uuid)))
        .set((
            file_ref::is_hidden.eq(set_flag),
            file_ref::updated_at.eq(chrono::Local::now().naive_local())
        ))
        .execute(conn)
        .map(|changes| changes == 1)
        .map_err(|err| {
            debug!("Failed set flag: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Sets is_hidden=true for old revisions by file uuid and filename
pub(crate) fn set_hidden_flag_revisions(
    file_uuid: &Uuid,
    filename: &str,
    conn: &mut PgConnection
) -> ServiceResult<usize> {
    let list_object = detect_relation_to_object(file_uuid, conn)?;
    let files_for_object_uuids = collect_file_uuids_of_object(&list_object, conn)?;
    diesel::update(file_ref::file_ref
        .filter(file_ref::uuid.eq_any(files_for_object_uuids)
            .and(file_ref::uuid.ne(file_uuid)
            .and(file_ref::filename.eq(filename)
            .and(file_ref::is_hidden.eq(false)
            .and(file_ref::is_delete.eq(false)))))))
        .set((
            file_ref::is_hidden.eq(true),
            file_ref::updated_at.eq(chrono::Local::now().naive_local())
        ))
        .execute(conn)
        .map_err(|err| {
            debug!("Failed set flag: {:?}", err);
            ServiceError::InternalServerError
        })
}