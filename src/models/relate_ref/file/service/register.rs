use crate::errors::{ServiceError, ServiceResult};
use crate::models::relate_ref::file::model::{
    ListObject,
    File,
    // FileData,
    PreliminaryFileData,
    InsertableFile,
    SlimFile,
};
use crate::models::component::relate::file::model::{
    InsertableComponentFile,
    ComponentFile,
};
use crate::models::component::component_modification::relate::file::model::{
    InsertableFileModification,
    FileModification,
};
use crate::models::component::component_modification::relate::modification_file_from_fileset::model::{
    ModificationFileFromFileset,
    InsertableModificationFileFromFileset,
};
// use crate::models::standard::relate::file::model:{
//     InsertableFileStandard,
//     FileStandard,
// };
// use crate::models::relate_ref::file as file;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn register(
    preliminary_file_data: PreliminaryFileData,
    conn: &PgConnection,
) -> ServiceResult<SlimFile> {
    let object = preliminary_file_data.object.clone();
    // register data in file_ref table
    let value_slim_file_data = match write_metadata(preliminary_file_data, conn) {
        Ok(value) => value,
        Err(err) => {
            debug!("Fail write metadata: {:#?}", err);
            return Err(ServiceError::BadRequest("Fail write metadata".to_string()))
        },
    };
    // register data in addiction table (depends on the request)
    if let Err(err) = write_addiction_data(object, value_slim_file_data.uuid, conn) {
        debug!("Fail write addiction data: {:#?}", err);
        return Err(ServiceError::BadRequest("Fail write addiction data".to_string()))
    }

    Ok(value_slim_file_data)
}

/// Write information of file to db file_ref
pub(crate) fn write_metadata(
    file_data: PreliminaryFileData,
    conn: &PgConnection
) -> ServiceResult<SlimFile> {
    use crate::schema::file_ref::dsl::file_ref;

    let file: InsertableFile = file_data.into();
    let inserted_file: File = diesel::insert_into(file_ref).values(&file).get_result(conn)?;
    Ok(inserted_file.into())
}

/// Write information of file to db file_to_component or file_to_modification
pub(crate) fn write_addiction_data(
    object: ListObject,
    file_uuid: Uuid,
    conn: &PgConnection
) -> ServiceResult<bool>{
    // select addiction table for write additional data
    match object {
        ListObject::User(_) => Ok(false),
        // adding a record to user_certificate_ref table is done in fn add_certificate (../user/../certificate/../add.rs)
        ListObject::UserCertificate(_) => Ok(false),
        // adding a record to company_certificate_ref table is done in fn add_certificate (../company/../certificate/../add.rs)
        ListObject::CompanyCertificate(_) => Ok(false),
        ListObject::Component(component_uuid) => {   // <-- add addiction data in file_to_component
            use crate::schema::file_to_component::dsl::file_to_component;

            let component_file = InsertableComponentFile {
                file_uuid,
                component_uuid,
            };

            let inserted_component: ComponentFile = diesel::insert_into(file_to_component)
                .values(&component_file)
                .get_result(conn)?;

            debug!("Select component table, data: {:?} ", &inserted_component);

            Ok(true)
        },
        ListObject::ComponentModification(modification_uuid) => {   // <- add addiction data in file_to_modification
            use crate::schema::file_to_modification::dsl::file_to_modification;

            let modification =  InsertableFileModification {
                file_uuid,
                modification_uuid,
            };
            let inserted_modification: FileModification = diesel::insert_into(file_to_modification)
                .values(&modification)
                .get_result(conn)?;

            debug!("Select modification table, data: {:?} ", &inserted_modification);

            Ok(true)
        },
        ListObject::ComponentModificationSet(fileset_uuid) => {   // <- add addiction data in modification_file_from_fileset
            use crate::schema::modification_file_from_fileset::dsl::modification_file_from_fileset;

            let modification =  InsertableModificationFileFromFileset {
                fileset_uuid,
                file_uuid,
            };
            let inserted_file_to_set: ModificationFileFromFileset = diesel::insert_into(modification_file_from_fileset)
                .values(&modification)
                .get_result(conn)?;

            debug!("Select modification table, addiction data: {:?} ", &inserted_file_to_set);

            Ok(true)
        }
        // ListObject::Standard(standard_uuid) => {   // <- add addiction data in file_to_standard
        //     use crate::schema::file_to_standard::dsl::file_to_standard;
        //
        //     let standard =  InsertableFileStandard {
        //         file_uuid,
        //         standard_uuid,
        //     };
        //     let inserted_standard: FileStandard = diesel::insert_into(file_to_standard)
        //         .values(&standard)
        //         .get_result(conn)?;
        //
        //     debug!("Select standard table, data: {:?} ", &inserted_standard);
        //
        //     Ok(true)
        // },
        _ => ServiceResult::Err(ServiceError::BadRequest("Error select addiction table".to_string()))?
    }
}
