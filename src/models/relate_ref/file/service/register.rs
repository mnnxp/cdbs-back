use crate::errors::err_msg::{get_err_msg, ErrorMessage};
use crate::errors::{ServiceError, ServiceResult};
use crate::models::component::component_modification::relate::{
    file::model::{FileModification, InsertableFileModification},
    fileset_for_program::file::model::{
        InsertableModificationFileFromFileset, ModificationFileFromFileset,
    },
};
use crate::models::component::relate::file::model::{ComponentFile, InsertableComponentFile};
use crate::models::relate_ref::file::{
    model::{InsertableFile, ListObject, PreliminaryFileData, SlimFile},
    util::{get_default_image, parsing_old_file},
};
use crate::models::standard::file::model::{InsertableStandardFile, StandardFile};
use crate::models::supplier_service::file::model::{InsertableServiceFile, ServiceFile};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Preliminary registration a file in database and bind with related object
pub(crate) fn preregister_file(
    logged_user_uuid: &Uuid,
    object: ListObject,
    filename: &str,
    commit_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<SlimFile> {
    let mut preliminary_file_data = PreliminaryFileData::from_ipt_file_data(
        *logged_user_uuid,
        object.clone(),
        filename,
        *commit_uuid,
        conn,
    );
    // check for new revision file
    let _has_parent = parsing_old_file(&mut preliminary_file_data, conn)?;
    // register data in file_ref table
    let value_slim_file_data = write_metadata(preliminary_file_data, conn)?;
    // register data in addiction table (depends on the request)
    write_addiction_data(object, value_slim_file_data.uuid, conn).map_err(|err| {
        debug!("Error recording dependency data: {:?}", err);
        get_err_msg(ErrorMessage::ErrorRecordingDependencyData)
    })?;

    Ok(value_slim_file_data)
}

/// Write information of file to db file_ref
fn write_metadata(
    file_data: PreliminaryFileData,
    conn: &mut PgConnection,
) -> ServiceResult<SlimFile> {
    let file: InsertableFile = file_data.into();
    let res = diesel::insert_into(file_ref::file_ref)
        .values(&file)
        .returning((
            file_ref::uuid,
            file_ref::hash,
            file_ref::sha256_hash,
            file_ref::filename,
            file_ref::filesize,
            file_ref::path_file,
        ))
        .get_result::<SlimFile>(conn)
        .map_err(|err| {
            debug!("Failed insert file row: {:?}", err);
            ServiceError::InternalServerError
        })?;
    // change the parent reference to itself
    if file.parent_file_uuid == get_default_image() {
        let _change_parent =
            diesel::update(file_ref::file_ref.filter(file_ref::uuid.eq(&res.uuid)))
                .set(file_ref::parent_file_uuid.eq(&res.uuid))
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed after insert file row: {:?}", err);
                    ServiceError::InternalServerError
                })?;
    }
    Ok(res)
}

/// Write information of file to db file_to_component or file_to_modification
fn write_addiction_data(
    object: ListObject,
    file_uuid: Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<bool> {
    // select addiction table for write additional data
    match object {
        ListObject::User(_) => Ok(false),
        ListObject::UserCertificate(_) => Ok(false),
        ListObject::CompanyFavicon(_) => Ok(false),
        ListObject::CompanyCertificate(_) => Ok(false),
        ListObject::Component(component_uuid) => {
            use crate::schema::file_to_component::dsl::file_to_component;

            let component_file = InsertableComponentFile {
                file_uuid,
                component_uuid,
            };

            let inserted_component: ComponentFile = diesel::insert_into(file_to_component)
                .values(&component_file)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert file to component row: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!("Select component table, data: {:?} ", &inserted_component);

            Ok(true)
        }
        ListObject::ComponentFavicon(component_uuid) => {
            use crate::schema::component_ref::dsl as component_ref;

            let change_image: usize = diesel::update(component_ref::component_ref)
                .filter(component_ref::uuid.eq(&component_uuid))
                .set(component_ref::image_file_uuid.eq(file_uuid))
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed set favicon for component: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!("Change component main image: {:?} ", &change_image);

            Ok(true)
        }
        ListObject::ComponentModification(modification_uuid) => {
            use crate::schema::file_to_modification::dsl::file_to_modification;

            let modification = InsertableFileModification {
                file_uuid,
                modification_uuid,
            };
            let inserted_modification: FileModification = diesel::insert_into(file_to_modification)
                .values(&modification)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert file to modification row: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!(
                "Select modification table, data: {:?} ",
                &inserted_modification
            );

            Ok(true)
        }
        ListObject::ComponentModificationSet(fileset_uuid) => {
            use crate::schema::modification_file_from_fileset::dsl::modification_file_from_fileset;

            let modification = InsertableModificationFileFromFileset {
                fileset_uuid,
                file_uuid,
            };
            let inserted_file_to_set: ModificationFileFromFileset =
                diesel::insert_into(modification_file_from_fileset)
                    .values(&modification)
                    .get_result(conn)
                    .map_err(|err| {
                        debug!("Failed insert modification: {:?}", err);
                        ServiceError::InternalServerError
                    })?;

            debug!(
                "Select modification table, addiction data: {:?} ",
                &inserted_file_to_set
            );

            Ok(true)
        }
        ListObject::Standard(standard_uuid) => {
            use crate::schema::file_to_standard::dsl::file_to_standard;

            let standard = InsertableStandardFile {
                file_uuid,
                standard_uuid,
            };
            let inserted_standard: StandardFile = diesel::insert_into(file_to_standard)
                .values(&standard)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert file to standard row: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!("Select standard table, data: {:?} ", &inserted_standard);

            Ok(true)
        }
        ListObject::StandardFavicon(standard_uuid) => {
            use crate::schema::standard_ref::dsl as standard_ref;

            let change_image: usize = diesel::update(standard_ref::standard_ref)
                .filter(standard_ref::uuid.eq(&standard_uuid))
                .set(standard_ref::image_file_uuid.eq(file_uuid))
                .execute(conn)
                .map_err(|err| {
                    debug!("Failed set favicon for standard: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!("Change standard main image: {:?} ", &change_image);

            Ok(true)
        }
        ListObject::Service(service_uuid) => {
            use crate::schema::file_to_service::dsl::file_to_service;
            let service = InsertableServiceFile {
                file_uuid,
                service_uuid,
            };
            let inserted_service: ServiceFile = diesel::insert_into(file_to_service)
                .values(&service)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert file to service row: {:?}", err);
                    ServiceError::InternalServerError
                })?;
            debug!("Select service table, data: {:?} ", &inserted_service);
            Ok(true)
        }
        not_match => {
            debug!("Failed write metadata: {:?}", not_match);
            Err(get_err_msg(ErrorMessage::FailedWriteMetadata))
        }
    }
}
