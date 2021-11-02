use crate::errors::{ServiceResult, ServiceError};
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
use crate::models::relate_ref::file::model::ShowFileForDownload;
use crate::schema::component_ref::dsl as component_ref;
use crate::models::component::access::util::check_access_component_for_user;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    /// Get component data from component_ref table by uuid
    pub(crate) fn get_component_by_uuid(
        target_component_uuid: &Uuid,
        conn: &PgConnection,
    ) -> ServiceResult<Component> {
        Ok(component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid))
            .first::<Component>(conn)?)
    }

    /// Filter components uuid for has access authorized user
    pub(crate) fn clear_uuids_without_access(
        logged_user_uuid: &Uuid,
        filter_components_uuids: &[Uuid],
        need_access_level: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Uuid>> {
        if filter_components_uuids.is_empty() {
            return Ok(Vec::new());
        }

        let mut res_comp_uuids: Vec<Uuid> = Vec::new();

        for tcu in filter_components_uuids {
            match check_access_component_for_user(
                logged_user_uuid,
                tcu,
                need_access_level,
                conn
            ) {
                Ok(true) => res_comp_uuids.push(*tcu),
                Ok(false) => debug!("Without access skip: {:?}", tcu),
                Err(err) => {
                    debug!("Without access skip: {:?}", tcu);
                    debug!("Error with check uuid access: {:?}", err);
                },
            }
        }

        match res_comp_uuids.is_empty() {
            true => {
                Err(ServiceError::BadRequest(
                    "Access denied".to_string()
                ))
            },
            false => Ok(res_comp_uuids),
        }
    }

    /// Search all components uuids by target user (owner)
    pub(crate) fn get_uuids_by_user(
        target_user_uuid: &Uuid,
        filter_components_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Uuid>> {
        if filter_components_uuids.is_empty() {
            component_ref::component_ref
                .filter(component_ref::user_uuid.eq(target_user_uuid))
                .select(component_ref::uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        } else {
            component_ref::component_ref
                .filter(component_ref::user_uuid.eq(target_user_uuid)
                .and(component_ref::uuid.eq_any(filter_components_uuids)))
                .select(component_ref::uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }

    /// Search favorite components uuids with filter by user uuid
    pub(crate) fn get_fav_list_uuids_by_user(
        target_user_uuid: &Uuid,
        filter_components_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Uuid>> {
        use crate::schema::component_fav::dsl as component_fav;

        if filter_components_uuids.is_empty() {
            component_fav::component_fav
                .filter(component_fav::user_uuid.eq(target_user_uuid)
                .and(component_fav::is_enabled.eq(true)))
                .select(component_fav::component_uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        } else {
            component_fav::component_fav
                .filter(component_fav::user_uuid.eq(target_user_uuid)
                .and(component_fav::is_enabled.eq(true))
                .and(component_fav::component_uuid.eq_any(filter_components_uuids)))
                .select(component_fav::component_uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }

    /// Search company components uuids with filter by company uuid
    /// collecting all uuids when related with company
    pub(crate) fn get_uuids_by_company(
        target_company_uuid: &Uuid,
        filter_components_uuids: &[Uuid],
        conn: &PgConnection,
    ) -> ServiceResult<Vec<Uuid>> {
        use crate::schema::supplier_to_component::dsl as supplier_to_component;

        if filter_components_uuids.is_empty() {
            supplier_to_component::supplier_to_component
                .filter(supplier_to_component::company_uuid.eq(target_company_uuid))
                .select(supplier_to_component::component_uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        } else {
            supplier_to_component::supplier_to_component
                .filter(supplier_to_component::company_uuid.eq(target_company_uuid)
                .and(supplier_to_component::component_uuid.eq_any(filter_components_uuids)))
                .select(supplier_to_component::component_uuid)
                .load::<Uuid>(conn).map_err(|err| {
                    debug!("Fail load uuid list target user: {:?}", err);
                    ServiceError::InternalServerError
                })
        }
    }

    /// Gets components short data without checking access
    pub(crate) fn get_without_check_by_uuids(
        logged_user_uuid: &Uuid,
        filter_components_uuids: &[Uuid],
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {

        let result: Vec<ShowComponentShort> = ShowComponentShort::get_list_by_uuids(
            filter_components_uuids,
            logged_user_uuid,
            set_lang_id,
            conn
        ).expect("Error loading list components and collect short data");

        debug!("Components data: {:#?}", result);

        Ok(result)
    }
}

impl ShowComponentShort {
    pub(crate) fn get_list_by_uuids(
        target_components_uuids: &[Uuid],
        target_user_uuid: &Uuid,
        set_lang_id: &i32,
        conn: &PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // the result for store the result :)
        let mut result: Vec<ShowComponentShort> = Vec::new();

        // collecting data for each component
        for target_component_uuid in target_components_uuids.iter() {
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
            let files = ShowFileForDownload::for_component(&component, conn)
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
    pub(crate) fn collect_related_data(
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
        let files = ShowFileForDownload::for_component(&component, conn)
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
