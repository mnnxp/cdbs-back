use crate::errors::ServiceResult;
use crate::models::component::model::Component;
use crate::models::component::component_modification::model::{
    ComponentModification,
    ComponentModificationWithActualStatus,
    ComponentModificationAndRelatedData,
};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use diesel::prelude::*;

impl ComponentModification {
    pub fn for_component_without_related_data(
        component: &Component,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentModification>> {
        // collect data for modifications the component
        Ok(ComponentModification::belonging_to(component).load::<ComponentModification>(conn)?)
    }
}

impl ComponentModificationWithActualStatus {
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentModificationWithActualStatus>> {
        let mut id_status_modification: Vec<i32> = Vec::new();
        for modification in component_modification.iter() {
            id_status_modification.push(modification.actual_status_id);
        }

        let actual_status_modification: Vec<ActualStatusTranslateList> = ActualStatusTranslateList::get_actual_status_by_vec_id(&id_status_modification, set_lang_id, conn)?;

        // debug!("Component modification actual_status_modification: {:#?}", actual_status_modification);

        let mut component_modification_with_status: Vec<ComponentModificationWithActualStatus> = Vec::new();
        for x in component_modification.iter() {
            for y in actual_status_modification.iter() {
                if x.actual_status_id == y.actual_status_id {
                    let res: ComponentModificationWithActualStatus = (x.clone(),y.clone()).into();
                    component_modification_with_status.push(res)
                }
            }
        }

        Ok(component_modification_with_status)
    }
}


impl ComponentModificationAndRelatedData {
    pub fn for_component_modification_list(
        component_modification: &[ComponentModification],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
        use crate::models::component::component_modification::param::model::ModificationParamWithTranslation;
        use crate::models::component::component_modification::set_of_files_program::model::{
            SetOfFilesProgram,
            SetOfFilesProgramRelatedData,
        };

        // get actual status with translation for list component modification
        let component_modification_with_status: Vec<ComponentModificationWithActualStatus> = ComponentModificationWithActualStatus::for_component_modification_list(
            component_modification,
            set_lang_id,
            conn
        ).expect("Error load component_modification_with_status");

        // get param with translation for component modification
        let param_component_modification_with_translate: Vec<Vec<ModificationParamWithTranslation>> = ModificationParamWithTranslation::for_component_modification_list(
            component_modification,
            set_lang_id,
            conn
        ).expect("Error load param_component_modification_with_translate");

        // debug!("Component modification param_component_modification_with_translate: {:#?}", param_component_modification_with_translate);

        // get sets of files for programs for component modification list
        let set_files_program_with_relate: Vec<Vec<SetOfFilesProgramRelatedData>> = SetOfFilesProgram::for_component_modification_list(
            component_modification,
            conn
        ).expect("Error load set_files_program_with_relate");

        // debug!("Component modification set_files_program_component_modification: {:#?}", set_files_program_component_modification);

        let mut component_modification_with_relate: Vec<ComponentModificationAndRelatedData> = Vec::new();
        for w in component_modification_with_status.iter() {
            let mut vec_values_set: Vec<SetOfFilesProgramRelatedData> = Vec::new();
            for x in set_files_program_with_relate.iter() {
                for y in x.iter() {
                    if w.modification.uuid == y.modification_uuid {
                        vec_values_set.push(y.to_owned())
                    }
                }
            }
            let mut vec_values_param: Vec<ModificationParamWithTranslation> = Vec::new();
            for x in param_component_modification_with_translate.iter() {
                for y in x.iter() {
                    if w.modification.uuid == y.modification_uuid {
                        vec_values_param.push(y.to_owned())
                    }
                }
            }
            component_modification_with_relate.push((
                w.clone(),
                vec_values_set,
                vec_values_param
            ).into())
        }

        Ok(component_modification_with_relate)
    }
}
