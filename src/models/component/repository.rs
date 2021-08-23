use crate::errors::ServiceResult;
use crate::models::component::model::{Component, ComponentAndRelatedData};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::component::param::model::ComponentParamWithTranslation;
use crate::models::component::spec::model::ComponentSpecWithTranslation;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::component::component_modification::model::{
    ComponentModification,
    ComponentModificationAndRelatedData,
};
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    pub fn from_uuid_component(
        target_uuid_component: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Component> {
        Ok(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_uuid_component))
            .first::<Component>(conn)?)
    }
}

impl ComponentAndRelatedData {
    pub fn collect_related_data(
        target_uuid_component: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // collect data for component
        let component: Component = Component::from_uuid_component(
            target_uuid_component,
            conn
        ).expect("Error loading component");

        // get component owner
        let slim_user = crate::models::user::model::SlimUser::from_uuid_user(
            &component.uuid_user,
            conn
        ).expect("Error loading slim_user");

        // get component type with translation for component
        let component_type: ComponentTypeTranslateList = ComponentTypeTranslateList::get_component_type_by_id(
            &component.id_component_type,
            set_id_lang,
            conn
        ).expect("Error loading component_type");

        // get actual status with translation for component
        let actual_status: ActualStatusTranslateList = ActualStatusTranslateList::get_actual_status_by_id(
            &component.id_actual_status,
            set_id_lang,
            conn
        ).expect("Error loading actual_status");

        // get params with translation for component
        let param_component_with_translate: Vec<ComponentParamWithTranslation> = ComponentParamWithTranslation::for_component(
            &component,
            set_id_lang,
            conn
        ).expect("Error loading param_component_with_translate");

        // get licenses for component
        let license: Vec<License> = License::for_component(
            &component,
            conn
        ).expect("Error loading license");

        // get files for component
        let component_file = ShowFile::for_component(&component, conn)
            .expect("Error loading component_file");

        // get specs with translation for component
        let spec_component_with_translate: Vec<ComponentSpecWithTranslation> = ComponentSpecWithTranslation::for_component(
            &component,
            set_id_lang,
            conn
        ).expect("Error loading spec_component_with_translate");

        // get keywords for component
        let keyword_component: Vec<Keyword> = Keyword::for_component(
            &component,
            conn
        ).expect("Error loading keyword_component");

        // collect data for modifications the component
        let component_modification: Vec<ComponentModification> = ComponentModification::for_component_without_related_data(
            &component,
            conn
        ).expect("Error loading component_modification");

        // get list component modifications with related data and translation
        let component_modification_with_relate: Vec<ComponentModificationAndRelatedData> = ComponentModificationAndRelatedData::for_component_modification_list(
            &component_modification,
            set_id_lang,
            conn
        ).expect("Error loading component_modification_with_relate");

        // collect data for supplier component
        let supplier_component_with_relate: Vec<ComponentSupplierRelatedData> = ComponentSupplierRelatedData::for_component(
            &component,
            conn
        ).expect("Error loading supplier_component_with_relate");

        let result = ComponentAndRelatedData {
            uuid: (component.uuid),
            uuid_component_parent: (component.uuid_component_parent),
            name: (component.name),
            description: (component.description),
            slim_user: (slim_user),
            id_type_access: (component.id_type_access),
            component_type: (component_type),
            actual_status: (actual_status),
            is_standard: (component.is_standard),
            updated_at: (component.updated_at),
            param_component: (param_component_with_translate),
            license: (license),
            file: (component_file),
            spec_component: (spec_component_with_translate),
            keyword_component: (keyword_component),
            component_modification: (component_modification_with_relate),
            supplier_component: (supplier_component_with_relate),
        };

        Ok(result)
    }
}
