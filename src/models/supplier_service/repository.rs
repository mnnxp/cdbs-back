use crate::auth::{require_permission, AccessEntity, AccessOperation};
use crate::errors::{ServiceError, ServiceResult};
use crate::graphql::service_model::{ServiceAndRelatedData, ShowServiceShort};
use crate::models::company::model::ShowCompanyShort;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::{objects_order, Paginate, Sort};
use crate::models::supplier_service::access::repository::get_accessible_service_uuids;
use crate::models::supplier_service::model::Service;
use crate::models::supplier_service::service_status::model::ServiceStatusTranslateList;
use crate::models::user::model::ShowUserShort;
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
            .filter(
                service_ref::uuid
                    .eq(target_service_uuid)
                    .and(service_ref::is_delete.eq(false)),
            )
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
            true => ShowServiceShort::get_all_public(options, sort, paginate, conn),
            false => ShowServiceShort::get_list_by_uuids(
                filter_services_uuids,
                options,
                sort,
                paginate,
                conn,
            ),
        }
    }

    /// Gets service short data without checking access
    pub(crate) fn get_without_check_by_uuid(
        target_service_uuid: &Uuid,
        options: &ExtraOptions,
        conn: &mut PgConnection,
    ) -> ServiceResult<ShowServiceShort> {
        // get target service
        let service: Service =
            Service::get_service_by_uuid(target_service_uuid, conn).expect("Error loading service");

        // get data a owner user for a service
        let owner_user =
            ShowUserShort::get_without_check_by_uuid(&service.user_uuid, &options.domain, conn)
                .expect("Error loading slim_user");

        // get service owner company
        let owner_company =
            ShowCompanyShort::get_without_check_by_uuid(&service.company_uuid, conn)
                .expect("Error loading company short data");

        // get service status with translation for service
        let service_status = ServiceStatusTranslateList::get_by_id(
            service.service_status_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error loading service_status");

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
        // filter by access (one SQL)
        let accessible = get_accessible_service_uuids(
            &options.logged_user_uuid,
            AccessOperation::Read,
            service_uuids,
            conn,
        )?;
        if accessible.is_empty() {
            return Ok(Vec::new());
        }
        // sort and paginate the accessible set
        let ordered_uuids = objects_order(&accessible, sort, paginate, conn)?;
        // hydrate (no access check, already filtered)
        let mut result = Vec::with_capacity(ordered_uuids.len());
        for uuid in &ordered_uuids {
            result.push(ShowServiceShort::get_without_check_by_uuid(
                uuid, options, conn,
            )?);
        }
        Ok(result)
    }

    /// Gets all public services short data
    pub(crate) fn get_all_public(
        options: &ExtraOptions,
        sort: &Sort,
        paginate: &Paginate,
        conn: &mut PgConnection,
    ) -> ServiceResult<Vec<ShowServiceShort>> {
        // gets all public services uuids
        let service_uuids = service_ref::service_ref
            .filter(
                service_ref::type_access_id
                    .eq(3)
                    .and(service_ref::is_delete.eq(false)),
            )
            .select(service_ref::uuid)
            .load::<Uuid>(conn)
            .map_err(|err| {
                debug!("Failed get public services: {:?}", err);
                ServiceError::InternalServerError
            })?;

        let mut result: Vec<ShowServiceShort> = Vec::new();
        for target_service_uuid in objects_order(&service_uuids, sort, paginate, conn)? {
            result.push(ShowServiceShort::get_without_check_by_uuid(
                &target_service_uuid,
                options,
                conn,
            )?);
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
        require_permission(
            &options.logged_user_uuid,
            AccessEntity::Service,
            target_service_uuid,
            AccessOperation::Read,
            conn,
        )?;

        // collect data for service
        let service: Service =
            Service::get_service_by_uuid(target_service_uuid, conn).expect("Error loading service");

        // get data a owner user for a service
        let owner_user =
            ShowUserShort::get_without_check_by_uuid(&service.user_uuid, &options.domain, conn)
                .expect("Error loading slim_user");

        // get data a owner company for a service
        let owner_company =
            ShowCompanyShort::get_without_check_by_uuid(&service.company_uuid, conn)
                .expect("Error loading company short data");

        // get service type with translation for service
        let service_status = ServiceStatusTranslateList::get_by_id(
            service.service_status_id,
            options.set_lang_id,
            conn,
        )
        .expect("Error loading service_status");

        // get region for company
        let region =
            RegionTranslateList::get_region_by_id(service.region_id, options.set_lang_id, conn)
                .expect("Error loading company_type");

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
