use crate::errors::{ServiceResult, ServiceError};
use crate::models::standard::model::ShowStandardShort;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowStandardShort {
    pub(crate) fn for_component(
        target_component_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        use crate::schema::standard_to_component::dsl::*;

        // search all standards for component
        let select_standards_uuids: Vec<Uuid> = standard_to_component
            .filter(component_uuid.eq(target_component_uuid))
            .select(standard_uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Fail getting standards for the component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // return empty vec if not found standard for component_type
        if select_standards_uuids.is_empty() {
            debug!("Not found standards for component");
            return Ok(Vec::new())
        }

        // collecting standards for component
        ShowStandardShort::get_list_by_uuids(
            &select_standards_uuids,
            target_user_uuid,
            set_lang_id,
            conn,
        )
    }
}
