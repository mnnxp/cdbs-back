use crate::errors::ServiceResult;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::set_of_files_program::model::{SetOfFilesProgram, SetOfFilesProgramRelatedData};
use crate::models::relate_ref::program::model::Program;
use diesel::prelude::*;

impl SetOfFilesProgram {
    /// Find set of files for programs without list files
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Vec<SetOfFilesProgramRelatedData>>> {
        let set_files_program_for_modification: Vec<Vec<SetOfFilesProgram>> = SetOfFilesProgram::belonging_to(component_modification)
            .load::<SetOfFilesProgram>(conn)
            .expect("Error loading set_files_program_for_modification")
            .grouped_by(component_modification);

        // debug!("Component modification set_files_program_for_modification: {:#?}", set_files_program_for_modification);

        let mut program_id_for_set: Vec<i32> = Vec::new();
        for x in set_files_program_for_modification.iter() {
            for y in x.iter() {
                program_id_for_set.push(y.program_id);
            }
        }

        // get program for set files component modification
        let program_for_set_files: Vec<Program> = Program::get_program_by_vec_id(&program_id_for_set, conn)?;

        let mut set_files_program_with_relate: Vec<Vec<SetOfFilesProgramRelatedData>> = Vec::new();
        for w in set_files_program_for_modification.iter() {
            for x in w.iter() {
                let mut vec_values: Vec<SetOfFilesProgramRelatedData> = Vec::new();
                for y in program_for_set_files.iter() {
                    if x.program_id == y.id {
                        let res: SetOfFilesProgramRelatedData = (x.to_owned(),y.clone()).into();
                        vec_values.push(res)
                    }
                }
                set_files_program_with_relate.push(vec_values)
            }
        }

        Ok(set_files_program_with_relate)
    }
}
