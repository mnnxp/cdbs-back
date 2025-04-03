use crate::errors::{ServiceResult, ServiceError};
use crate::graphql::service_model::{ShowServiceShort, ServiceAndRelatedData};
use crate::models::supplier_service::{
    model::Service,
    service_status::model::ServiceStatusTranslateList,
    access::util::check_access_service_for_user,
};
use crate::models::user::model::ShowUserShort;
use crate::models::company::model::ShowCompanyShort;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::{Paginate, Sort, objects_order};
use crate::schema::service_ref::dsl as service_ref;
use diesel::prelude::*;
use uuid::Uuid;

impl Service {
    /// Get service data from service_ref table by uuid
    pub(crate) fn get_service_by_uuid(
        target_service_uuid: &Uuid,
        conn: &mut PgConnection,
    ) -> ServiceResult<Service> {
        service_ref::service_ref
            .filter(service_ref::uuid.eq(target_service_uuid)
            .and(service_ref::is_delete.eq(false)))
            .select((
                service_ref::uuid,
                service_ref::name,
                service_ref::description,
                service_ref::user_uuid,
                service_ref::company_uuid,
                service_ref::service_status_id,
                service_ref::region_id,
                service_ref::created_at,
                service_ref::updated_at,
            ))
            .first::<Service>(conn)
            .map_err(|err| {
                debug!("Failed get service: {:?}", err);
                ServiceError::InternalServerError
            })
    }
}

impl ShowServiceShort {
    /// Gets services by filter or all public
    /// limit and offset works only without filter
    pub(crate) fn get_services(
        filter_services_uuids: &[Uuid],
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowServiceShort>> {
        match filter_services_uuids.is_empty() {
            true => ShowServiceShort::get_all_public(options, paginate, conn),
            false => ShowServiceShort::get_list_by_uuids(
                filter_services_uuids,
                options,
                sort,
                paginate,
                conn
            )
        }
    }

    /// Gets service short data by service_uuid with check access
    pub(crate) fn get_by_uuid(
        target_service_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowServiceShort> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        check_access_service_for_user(
            &options.logged_user_uuid,
            target_service_uuid,
            &need_access_level,
            conn
        )?;

        // get target service
        let service: Service = Service::get_service_by_uuid(
            target_service_uuid,
            conn
        ).expect("Error loading service");

        // get data a owner user for a service
        let owner_user = ShowUserShort::get_without_check_by_uuid(
            &service.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get service owner company
        let owner_company = ShowCompanyShort::get_without_check_by_uuid(
            &service.company_uuid,
            &options.logged_user_uuid,
            &options.set_lang_id,
            conn
        ).expect("Error loading company short data");

        // get service type with translation for service
        let service_status = ServiceStatusTranslateList::get_by_id(
            &service.service_status_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading service_status");

        Ok(ShowServiceShort {
            uuid: service.uuid,
            name: service.name,
            description: service.description,
            owner_user,
            owner_company,
            service_status,
            updated_at: service.updated_at,
        })

    }

    pub(crate) fn get_list_by_uuids(
        service_uuids: &[Uuid],
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowServiceShort>> {
        let mut result: Vec<ShowServiceShort> = Vec::new();
        // collecting data for each service
        for ct_uuid in objects_order(service_uuids, sort, paginate, conn)? {
            match ShowServiceShort::get_by_uuid(&ct_uuid, options, conn) {
                Ok(value) => result.push(value),
                Err(err) => {
                    debug!("Failed get service short data: {:?}", err);
                },
            };
        }
        Ok(result)
    }

    /// Gets all public services short data
    pub(crate) fn get_all_public(
        options: &ExtraOptions,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowServiceShort>> {
        // gets all public services uuids
        let service_uuids = service_ref::service_ref
            .filter(service_ref::type_access_id.eq(3)
                .and(service_ref::is_delete.eq(false)))
            .select(service_ref::uuid)
            .limit(paginate.limit)
            .offset(paginate.offset)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get public services: {:?}", err);
                ServiceError::InternalServerError
            })?;

        // the for collect the result :)
        let mut result: Vec<ShowServiceShort> = Vec::new();
        // collecting data for each service
        for target_service_uuid in service_uuids.iter() {
            let _res = ShowServiceShort::get_by_uuid(target_service_uuid, options, conn)
                .map(|value| result.push(value))
                .map_err(|err| debug!("Failed get service data: {:?}", err));
        }
        Ok(result)
    }
}

impl ServiceAndRelatedData {
    /// Gathers data for a service and related data by uuid
    pub(crate) fn collect_related_data(
        target_service_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ServiceAndRelatedData> {
        let need_access_level = 3; // todo!(create enum for manage access level)

        check_access_service_for_user(
            &options.logged_user_uuid,
            target_service_uuid,
            &need_access_level,
            conn
        )?;

        // collect data for service
        let service: Service = Service::get_service_by_uuid(
            target_service_uuid,
            conn
        ).expect("Error loading service");

        // get data a owner user for a service
        let owner_user = ShowUserShort::get_without_check_by_uuid(
            &service.user_uuid,
            conn
        ).expect("Error loading slim_user");

        // get data a owner company for a service
        let owner_company = ShowCompanyShort::get_without_check_by_uuid(
            &service.company_uuid,
            &options.logged_user_uuid,
            &options.set_lang_id,
            conn
        ).expect("Error loading company short data");

        // get service type with translation for service
        let service_status = ServiceStatusTranslateList::get_by_id(
            &service.service_status_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading service_status");

        // get region for company
        let region = RegionTranslateList::get_region_by_id(
            &service.region_id,
            &options.set_lang_id,
            conn
        ).expect("Error loading company_type");

        Ok(ServiceAndRelatedData {
            uuid: service.uuid,
            name: service.name,
            description: service.description,
            owner_user,
            owner_company,
            service_status,
            region,
            created_at: service.created_at,
            updated_at: service.updated_at,
        })
    }
}
