use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::component_modification::fileset_for_program::model::{
    FilesetProgram, FilesetProgramRelatedData
};
use crate::models::relate_ref::program::model::Program;
use crate::schema::fileset_for_program::dsl as fileset_for_program;
use diesel::prelude::*;
use uuid::Uuid;

impl FilesetProgramRelatedData {
    /// Get filesets by modification uuid
    pub(crate) fn by_modification_uuid(
        component_modification_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        let filesets = fileset_for_program::fileset_for_program
            .filter(fileset_for_program::modification_uuid.eq(component_modification_uuid))
            .load::<FilesetProgram>(conn)
            .map_err(|err| {
                debug!("Failed get fileset for program: {:?}", err);
                ServiceError::InternalServerError
            })?;

        FilesetProgramRelatedData::for_filesets(&filesets, conn)
    }

    /// Get program translate data for fileset
    pub(crate) fn for_fileset(
        filesets: &FilesetProgram,
        conn: &mut PgConnection,
    ) -> ServiceResult<FilesetProgramRelatedData> {
        Ok(FilesetProgramRelatedData{
            uuid: filesets.uuid,
            modification_uuid: filesets.modification_uuid,
            program: Program::get_program_by_id(&filesets.program_id, conn)?,
        })
    }

    /// Get program translate data for filesets list
    pub(crate) fn for_filesets(
        filesets: &[FilesetProgram],
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        let mut result: Vec<FilesetProgramRelatedData> = Vec::new();
        for x in filesets {
            result.push(FilesetProgramRelatedData::for_fileset(x, conn)?);
        }
        // sorting the list of program names alphabetically
        result.sort_by(|a, b| a.program.name.cmp(&b.program.name));
        Ok(result)
    }
}
