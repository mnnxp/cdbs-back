use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::graphql::{
    component_model::{
        ComponentAndRelatedData, ShowComponentShort, IptComponentsArg, IptComponentFilesArg,
        ComponentModificationAndRelatedData
    },
    relate::attributes::IptPaginate,
};
use crate::models::search::model::{ExtraOptions, IptSearchArg};
use crate::models::user::access::logged::{get_logged_user_uuid, check_authorized};
use crate::models::component::{
    model::{ComponentsArg, ComponentFilesArg},
    relate::{
        supplier::model::ComponentSupplierRelatedData,
        keyword::model::{IptComponentKeywordsArg, ComponentKeywordsArg},
        spec::model::{IptComponentSpecsArg, ComponentSpecsArg},
        component_type::model::ComponentTypeTranslateList,
        actual_status::model::ActualStatusTranslateList,
    },
    component_modification,
    component_modification::{
        model::{
            IptComponentModificationArg, ComponentModificationArg,
            IptModificationFilesArg, ModificationFilesArg
        },
        fileset_for_program::model::{FilesetProgramRelatedData, IptFilesetProgramArg, FilesetProgramArg},
        fileset_for_program::file::model::{IptFileOfFilesetArg, FileOfFilesetArg},
    },
    access::company::model::CompanyAccessComponentAndRelatedData,
    access::user::model::UserAccessComponentAndRelatedData,
};
use crate::models::relate_ref::{
    file::model::{DownloadFile, ShowFileRelatedData},
    keyword::model::Keyword,
    spec::model::SpecTranslateList,
    language::get_set_language,
};
use crate::models::search::order::Paginate;
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
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        use crate::models::component::service::list::get_components_by_uuids;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        // authorization check
        let options = ExtraOptions::from_cxt(cxt)?;
        get_components_by_uuids(&args, &options, conn)
    }

    /// Returns brief information about components with filter by:
    /// UUIDs, company, standard, user, favorite (for self or other user).
    async fn components(
        &self,
        cxt: &Context<'_>,
        args: Option<IptComponentsArg>
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        use crate::models::component::service::list::get_components;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: ComponentsArg = match args {
            Some(x) => ComponentsArg::from(x),
            None => ComponentsArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_components(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn,
        )
    }

    /// Returns complete information about the component by UUID.
    async fn component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentAndRelatedData> {
        use crate::models::component::service::list::get_component_by_uuid;

        // authorization check
        let options = ExtraOptions::from_cxt(cxt)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_by_uuid(
            &component_uuid,
            &options,
            conn,
        )
    }

    /// Returns a list of component modifications by component UUID.
    async fn component_modifications(
        &self,
        cxt: &Context<'_>,
        args: IptComponentModificationArg,
    ) -> ServiceResult<Vec<ComponentModificationAndRelatedData>> {
        use crate::models::component::component_modification::service::list::get_component_modifications;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let args: ComponentModificationArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_modifications(
            &logged_user_uuid,
            &args,
            &get_set_language(cxt),
            conn
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
        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        let p = paginate
                .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
                .unwrap_or_default();
        get_component_suppliers(
            &logged_user_uuid,
            &component_uuid,
            &p,
            conn
        )
    }

    /// Returns array of keywords associated with the component.
    async fn component_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptComponentKeywordsArg,
    ) -> ServiceResult<Vec<Keyword>> {
        use crate::models::component::keyword::service::list::get_component_keywords;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: ComponentKeywordsArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_keywords(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    /// Returns a list of companies that have access to a component.
    async fn get_companies_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
        use crate::models::component::access::company::manage::get_companies_list_access_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_companies_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns a list of users who have access to a component.
    async fn get_users_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
        use crate::models::component::access::user::manage::get_users_list_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_users_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns pre-signed URLs and other information for downloading component files.
    async fn component_files(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesArg,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::component::file::service::list::get_component_files;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: ComponentFilesArg = args.into();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_files(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    /// Returns information about files of a component.
    async fn component_files_list(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesArg,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use crate::models::component::file::service::list::get_component_files_list;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: ComponentFilesArg = args.into();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_files_list(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    /// Returns an array of directory partitions associated with a component.
    async fn component_specs(
        &self,
        cxt: &Context<'_>,
        args: IptComponentSpecsArg,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::component::spec::service::list::get_component_specs;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: ComponentSpecsArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_specs(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns pre-signed URLs and other information for downloading component modification files.
    async fn component_modification_files(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFilesArg,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::file::service::list::get_component_modification_files;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let args: ModificationFilesArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_modification_files(&logged_user_uuid, &args, conn)
    }

    /// Returns information about files of a component modification.
    async fn component_modification_files_list(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFilesArg,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use component_modification::file::service::list::get_component_modification_files_list;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let args: ModificationFilesArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_component_modification_files_list(&logged_user_uuid, &args, conn)
    }

    /// Returns a list of filesets by component modification UUID.
    /// Filtering by program IDs is available.
    async fn component_modification_filesets(
        &self,
        cxt: &Context<'_>,
        args: IptFilesetProgramArg,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        use component_modification::fileset_for_program::service::list::get_modification_filesets;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments = FilesetProgramArg::from(args);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_modification_filesets(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    /// Returns information about files from a component modification fileset.
    async fn component_modification_files_of_fileset(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use component_modification::fileset_for_program::file::service::list::get_files_of_fileset;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_files_of_fileset(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    /// Returns pre-signed URLs and other information for downloading files of component modification fileset.
    async fn component_modification_fileset_files(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::fileset_for_program::file::service::list::get_fileset_files;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_fileset_files(
            &logged_user_uuid,
            &arguments,
            conn
        )
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

        get_component_types(
            &filter,
            &get_set_language(cxt),
            conn
        )
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

        get_actual_statuses(
            &filter,
            &get_set_language(cxt),
            conn
        )
    }
}
