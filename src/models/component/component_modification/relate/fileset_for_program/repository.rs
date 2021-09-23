use crate::errors::ServiceResult;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::fileset_for_program::model::{FilesetProgram, FilesetProgramRelatedData};
use crate::models::relate_ref::program::model::Program;
use diesel::prelude::*;

impl FilesetProgramRelatedData {
    /// Find set of files for programs without list files
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Vec<FilesetProgramRelatedData>>> {
        let filesets_program_for_modification: Vec<Vec<FilesetProgram>> = FilesetProgram::belonging_to(component_modification)
            .load::<FilesetProgram>(conn)
            .expect("Error loading filesets_program_for_modification")
            .grouped_by(component_modification);

        // debug!("Component modification filesets_program_for_modification: {:#?}", filesets_program_for_modification);

        let mut program_id_for_set: Vec<i32> = Vec::new();
        for x in filesets_program_for_modification.iter() {
            for y in x.iter() {
                program_id_for_set.push(y.program_id);
            }
        }

        let mut filesets_program_with_relate: Vec<Vec<FilesetProgramRelatedData>> = Vec::new();
        for filesets in filesets_program_for_modification.iter() {
            filesets_program_with_relate.push(
                FilesetProgramRelatedData::for_filesets(filesets, conn)?
            )
        }

        Ok(filesets_program_with_relate)
    }

    /// Gest filesets for program without list files
    /// by modification_uuid with filter program_id
    pub fn for_filesets(
        filesets: &[FilesetProgram],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {

        // get programs ids for filesets component modification
        let mut program_ids_for_set: Vec<i32> = Vec::new();
        for set in filesets {
            program_ids_for_set.push(set.program_id);
        }

        // get program for filesets component modification
        let program_for_filesets: Vec<Program> = Program::get_program_by_vec_id(&program_ids_for_set, conn)?;

        let mut filesets_program_with_relate: Vec<FilesetProgramRelatedData> = Vec::new();
        for x in filesets {
            for y in &program_for_filesets {
                if x.program_id == y.id {
                    let res: FilesetProgramRelatedData = (x.to_owned(),y.clone()).into();
                    filesets_program_with_relate.push(res)
                }
            }
        }

        Ok(filesets_program_with_relate)
    }
}
