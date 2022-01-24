use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::{
    ListObject, PreliminaryFileData, InsertableFile, SlimFile,
};
use crate::models::component::relate::file::model::{
    InsertableComponentFile, ComponentFile,
};
use crate::models::component::component_modification::relate::file::model::{
    InsertableFileModification, FileModification,
};
use crate::models::component::component_modification::relate::modification_file_from_fileset::model::{
    ModificationFileFromFileset, InsertableModificationFileFromFileset,
};
use crate::models::standard::file::model::{
    StandardFile, InsertableStandardFile,
};
use crate::schema::file_ref::dsl as file_ref;
use diesel::prelude::*;
use uuid::Uuid;

/// Preliminary registration a file in database and bind with related object
pub(crate) fn preregister_file(
    preliminary_file_data: PreliminaryFileData,
    conn: &PgConnection,
) -> ServiceResult<SlimFile> {
    let object = preliminary_file_data.object.clone();
    // register data in file_ref table
    let value_slim_file_data = write_metadata(preliminary_file_data, conn)?;
    // register data in addiction table (depends on the request)
    write_addiction_data(object, value_slim_file_data.uuid, conn)
        .map_err(|err| {
            debug!("Fail write addiction data: {:?}", err);
            ServiceError::BadRequest("Fail write addiction data".to_string())
        })?;

    Ok(value_slim_file_data)
}

/// Write information of file to db file_ref
fn write_metadata(
    file_data: PreliminaryFileData,
    conn: &PgConnection
) -> ServiceResult<SlimFile> {
    let file: InsertableFile = file_data.into();
    diesel::insert_into(file_ref::file_ref)
        .values(&file)
        .returning((
            file_ref::uuid,
            file_ref::filename,
            file_ref::filesize,
            file_ref::path_file,
        ))
        .get_result::<SlimFile>(conn)
        .map_err(|err| {
            debug!("Failed insert file row: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Write information of file to db file_to_component or file_to_modification
fn write_addiction_data(
    object: ListObject,
    file_uuid: Uuid,
    conn: &PgConnection
) -> ServiceResult<bool>{
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
        },
        ListObject::ComponentModification(modification_uuid) => {
            use crate::schema::file_to_modification::dsl::file_to_modification;

            let modification =  InsertableFileModification {
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

            debug!("Select modification table, data: {:?} ", &inserted_modification);

            Ok(true)
        },
        ListObject::ComponentModificationSet(fileset_uuid) => {
            use crate::schema::modification_file_from_fileset::dsl::modification_file_from_fileset;

            let modification =  InsertableModificationFileFromFileset {
                fileset_uuid,
                file_uuid,
            };
            let inserted_file_to_set: ModificationFileFromFileset = diesel::insert_into(modification_file_from_fileset)
                .values(&modification)
                .get_result(conn)
                .map_err(|err| {
                    debug!("Failed insert modification: {:?}", err);
                    ServiceError::InternalServerError
                })?;

            debug!("Select modification table, addiction data: {:?} ", &inserted_file_to_set);

            Ok(true)
        },
        ListObject::Standard(standard_uuid) => {
            use crate::schema::file_to_standard::dsl::file_to_standard;

            let standard =  InsertableStandardFile {
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
        },
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
        },
        not_match => {
            debug!("Failed write metadata: {:?}", not_match);

            Err(ServiceError::BadRequest("Failed write metadata".to_string()))
        },
    }
}
