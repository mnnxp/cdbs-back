use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::get_logged_user_uuid;
use crate::models::standard::model::{IptStandardData, IptUpdateStandardData, SlimStandard};
use crate::models::standard::access::model::{ChangeOwnerStandard, ChangeTypeAccessStandard};
use crate::models::standard::access::company::model::{
    IptCompanyAccessStandardData, DelCompanyAccessStandardData
};
use crate::models::standard::spec::model::IptStandardSpecsData;
use crate::models::standard::keyword::model::IptStandardKeywordsData;
use crate::models::standard::file::model::{IptStandardFilesData, DeleteStandardFileData};
use crate::models::relate_ref::file::model::{UploadFile, DownloadFile};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardMutation;

#[Object]
impl StandardMutation {
    async fn register_standard(
        &self,
        cxt: &Context<'_>,
        data: IptStandardData,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::register::create_standard;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_standard(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    /// Transfer component ownership to another user
    async fn transfer_standard_ownership(
        &self,
        cxt: &Context<'_>,
        data: ChangeOwnerStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_owner_user;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_standard_owner_user(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    /// Change standard type access
    async fn change_standard_access(
        &self,
        cxt: &Context<'_>,
        data: ChangeTypeAccessStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_standard_type_access(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_standard_update(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        data: IptUpdateStandardData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::service::update::update_standard_data;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::delete::del_standard_data;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            conn
        )
    }

    // Start Manage access standard
    async fn set_company_access_standard(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::set_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_company_access_standard(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_company_access_standard(
        &self,
        cxt: &Context<'_>,
        data: DelCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::del_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_access_standard(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_standard_specs(
        &self,
        cxt: &Context<'_>,
        data: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::add::add_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard_specs(
        &self,
        cxt: &Context<'_>,
        data: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::delete::del_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_standard_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptStandardKeywordsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::keyword::service::add::add_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_standard_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptStandardKeywordsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::keyword::service::delete::del_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn upload_standard_files(
        &self,
        cxt: &Context<'_>,
        data: IptStandardFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use crate::models::standard::file::service::add::add_standard_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_files(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn standard_files(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::standard::file::service::list::get_standard_files;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_standard_files(
            &logged_user_uuid,
            &standard_uuid,
            conn
        )
    }

    async fn delete_standard_files(
        &self,
        cxt: &Context<'_>,
        data: DeleteStandardFileData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::file::service::delete::delete_standard_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_standard_file(
            &logged_user_uuid,
            &data,
            conn
        )
    }
}
