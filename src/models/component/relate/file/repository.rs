use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::util::get_files_by_ext;
use crate::models::relate_ref::file::model::{
    ShowFileRelatedData, DownloadFile, FileByExtArg
};
use crate::models::search::order::{Paginate, Sort};
use crate::schema::file_to_component::dsl as file_to_component;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowFileRelatedData {
    /// Gets all files for modification fileset by uuid without check for hide, delete etc
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let object_uuids = get_file_uuids_by_component_uuid(component_uuid, &[], conn)?;
        ShowFileRelatedData::get_file_by_uuids(&object_uuids, sort, paginate, conn)
    }
}

/// Returns an array of UUIDs of files relate with target component
pub(crate) fn get_file_uuids_by_component_uuid(
    component_uuid: &Uuid,
    file_uuids: &[Uuid],
    conn: &mut PgConnection,
) -> ServiceResult<Vec<Uuid>> {
    let mut query = file_to_component::file_to_component.into_boxed();

    query = match file_uuids.is_empty() {
        true => query.filter(file_to_component::component_uuid.eq(component_uuid)),
        false => query.filter(file_to_component::component_uuid.eq(component_uuid)
            .and(file_to_component::file_uuid.eq_any(file_uuids)))
    };

    query
        .select(file_to_component::file_uuid)
        .load::<Uuid>(conn)
        .map_err(|err| {
            debug!("Failed get files for component: {:?}", err);
            ServiceError::InternalServerError
        })
}

/// Determines a component UUID by a file UUID
pub(crate) fn get_component_uuid_by_file_uuid(
    file_uuid: &Uuid,
    conn: &mut PgConnection,
) -> ServiceResult<Option<Uuid>> {
    file_to_component::file_to_component
        .select(file_to_component::component_uuid)
        .filter(file_to_component::file_uuid.eq(file_uuid))
        .first::<Uuid>(conn)
        .optional()
        .map_err(|err| {
            debug!("Failed get component uuid by file uuid: {:?}", err);
            ServiceError::InternalServerError
        })
}

impl DownloadFile {
    pub(crate) fn component_image_files(
        component_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        match get_files_by_ext(component_uuid, &FileByExtArg::image(), conn) {
            Ok(image_uuids) =>
                DownloadFile::get_by_file_uuids(&image_uuids, paginate, conn),
            Err(err) => {
                debug!("Error get files by ext: {}", err);
                Ok(Vec::new())
            },
        }
    }

    /// Gets all files for modification fileset by uuid without check for hide, delete etc
    pub(crate) fn by_component_uuid(
        component_uuid: &Uuid,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<DownloadFile>> {
        let target_file_uuids: Vec<Uuid> = get_file_uuids_by_component_uuid(component_uuid, &[], conn)?;
        if target_file_uuids.is_empty() {
            return Ok(Vec::new())
        }
        DownloadFile::get_by_file_uuids(&target_file_uuids, paginate, conn)
    }
}