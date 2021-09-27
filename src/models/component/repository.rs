use crate::errors::ServiceResult;
use crate::models::component::model::{Component, ShowComponentShort, ComponentAndRelatedData};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::component::param::model::ComponentParamWithTranslation;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::component::spec::model::ComponentSpecWithTranslation;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::component::component_modification::model::{ComponentModification, ComponentModificationAndRelatedData};
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    /// Get component data from component_ref table by uuid
    pub fn get_component_by_uuid(
        target_component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Component> {
        Ok(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid))
            .first::<Component>(conn)?)
    }
}

impl ShowComponentShort {
    pub fn get_list_by_uuids(
        target_uuids_components: &[Uuid],
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // the result for store the result :)
        let mut result: Vec<ShowComponentShort> = Vec::new();

        // collecting data for each component
        for target_component_uuid in target_uuids_components.iter() {
            // get target component
            let component: Component = Component::get_component_by_uuid(
                target_component_uuid,
                conn
            )
            .expect("Error loading component");

            // get component owner
            let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
                &component.user_uuid,
                conn
            ).expect("Error loading slim_user");

            // get component type with translation for component
            let component_type: ComponentTypeTranslateList = ComponentTypeTranslateList::get_component_type_by_id(
                &component.component_type_id,
                set_lang_id,
                conn
            ).expect("Error loading component_type");

            // get actual status with translation for component
            let actual_status: ActualStatusTranslateList = ActualStatusTranslateList::get_actual_status_by_id(
                &component.actual_status_id,
                set_lang_id,
                conn
            ).expect("Error loading actual_status");

            // check whether the object is being tracked auth user
            let is_followed = crate::models::component::component_fav::util::check_subscriber_by_uuid(
                target_component_uuid,
                target_user_uuid,
                conn
            ).expect("Error get is_followed");

            // get licenses for component
            let licenses: Vec<License> = License::get_by_component(
                &component,
                conn
            ).expect("Error loading license");

            // get files for component
            let files = ShowFile::for_component(&component, conn)
                .expect("Error loading component_file");

            // collect data for supplier component
            let component_suppliers: Vec<ComponentSupplierRelatedData> = ComponentSupplierRelatedData::get_first_supplier(
                &component,
                conn
            ).expect("Error loading supplier_component_with_relate");

            result.push(ShowComponentShort {
                uuid: component.uuid,
                name: component.name,
                description: component.description,
                owner_user,
                type_access_id: component.type_access_id,
                component_type,
                actual_status,
                is_followed,
                is_base: component.is_base,
                updated_at: component.updated_at,
                licenses,
                files,
                component_suppliers,
            });
        }
        Ok(result)
    }
}

impl ComponentAndRelatedData {
    /// Collecting component data and related data using uuid
    pub fn collect_related_data(
        target_component_uuid: &Uuid,
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // collect data for component
        let component: Component = Component::get_component_by_uuid(
            target_component_uuid,
            conn
        ).expect("Error loading component");

        // get component owner
        let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
            &component.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // todo!(need make access manager)
        // get component type with translation for component
        // let type_access: TypeAccessTranslateList = TypeAccessTranslateList::get_component_type_by_id(
        //     &component.type_access_id,
        //     set_lang_id,
        //     conn
        // ).expect("Error loading type_access");

        // get component type with translation for component
        let component_type: ComponentTypeTranslateList = ComponentTypeTranslateList::get_component_type_by_id(
            &component.component_type_id,
            set_lang_id,
            conn
        ).expect("Error loading component_type");

        // get actual status with translation for component
        let actual_status: ActualStatusTranslateList = ActualStatusTranslateList::get_actual_status_by_id(
            &component.actual_status_id,
            set_lang_id,
            conn
        ).expect("Error loading actual status");

        // count subscribers component
        let subscribers: i32 = ComponentFav::get_count_followers_by_uuid(&component.uuid, conn)?;

        // check whether the object is being tracked auth user
        let is_followed = crate::models::component::component_fav::util::check_subscriber_by_uuid(
            target_component_uuid,
            target_user_uuid,
            conn
        ).expect("Error get is_followed");

        // get params with translation for component
        let component_params: Vec<ComponentParamWithTranslation> = ComponentParamWithTranslation::for_component(
            &component,
            set_lang_id,
            conn
        ).expect("Error loading params component with translate");

        // get licenses for component
        let licenses: Vec<License> = License::get_by_component(
            &component,
            conn
        ).expect("Error loading license");

        // get files for component
        let files = ShowFile::for_component(&component, conn)
            .expect("Error loading component files");

        // get specs with translation for component
        let component_specs: Vec<ComponentSpecWithTranslation> = ComponentSpecWithTranslation::for_component(
            &component,
            set_lang_id,
            conn
        ).expect("Error loading spec component with translate");

        // get keywords for component
        let component_keywords: Vec<Keyword> = Keyword::get_by_component(
            &component,
            conn
        ).expect("Error loading component keywords");

        // collect data for modifications the component
        let component_modifications: Vec<ComponentModification> = ComponentModification::for_component_without_related_data(
            &component,
            conn
        ).expect("Error loading component modifications");

        // get list component modifications with related data and translation
        let component_modifications: Vec<ComponentModificationAndRelatedData> = ComponentModificationAndRelatedData::for_component_modification_list(
            &component_modifications,
            set_lang_id,
            conn
        ).expect("Error loading component modifications with related data");

        // collect data for supplier component
        let component_suppliers: Vec<ComponentSupplierRelatedData> = ComponentSupplierRelatedData::for_component(
            &component,
            conn
        ).expect("Error loading supplier component with relate");

        // collect data for component standards
        let component_standards: Vec<ShowStandardShort> = ShowStandardShort::for_component(
            target_component_uuid,
            target_user_uuid,
            set_lang_id, 
            conn
        ).expect("Error loading supplier component with relate");


        let result = ComponentAndRelatedData {
            uuid: component.uuid,
            parent_component_uuid: component.parent_component_uuid,
            name: component.name,
            description: component.description,
            owner_user,
            type_access_id: component.type_access_id,
            component_type,
            actual_status,
            is_base: component.is_base,
            subscribers,
            is_followed,
            updated_at: component.updated_at,
            component_params,
            licenses,
            files,
            component_specs,
            component_keywords,
            component_modifications,
            component_suppliers,
            component_standards,
        };

        Ok(result)
    }
}
