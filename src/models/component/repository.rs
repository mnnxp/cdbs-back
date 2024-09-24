use crate::errors::{ServiceResult, ServiceError};
use crate::models::search::order::{objects_order, Paginate, Sort};
use crate::models::search::model::ExtraOptions;
use crate::models::component::{
    model::{Component, ShowComponentShort, ComponentAndRelatedData},
    actual_status::model::ActualStatusTranslateList,
    component_type::model::ComponentTypeTranslateList,
    param::model::ComponentParamWithTranslation,
    component_fav::model::ComponentFav,
    supplier::model::ComponentSupplierRelatedData,
    component_modification::model::{
        ComponentModification, ComponentModificationAndRelatedData, ComponentModificationArg
    },
    access::util::check_access_component_for_user,
    util::get_files_by_ext,
};
use crate::models::user::model::ShowUserShort;
use crate::models::standard::model::ShowStandardShort;
use crate::models::relate_ref::{
    type_access::model::TypeAccessTranslateList,
    license::model::License,
    file::model::{ShowFileRelatedData, DownloadFile, FileByExtArg},
    keyword::model::Keyword,
    spec::model::SpecTranslateList,
};
use crate::schema::component_ref::dsl as component_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Component {
    /// Get component data from component_ref table by uuid
    pub(crate) fn get_component_by_uuid(
        target_component_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Component> {
        component_ref::component_ref
            .filter(component_ref::uuid.eq(target_component_uuid)
            .and(component_ref::is_delete.eq(false)))
            .select((
                component_ref::uuid,
                component_ref::parent_component_uuid,
                component_ref::name,
                component_ref::description,
                component_ref::image_file_uuid,
                component_ref::user_uuid,
                component_ref::type_access_id,
                component_ref::component_type_id,
                component_ref::actual_status_id,
                component_ref::is_base,
                component_ref::created_at,
                component_ref::updated_at,
            ))
            .first::<Component>(conn)
            .map_err(|err| {
                debug!("Failed get component: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowComponentShort {
    /// Gets components by filter or all public
    pub(crate) fn get_components(
        filter_component_uuids: &[Uuid],
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        match filter_component_uuids.is_empty() {
            true => ShowComponentShort::get_all_public(options, sort, paginate, conn),
            false => ShowComponentShort::get_list_by_uuids(
                filter_component_uuids,
                options,
                sort,
                paginate,
                conn
            ),
        }
    }

    pub(crate) fn get_by_uuid(
        component_uuid: &Uuid,
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowComponentShort> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        // check access user for select component
        check_access_component_for_user(
            &options.logged_user_uuid,
            component_uuid,
            &need_access_level,
            conn
        )?;

        ShowComponentShort::get_without_check_by_uuid(
            component_uuid,
            options,
            paginate,
            conn
        )
    }

    /// Gets component short data without checking access
    pub(crate) fn get_without_check_by_uuid(
        target_component_uuid: &Uuid,
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowComponentShort> {
        // get target component
        let component = Component::get_component_by_uuid(
            target_component_uuid,
            conn
        ).expect("Failed get Component data");

        // get image file (favicon) for component
        let image_file = DownloadFile::get_by_file_uuid(
            &component.image_file_uuid,
            conn
        ).expect("Error get presigned url main image");

        // get component owner
        let owner_user = ShowUserShort::get_without_check_by_uuid(
            &component.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get component type with translation
        let type_access = TypeAccessTranslateList::get_type_access_by_id(
            &component.type_access_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading type_access");

        // get component type with translation for component
        let component_type = ComponentTypeTranslateList::get_by_id(
            &component.component_type_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading component_type");

        // get actual status with translation for component
        let actual_status = ActualStatusTranslateList::get_by_id(
            &component.actual_status_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading actual_status");

        // check whether the object is being tracked auth user
        let is_followed = crate::models::component::component_fav::util::check_subscriber_by_uuid(
            target_component_uuid,
            &options.logged_user_uuid,
            conn
        ).expect("Error get is_followed");

        // get licenses for component
        let licenses = License::get_by_component(
            &component,
            conn
        ).expect("Error loading license");

        // get files for component
        let files = {
            let image_uuids: Vec<Uuid> = get_files_by_ext(
                &component.uuid,
                &FileByExtArg::image(),
                conn
            )?;

            DownloadFile::get_by_file_uuids(
                &image_uuids,
                paginate,
                conn
            )
            .expect("Error loading component_file")
        };

        // collect data for supplier component
        let component_suppliers = ComponentSupplierRelatedData::get_first_supplier(
            &component.uuid,
            conn
        ).expect("Error loading supplier_component_with_relate");

        Ok(ShowComponentShort {
            uuid: component.uuid,
            name: component.name,
            description: component.description,
            image_file,
            owner_user,
            type_access,
            component_type,
            actual_status,
            is_base: component.is_base,
            is_followed,
            updated_at: component.updated_at,
            licenses,
            files,
            component_suppliers,
        })
    }

    /// Gets components short data by uuids
    pub(crate) fn get_list_by_uuids(
        component_uuids: &[Uuid],
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        let mut result: Vec<ShowComponentShort> = Vec::new();
        // collecting data for each component
        for ct_uuid in objects_order(component_uuids, sort, paginate, conn)? {
            match ShowComponentShort::get_by_uuid(
                &ct_uuid,
                options,
                paginate,
                conn
            ) {
                Ok(value) => result.push(value),
                Err(err) => {
                    debug!("Failed get component short data: {:?}", err);
                },
            };
        }
        Ok(result)
    }

    /// Gets all public components short data
    pub(crate) fn get_all_public(
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        let component_uuids = component_ref::component_ref
            .filter(component_ref::type_access_id.eq(3)
            .and(component_ref::is_delete.eq(false)))
            .select(component_ref::uuid)
            // .order(component_ref::created_at.desc())
            // .limit(1000)
            // .offset(options.offset as i64)
            .load::<Uuid>(conn)
            .expect("Failed get public components");

        // the result for store the result :)
        let mut result: Vec<ShowComponentShort> = Vec::new();
        // collecting data for each component without check
        for ct_uuid in objects_order(&component_uuids, sort, paginate, conn)? {
            result.push(ShowComponentShort::get_without_check_by_uuid(
                &ct_uuid,
                options,
                paginate,
                conn
            )?);
        }
        Ok(result)
    }
}

impl ComponentAndRelatedData {
    /// Collecting component data and related data using uuid
    pub(crate) fn get_component(
        target_component_uuid: &Uuid,
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<ComponentAndRelatedData> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        check_access_component_for_user(
            &options.logged_user_uuid,
            target_component_uuid,
            &need_access_level,
            conn
        )?;

        // collect data for component
        let component = Component::get_component_by_uuid(
            target_component_uuid,
            conn
        ).expect("Error loading component");

        // get image file (favicon) for component
        let image_file = DownloadFile::get_by_file_uuid(
            &component.image_file_uuid,
            conn
        ).expect("Error get presigned url main image");

        // get component owner
        let owner_user = ShowUserShort::get_without_check_by_uuid(
            &component.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get component type with translation
        let type_access = TypeAccessTranslateList::get_type_access_by_id(
            &component.type_access_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading type_access");

        // get component type with translation for component
        let component_type = ComponentTypeTranslateList::get_by_id(
            &component.component_type_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading component_type");

        // get actual status with translation for component
        let actual_status = ActualStatusTranslateList::get_by_id(
            &component.actual_status_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading actual status");

        // count subscribers component
        let subscribers: i32 = ComponentFav::get_count_followers_by_uuid(&component.uuid, conn)?;

        // check whether the object is being tracked auth user
        let is_followed = crate::models::component::component_fav::util::check_subscriber_by_uuid(
            target_component_uuid,
            &options.logged_user_uuid,
            conn
        ).expect("Error get is_followed");

        // get params with translation for component
        let component_params = ComponentParamWithTranslation::by_component_uuid(
            &component.uuid,
            &options.set_lang_id,
            conn
        ).expect("Error loading params component with translate");

        // get licenses for component
        let licenses = License::get_by_component(
            &component,
            conn
        ).expect("Error loading license");

        // get files for component
        let files = ShowFileRelatedData::by_component_uuid(
            &component.uuid,
            paginate,
            conn
        ).expect("Error loading component files");

        // get specs with translation for component
        let component_specs = SpecTranslateList::for_component(
            &component,
            &options.set_lang_id,
            conn
        ).expect("Error loading spec component with translate");

        // get keywords for component
        let component_keywords = Keyword::get_by_component(
            &component,
            conn
        ).expect("Error loading component keywords");

        // collect data for modifications the component
        let component_modifications = ComponentModification::by_args(
            &ComponentModificationArg::component_uuid(&component.uuid),
            conn
        ).expect("Error loading component modifications");

        // get list component modifications with related data and translation
        let component_modifications = ComponentModificationAndRelatedData::for_modifications(
            &component_modifications,
            &options.set_lang_id,
            conn
        ).expect("Error loading component modifications with related data");

        // collect data for supplier component
        let component_suppliers = ComponentSupplierRelatedData::by_component_uuid(
            &component.uuid,
            conn
        ).expect("Error loading supplier component with relate");

        // collect data for component standards
        let component_standards = ShowStandardShort::for_component(
            target_component_uuid,
            options,
            conn
        ).expect("Error loading supplier component with relate");

        Ok(ComponentAndRelatedData {
            uuid: component.uuid,
            parent_component_uuid: component.parent_component_uuid,
            name: component.name,
            description: component.description,
            image_file,
            owner_user,
            type_access,
            component_type,
            actual_status,
            is_base: component.is_base,
            subscribers,
            is_followed,
            created_at: component.created_at,
            updated_at: component.updated_at,
            component_params,
            licenses,
            files,
            component_specs,
            component_keywords,
            component_modifications,
            component_suppliers,
            component_standards,
        })
    }
}
