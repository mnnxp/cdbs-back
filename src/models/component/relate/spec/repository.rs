use crate::errors::{ServiceResult, ServiceError};
use crate::models::component::model::Component;
use crate::models::component::spec::model::{ComponentSpec, ComponentSpecsArg};
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::schema::spec_to_component::dsl as spec_to_component;
use diesel::prelude::*;
// use uuid::Uuid;

impl SpecTranslateList {
    /// Gets all specs for component by uuid
    pub(crate) fn for_component_by_uuid(
        arg: &ComponentSpecsArg,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let ComponentSpecsArg {
            component_uuid,
            limit,
            offset,
        } = arg;

        let specs_ids = spec_to_component::spec_to_component
            .filter(spec_to_component::component_uuid.eq(component_uuid))
            .select(spec_to_component::spec_id)
            .limit(*limit as i64)
            .offset(*offset as i64)
            .load::<i32>(conn)
            .map_err(|err| {
                debug!("Failed get specs for component: {:?}", err);
                ServiceError::InternalServerError
            })?;

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for component
        SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        ).map_err(|err| {
            debug!("Failed get specs for component: {:?}", err);
            ServiceError::InternalServerError
        })
    }

    /// Gets all specs for component
    pub(crate) fn for_component(
        component: &Component,
        set_lang_id: &i32,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        let spec_component: Vec<ComponentSpec> = ComponentSpec::belonging_to(component)
            .load::<ComponentSpec>(conn)
            .expect("Error loading spec_component");

        // get specs for component
        let mut specs_ids: Vec<i32> = Vec::new();
        for spec in spec_component.iter() {
            specs_ids.push(spec.spec_id);
        }

        if specs_ids.is_empty() {
            return Ok(Vec::new()) // not found specs
        }

        // get specs with translation for component
        SpecTranslateList::get_by_ids(
            &specs_ids,
            &100,
            &0,
            set_lang_id,
            conn
        ).map_err(|err| {
            debug!("Failed get specs for component: {:?}", err);
            ServiceError::InternalServerError
        })
    }
}
