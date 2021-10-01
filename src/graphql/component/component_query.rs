use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user;
use crate::models::component;
use crate::models::component::access::company::model::CompanyAccessComponentAndRelatedData;
use crate::models::component::access::user::model::UserAccessComponentAndRelatedData;
use crate::models::component::component_modification;
use crate::models::component::component_modification::fileset_for_program::model::FilesetProgramRelatedData;
use crate::models::component::component_modification::modification_file_from_fileset::model::ShowFileOfFileset;
use crate::models::component::model::{ComponentAndRelatedData, ShowComponentShort};
use crate::models::relate_ref::file::model::DownloadFile;

#[derive(Default)]
pub struct ComponentQuery;

#[Object]
impl ComponentQuery {
    async fn components(
        &self,
        cxt: &Context<'_>,
        components_uuids: Vec<Uuid>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // authorization check
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        component::service::list::find_components(
            &components_uuids,
            &logged_user_uuid,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }

    async fn component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        component::service::list::find_component_uuid(
            &component_uuid,
            &logged_user_uuid,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }

    async fn get_companies_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessComponentAndRelatedData>> {
        use component::access::company::manage::get_companies_list_access_component;

        // checking authorization and getting company uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_companies_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &crate::models::user::get_set_language(cxt),
            conn
        )
    }

    async fn get_users_list_access_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessComponentAndRelatedData>> {
        use component::access::user::manage::get_users_list_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_users_list_access_component(
            &logged_user_uuid,
            &component_uuid,
            &crate::models::user::get_set_language(cxt),
            conn
        )
    }

    async fn component_files(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Vec<DownloadFile>> {

        // authorization check
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        component::file::service::list::get_component_files(
            &logged_user_uuid,
            &component_uuid,
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
        let logged_user_uuid: Uuid = user::get_logged_user_uuid(cxt, true)?;

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

        // authorization check
        user::util::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_modification_filesets(
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
    ) -> ServiceResult<Vec<ShowFileOfFileset>> {
        use component_modification::modification_file_from_fileset::service::list::get_files_of_fileset;

        // authorization check
        user::util::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_of_fileset(
            &fileset_uuid,
            &file_uuids,
            &limit,
            &offset,
            conn
        )
    }
}
