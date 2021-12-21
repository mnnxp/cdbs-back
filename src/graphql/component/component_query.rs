use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::access::logged::{get_logged_user_uuid, check_authorized};
use crate::models::component::{
    model::{
        ComponentAndRelatedData, ShowComponentShort,
        ComponentsArg, IptComponentsArg, IptComponentFilesArg, ComponentFilesArg
    },
    relate::spec::model::{IptComponentSpecsArg, ComponentSpecsArg},
    relate::actual_status::model::ActualStatusTranslateList,
    component_modification,
    component_modification::{
        fileset_for_program::model::{FilesetProgramRelatedData, IptFilesetProgramArg, FilesetProgramArg},
        modification_file_from_fileset::model::{IptFileOfFilesetArg, FileOfFilesetArg},
    },
    access::company::model::CompanyAccessComponentAndRelatedData,
    access::user::model::UserAccessComponentAndRelatedData,
};
use crate::models::relate_ref::{
    file::model::{DownloadFile, ShowFileRelatedData},
    spec::model::SpecTranslateList,
    language::get_set_language,
};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct ComponentQuery;

#[Object]
impl ComponentQuery {
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

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_components(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentAndRelatedData> {
        use crate::models::component::service::list::get_component_by_uuid;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_by_uuid(
            &logged_user_uuid,
            &component_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn get_companies_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
        use crate::models::component::access::company::manage::get_companies_list_access_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_companies_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    async fn get_users_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
        use crate::models::component::access::user::manage::get_users_list_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_users_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    async fn component_files(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesArg,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::component::file::service::list::get_component_files;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: ComponentFilesArg = args.into();
        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_files(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    async fn component_specs(
        &self,
        cxt: &Context<'_>,
        args: IptComponentSpecsArg,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::component::spec::service::list::get_component_specs;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: ComponentSpecsArg = args.into();

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_specs(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn
        )
    }

    async fn component_modification_files(
        &self,
        cxt: &Context<'_>,
        modification_uuid: Uuid,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::file::service::list::get_component_modification_files;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_modification_files(
            &logged_user_uuid,
            &modification_uuid,
            conn
        )
    }

    async fn component_modification_filesets(
        &self,
        cxt: &Context<'_>,
        args: IptFilesetProgramArg,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        use component_modification::fileset_for_program::service::list::get_modification_filesets;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments = FilesetProgramArg::from(args);
        let conn: &PooledConnection = &get_conn(cxt)?;

        get_modification_filesets(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    async fn component_modification_files_of_fileset(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        use component_modification::modification_file_from_fileset::service::list::get_files_of_fileset;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let conn: &PooledConnection = &get_conn(cxt)?;

        get_files_of_fileset(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    async fn component_modification_fileset_files(
        &self,
        cxt: &Context<'_>,
        args: IptFileOfFilesetArg,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use component_modification::modification_file_from_fileset::service::list::get_fileset_files;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: FileOfFilesetArg = FileOfFilesetArg::from(args);
        let conn: &PooledConnection = &get_conn(cxt)?;

        get_fileset_files(
            &logged_user_uuid,
            &arguments,
            conn
        )
    }

    async fn component_actual_statuses(
        &self,
        cxt: &Context<'_>,
        filter: Option<Vec<i32>>,
    ) -> ServiceResult<Vec<ActualStatusTranslateList>> {
        use crate::models::component::relate::actual_status::service::list::get_actual_statuses;

        check_authorized(cxt)?;
        
        let filter: Vec<i32> = filter.unwrap_or_default();
        let conn: &PooledConnection = &get_conn(cxt)?;

        get_actual_statuses(
            &filter,
            &get_set_language(cxt),
            conn
        )
    }
}
