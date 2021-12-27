use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::{
    actual_status::model::ActualStatusTranslateList,
    component_modification::{
        model::{ComponentModification, ComponentModificationAndRelatedData, ComponentModificationArg},
        param::model::ModificationParamWithTranslation,
        fileset_for_program::model::FilesetProgramRelatedData,
    },
};
use crate::schema::component_modification_list::dsl as component_modification_list;
use diesel::prelude::*;

impl ComponentModification {
    pub(crate) fn by_args(
        args: &ComponentModificationArg,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentModification>> {
        // collect data for modifications the component
        component_modification_list::component_modification_list
            .filter(component_modification_list::component_uuid.eq(&args.component_uuid))
            .limit(args.limit as i64)
            .offset(args.offset as i64)
            .load::<ComponentModification>(conn)
            .map_err(|err| {
                debug!("Failed get component modification: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ComponentModificationAndRelatedData {
    /// Get related data for component modification
    pub(crate) fn for_modification(
        component_modification: &ComponentModification,
        set_lang_id: &i32,
        conn: &PgConnection
    ) -> ServiceResult<ComponentModificationAndRelatedData> {
        let mut data = ComponentModificationAndRelatedData::new(component_modification);

        // set actual status with translation for list component modification
        data.put_actual_status(&ActualStatusTranslateList::get_by_id(
            &component_modification.actual_status_id,
            set_lang_id,
            conn
        )?);

        // get sets of files for programs for component modification list
        data.put_fileset_program(FilesetProgramRelatedData::by_modification_uuid(
            &component_modification.uuid,
            conn
        )?);

        // get sets of files for programs for component modification list
        data.put_modification_params(ModificationParamWithTranslation::for_modificaiton(
            component_modification,
            set_lang_id,
            conn
        )?);

        Ok(data)
    }

    pub(crate) fn for_modifications(
        component_modifications: &[ComponentModification],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
        let mut result: Vec<ComponentModificationAndRelatedData> = Vec::new();
        for x in component_modifications.iter() {
            result.push(ComponentModificationAndRelatedData::for_modification(x, set_lang_id, conn)?);
        }
        Ok(result)
    }
}
