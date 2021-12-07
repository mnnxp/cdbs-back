use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::component::{
    model::{ComponentAndRelatedData, ShowComponentShort, ComponentsArg, IptComponentsArg},
    relate::spec::model::{IptComponentSpecsArg, ComponentSpecsArg},
    component_modification,
    component_modification::fileset_for_program::model::FilesetProgramRelatedData,
    component_modification::modification_file_from_fileset::model::FileOfFileset,
    access::company::model::CompanyAccessComponentAndRelatedData,
    access::user::model::UserAccessComponentAndRelatedData,
};
use crate::models::relate_ref::{
    file::model::DownloadFile,
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
        arguments: Option<IptComponentsArg>
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        use crate::models::component::service::list::get_components;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: ComponentsArg = match arguments {
            Some(args) => ComponentsArg::from(args),
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
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::component::file::service::list::get_component_files;

        // authorization check
        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_files(
            &logged_user_uuid,
            &component_uuid,
            conn
        )
    }

    async fn component_specs(
        &self,
        cxt: &Context<'_>,
        arg: IptComponentSpecsArg,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::component::spec::service::list::get_component_specs;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arg: ComponentSpecsArg = arg.into();

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_component_specs(
            &logged_user_uuid,
            &arg,
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
        modification_uuid: Uuid,
        program_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FilesetProgramRelatedData>> {
        use component_modification::fileset_for_program::service::list::get_modification_filesets;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_modification_filesets(
            &logged_user_uuid,
            &modification_uuid,
            &program_id,
            &limit,
            &offset,
            conn
        )
    }

    async fn component_modification_files_of_fileset(
        &self,
        cxt: &Context<'_>,
        fileset_uuid: Uuid,
        file_uuids: Option<Vec<Uuid>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FileOfFileset>> {
        use component_modification::modification_file_from_fileset::service::list::get_files_of_fileset;

        let logged_user_uuid: Uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_of_fileset(
            &logged_user_uuid,
            &fileset_uuid,
            &file_uuids,
            &limit,
            &offset,
            conn
        )
    }
}
