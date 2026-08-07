use crate::auth::token::logged::check_authorized;
use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::relate::attributes::{IptPaginate, IptSort};
use crate::graphql::service_model::{
    IptServiceFilesArg, IptServicesArg, ServiceAndRelatedData, ShowServiceShort,
};
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, language::get_set_language,
    spec::model::SpecTranslateList,
};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::{Paginate, Sort, TableName};
use crate::models::supplier_service::{
    access::company::model::CompanyAccessServiceAndRelatedData,
    access::user::model::UserAccessServiceAndRelatedData,
    keyword::service::list::get_service_keywords,
    model::{ServiceFilesArg, ServicesArg},
    relate::service_status::model::ServiceStatusTranslateList,
};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct ServiceQuery;

#[Object]
impl ServiceQuery {
    /// Returns brief information about services with filter by:
    /// UUIDs, company, user, favorite (for self or other user).
    async fn services(
        &self,
        ctx: &Context<'_>,
        args: Option<IptServicesArg>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowServiceShort>> {
        use crate::models::supplier_service::service::list::get_services;
        // authorization check
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let arguments = ServicesArg::by_arg(args);
        let s = sort
            .map(|s| Sort::parsing(TableName::ServiceRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::ServiceRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_services(&arguments, &options, &s, &p, conn)
    }

    /// Returns complete information about the service by UUID.
    async fn service(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
    ) -> ServiceResult<ServiceAndRelatedData> {
        use crate::models::supplier_service::service::list::find_by_uuid;
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        find_by_uuid(&service_uuid, &options, conn)
    }

    /// Returns pre-signed URLs and other information for downloading service files.
    async fn service_files(
        &self,
        ctx: &Context<'_>,
        args: IptServiceFilesArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::supplier_service::file::service::list::get_service_files;
        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let arguments: ServiceFilesArg = args.into();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_service_files(
            &logged_user_uuid,
            &arguments,
            &p,
            &extract_client_domain(ctx),
            conn,
        )
    }

    /// Returns an array of catalogs associated with service
    async fn service_specs(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::supplier_service::spec::service::list::get_service_specs;
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        get_service_specs(&service_uuid, &options, &p, conn)
    }

    /// Returns array of keywords associated with the service.
    async fn service_keywords(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_service_keywords(&service_uuid, &logged_user_uuid, &p, conn)
    }

    /// Returns a list of companies that have access to a service.
    async fn get_companies_list_access_service(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessServiceAndRelatedData>> {
        use crate::models::supplier_service::access::company::manage::get_companies_list_access_service;
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_companies_list_access_service(&service_uuid, &options, conn)
    }

    /// Returns a list of users who have access to a service.
    async fn get_users_list_access_service(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessServiceAndRelatedData>> {
        use crate::models::supplier_service::access::user::manage::get_users_list_access_service;
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_users_list_access_service(&service_uuid, &options, conn)
    }

    /// Returns a list of available states (statuses) for services.
    /// Filtering by service actual status IDs is available.
    async fn service_statuses(
        &self,
        ctx: &Context<'_>,
        filter: Option<Vec<i32>>,
    ) -> ServiceResult<Vec<ServiceStatusTranslateList>> {
        use crate::models::supplier_service::relate::service_status::service::list::get_service_statuses;
        check_authorized(ctx)?; // checking authorization
        let filter: Vec<i32> = filter.unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_service_statuses(&filter, get_set_language(ctx), conn)
    }
}
