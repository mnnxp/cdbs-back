use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::{
    component_model::{
        ComponentAndRelatedData, ComponentModificationAndRelatedData, IptComponentFilesArg,
        IptComponentsArg, ShowComponentShort,
    },
    file::ShowFileRelatedData,
    relate::attributes::{IptPaginate, IptSort},
};
use crate::models::component::{
    access::company::model::CompanyAccessComponentAndRelatedData,
    access::user::model::UserAccessComponentAndRelatedData,
    component_modification,
    component_modification::{
        fileset_for_program::file::model::{FileOfFilesetArg, IptFileOfFilesetArg},
        fileset_for_program::model::{
            FilesetProgramArg, FilesetProgramRelatedData, IptFilesetProgramArg,
        },
        model::{ComponentModificationArg, IptModificationFilesArg, ModificationFilesArg},
    },
    model::{ComponentFilesArg, ComponentsArg},
    relate::{
        actual_status::model::ActualStatusTranslateList,
        component_type::model::ComponentTypeTranslateList,
        supplier::model::ComponentSupplierRelatedData,
    },
};
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, language::get_set_language,
    spec::model::SpecTranslateList,
};
use crate::models::search::model::{ExtraOptions, IptSearchArg};
use crate::models::search::order::{Paginate, Sort, TableName};
use crate::models::user::access::logged::check_authorized;
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct ComponentQuery;

