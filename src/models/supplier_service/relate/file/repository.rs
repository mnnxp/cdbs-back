use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::file::ShowFileRelatedData;
use crate::models::component::util::get_files_by_ext;
use crate::models::relate_ref::file::model::{DownloadFile, FileByExtArg};
use crate::models::search::order::{Paginate, Sort};
use crate::schema::file_to_service::dsl as file_to_service;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    /// Gets all files for modification fileset by uuid without check for hide, delete etc
    pub(crate) fn by_service_uuid(
        service_uuid: &Uuid,
        sort: &Sort,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let object_uuids = get_file_uuids_by_service_uuid(service_uuid, &[], conn)?;
        ShowFileRelatedData::get_file_by_uuids(&object_uuids, sort, paginate, domain, conn)
    }
}

/// Returns an array of UUIDs of files relate with target service
pub(crate) fn get_file_uuids_by_service_uuid(
    service_uuid: &Uuid,
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = file_to_service::file_to_service.into_boxed();

    query = match file_uuids.is_empty() {
        true => query.filter(file_to_service::service_uuid.eq(service_uuid)),
        false => query.filter(
            file_to_service::service_uuid
                .eq(service_uuid)
                .and(file_to_service::file_uuid.eq_any(file_uuids)),
        ),
    };

    query
        .select(file_to_service::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files for service: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Determines a service UUID by a file UUID
pub(crate) fn get_service_uuid_by_file_uuid(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<Uuid>> {
    file_to_service::file_to_service
        .select(file_to_service::service_uuid)
        .filter(file_to_service::file_uuid.eq(file_uuid))
        .first::<Uuid>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed get service uuid by file uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}

impl DownloadFile {
    pub(crate) fn service_image_files(
        service_uuid: &Uuid,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        match get_files_by_ext(service_uuid, &FileByExtArg::image(), conn) {
            Ok(image_uuids) => DownloadFile::get_by_file_uuids(&image_uuids, paginate, domain, conn),
            Err(err) => {
                debug!("Error get files by ext: {}", err);
                Ok(Vec::new())
            }
        }
    }

    /// Gets all files for modification fileset by uuid without check for hide, delete etc
    pub(crate) fn by_service_uuid(
        service_uuid: &Uuid,
        paginate: &Paginate,
        domain: &str,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let target_file_uuids: Vec<Uuid> = get_file_uuids_by_service_uuid(service_uuid, &[], conn)?;
        if target_file_uuids.is_empty() {
            return Ok(Vec::new());
        }
        DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, domain, conn)
    }
}
