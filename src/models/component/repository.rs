use crate::errors::ServiceResult;
use crate::models::component::model::{Component, ShowComponentShort, ComponentAndRelatedData};
use crate::models::component::actual_status::model::ActualStatusTranslateList;
use crate::models::component::component_type::model::ComponentTypeTranslateList;
use crate::models::component::param::model::ComponentParamWithTranslation;
use crate::models::component::component_fav::model::ComponentFav;
use crate::models::component::spec::model::ComponentSpecWithTranslation;
use crate::models::component::supplier::model::ComponentSupplierRelatedData;
use crate::models::component::component_modification::model::{ComponentModification, ComponentModificationAndRelatedData};
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::file::model::ShowFile;
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    /// Get component data from component_ref table by uuid
    pub fn get_component_by_uuid(
        target_uuid_component: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Component> {
        Ok(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_uuid_component))
            .first::<Component>(conn)?)
    }
}

impl ShowComponentShort {
    pub fn get_list_by_uuids(
        target_uuids_components: &[Uuid],
        target_uuid_user: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // the result for store the result :)
        let mut result: Vec<ShowComponentShort> = Vec::new();

        // collecting data for each component
        for target_uuid_component in target_uuids_components.iter() {
            // get target component
            let component: Component = Component::get_component_by_uuid(
                target_uuid_component,
                conn
            )
            .expect("Error loading component");

            // get component owner
            let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
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

            // check whether the object is being tracked auth user
            let is_followed = crate::models::component::component_fav::util::check_subscriber_by_uuid(
                target_uuid_component,
                target_uuid_user,
                conn
            ).expect("Error get is_followed");

            // get licenses for component
            let licenses: Vec<License> = License::get_by_component(
                &component,
                conn
            ).expect("Error loading license");

            // get files for component
            let component_files = ShowFile::for_component(&component, conn)
                .expect("Error loading component_file");

            // collect data for supplier component
            let component_suppliers_with_related_data: Vec<ComponentSupplierRelatedData> = ComponentSupplierRelatedData::get_first_supplier(
                &component,
                conn
            ).expect("Error loading supplier_component_with_relate");

            result.push(ShowComponentShort {
                uuid: component.uuid,
                name: component.name,
                description: component.description,
                owner_user,
                id_type_access: component.id_type_access,
                component_type,
                actual_status,
                is_followed,
                is_standard: component.is_standard,
                updated_at: component.updated_at,
                licenses,
                files: component_files,
                component_suppliers: component_suppliers_with_related_data,
            });
        }
        Ok(result)
    }
}

impl ComponentAndRelatedData {
    /// Collecting component data and related data using uuid
    pub fn collect_related_data(
        target_uuid_component: &Uuid,
        set_id_lang: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // collect data for component
        let component: Component = Component::get_component_by_uuid(
            target_uuid_component,
            conn
        ).expect("Error loading component");

        // get component owner
        let owner_user = crate::models::user::model::ShowUserShort::get_by_uuid(
            &component.uuid_user,
            conn
        ).expect("Error loading slim_user");

        // todo!(need make access manager)
        // get component type with translation for component
        // let type_access: TypeAccessTranslateList = TypeAccessTranslateList::get_component_type_by_id(
        //     &component.id_type_access,
        //     set_id_lang,
        //     conn
        // ).expect("Error loading type_access");

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
        ).expect("Error loading actual status");

        // count subscribers component
        let component_subscribers_count: i32 = ComponentFav::get_count_followers_by_uuid(&component.uuid, conn)?;

        // get params with translation for component
        let params_component_with_translate: Vec<ComponentParamWithTranslation> = ComponentParamWithTranslation::for_component(
            &component,
            set_id_lang,
            conn
        ).expect("Error loading params component with translate");

        // get licenses for component
        let licenses: Vec<License> = License::get_by_component(
            &component,
            conn
        ).expect("Error loading license");

        // get files for component
        let component_files = ShowFile::for_component(&component, conn)
            .expect("Error loading component files");

        // get specs with translation for component
        let component_specs_with_translate: Vec<ComponentSpecWithTranslation> = ComponentSpecWithTranslation::for_component(
            &component,
            set_id_lang,
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
        let component_modifications_with_related_data: Vec<ComponentModificationAndRelatedData> = ComponentModificationAndRelatedData::for_component_modification_list(
            &component_modifications,
            set_id_lang,
            conn
        ).expect("Error loading component modifications with related data");

        // collect data for supplier component
        let component_suppliers_with_related_data: Vec<ComponentSupplierRelatedData> = ComponentSupplierRelatedData::for_component(
            &component,
            conn
        ).expect("Error loading supplier component with relate");

        let result = ComponentAndRelatedData {
            uuid: (component.uuid),
            uuid_component_parent: (component.uuid_component_parent),
            name: (component.name),
            description: (component.description),
            owner_user: (owner_user),
            id_type_access: (component.id_type_access),
            component_type: (component_type),
            actual_status: (actual_status),
            is_standard: (component.is_standard),
            subscribers: component_subscribers_count,
            updated_at: (component.updated_at),
            component_params: (params_component_with_translate),
            licenses: (licenses),
            files: (component_files),
            component_specs: (component_specs_with_translate),
            component_keywords: (component_keywords),
            component_modifications: (component_modifications_with_related_data),
            component_suppliers: (component_suppliers_with_related_data),
        };

        Ok(result)
    }
}