#[Object]
impl ComponentQuery {
    /// Returns a list of ShowComponentShort that matches the given search parameters.
    /// In case of incompatibility of argument values, a matching error will be returned.
    async fn search_by_components(
        &self,
        cxt: &Context<'_>,
        args: IptSearchArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        use crate::models::component::service::list::get_components_by_uuids;
        debug!("Query search components: {:?}", args);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        // authorization check, search maybe without login (no_entry)
        let options = ExtraOptions::from_cxt(cxt, true)?;
        let s = sort
            .map(|s| Sort::parsing(TableName::ComponentRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::ComponentRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        get_components_by_uuids(&args, &options, &s, &p, conn)
    }

    /// Returns brief information about components with filter by:
    /// UUIDs, company, standard, user, favorite (for self or other user).
    async fn components(
        &self,
        cxt: &Context<'_>,
        args: Option<IptComponentsArg>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        use crate::models::component::service::list::get_components;

        let arguments = ComponentsArg::by_arg(args);
        // authorization check, if token verification fails, try to get the default user UUID
        let options = ExtraOptions::from_cxt(cxt, !arguments.favorite)?;
        let s = sort
            .map(|s| Sort::parsing(TableName::ComponentRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::ComponentRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_components(&arguments, &options, &s, &p, conn)
    }

    /// Returns complete information about the component by UUID.
    async fn component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentAndRelatedData> {
        use crate::models::component::service::list::get_component_by_uuid;
        debug!("Query component: {}", component_uuid);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        // authorization check, if token verification fails, try to get the default user UUID
        let options = ExtraOptions::from_cxt(cxt, true)?;
        if options.no_entry {
            debug!(
                "Get component without login (no_entry): {:?}",
                component_uuid
            );
        }
        get_component_by_uuid(&component_uuid, &options, conn)
    }

    /// Returns a list of component modifications by component UUID.
    async fn component_modifications(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        filter: Option<Vec<Uuid>>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
        use crate::models::component::component_modification::service::list::get_component_modifications;
        // authorization check, if token verification fails, try to get the default user UUID
        let options = ExtraOptions::from_cxt(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_component_modifications(
            &ComponentModificationArg::parsing(component_uuid, filter, sort, paginate),
            &options,
            conn,
        )
    }

    /// Returns a list of component suppliers.
    async fn component_suppliers(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ComponentSupplierRelatedData>> {
        use crate::models::component::supplier::service::list::get_component_suppliers;
        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        get_component_suppliers(&logged_user_uuid, &component_uuid, &p, conn)
    }

    /// Returns array of keywords associated with the component.
    async fn component_keywords(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Keyword>> {
        use crate::models::component::keyword::service::list::get_component_keywords;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_keywords(&logged_user_uuid, &component_uuid, &p, conn)
    }

    /// Returns a list of companies that have access to a component.
    async fn get_companies_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
        use crate::models::component::access::company::manage::get_companies_list_access_component;

        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_companies_list_access_component(&component_uuid, &options, conn)
    }

    /// Returns a list of users who have access to a component.
    async fn get_users_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
        use crate::models::component::access::user::manage::get_users_list_access_component;

        // checking authorization and getting user uuid
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_users_list_access_component(&component_uuid, &options, conn)
    }

    /// Returns pre-signed URLs and other information for downloading component files.
    async fn component_files(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::component::file::service::list::get_component_files;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let arguments: ComponentFilesArg = args.into();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_files(&logged_user_uuid, &arguments, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns information about files of a component.
    async fn component_files_list(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use crate::models::component::file::service::list::get_component_files_list;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let arguments: ComponentFilesArg = args.into();
        let s = sort
            .map(|s| Sort::parsing(TableName::FileRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::FileRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_component_files_list(&logged_user_uuid, &arguments, &s, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns an array of catalogs associated with a component
    async fn component_specs(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::component::spec::service::list::get_component_specs;

        // authorization check, if token verification fails, try to get the default user UUID
        let options = ExtraOptions::from_cxt(cxt, true)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_specs(&component_uuid, &options, &p, conn)
    }

    /// Returns pre-signed URLs and other information for downloading component modification files.
    async fn component_modification_files(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFilesArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::file::service::list::get_component_modification_files;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let args: ModificationFilesArg = args.into();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_component_modification_files(&logged_user_uuid, &args, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns information about files of a component modification.
    async fn component_modification_files_list(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFilesArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use component_modification::file::service::list::get_component_modification_files_list;
        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let args: ModificationFilesArg = args.into();
        let s = sort
            .map(|s| Sort::parsing(TableName::FileRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::FileRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_component_modification_files_list(&logged_user_uuid, &args, &s, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns a list of filesets by component modification UUID.
    /// Filtering by program IDs is available.
    async fn component_modification_filesets(
        &self,
        cxt: &Context<'_>,
        args: IptFilesetProgramArg,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        use component_modification::fileset_for_program::service::list::get_modification_filesets;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let arguments = FilesetProgramArg::from(args);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_modification_filesets(&logged_user_uuid, &arguments, conn)
    }

    /// Returns information about files from a component modification fileset.
    async fn component_modification_files_of_fileset(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use component_modification::fileset_for_program::file::service::list::get_files_of_fileset;
        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let s = sort
            .map(|s| Sort::parsing(TableName::FileRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::FileRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_files_of_fileset(&logged_user_uuid, &arguments, &s, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns pre-signed URLs and other information for downloading files of component modification fileset.
    async fn component_modification_fileset_files(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::fileset_for_program::file::service::list::get_fileset_files;

        // authorization check, if token verification fails, try to get the default user UUID
        let logged_user_uuid = ExtraOptions::from_cxt(cxt, true).map(|eo| eo.logged_user_uuid)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_fileset_files(&logged_user_uuid, &arguments, &p, &extract_client_domain(cxt), conn)
    }

    /// Returns a list of component types.
    /// Filtering by component type IDs is available.
    async fn component_types(
        &self,
        cxt: &Context<'_>,
        filter: Option<Vec<i32>>,
    ) -> ServiceResult<Vec<ComponentTypeTranslateList>> {
        use crate::models::component::relate::component_type::service::list::get_component_types;

        check_authorized(cxt)?;

        let filter: Vec<i32> = filter.unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_types(&filter, &get_set_language(cxt), conn)
    }

    /// Returns a list of available states (statuses) for components.
    /// Filtering by component actual status IDs is available.
    async fn component_actual_statuses(
        &self,
        cxt: &Context<'_>,
        filter: Option<Vec<i32>>,
    ) -> ServiceResult<Vec<ActualStatusTranslateList>> {
        use crate::models::component::relate::actual_status::service::list::get_actual_statuses;

        check_authorized(cxt)?;

        let filter: Vec<i32> = filter.unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_actual_statuses(&filter, &get_set_language(cxt), conn)
    }
}
