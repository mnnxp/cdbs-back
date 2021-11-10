use crate::errors::ServiceResult;
use crate::models::standard::model::ShowStandardShort;
use diesel::prelude::*;
use uuid::Uuid;

impl ShowStandardShort {
    pub(crate) fn for_component(
        target_component_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // search all standards for component
        let select_standards_uuids: Vec<Uuid> = get_standards_uuids_by_component_uuid(
            target_component_uuid,
            conn,
        );

        // return empty vec if not found standard for component_type
        if select_standards_uuids.is_empty() {
            debug!("Not found standards for compont");
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

fn get_standards_uuids_by_component_uuid(
    target_component_uuid: &Uuid,
    conn: &PgConnection,
) -> Vec<Uuid> {
    use crate::schema::standard_to_component::dsl::*;

    let res_standards_uuids = standard_to_component
        .filter(component_uuid.eq(target_component_uuid))
        .select(standard_uuid)
        .load::<Uuid>(conn);

    match res_standards_uuids {
        Ok(res) => res,
        Err(err) => {
            debug!("Fail getting standards for the component: {:?}", err);
            Vec::new()
        },
    }
}
