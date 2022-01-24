use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::standard::{
    model::{IptStandardData, IptUpdateStandardData},
    access::model::{ChangeOwnerStandard, ChangeTypeAccessStandard},
    access::company::model::{IptCompanyAccessStandardData, DelCompanyAccessStandardData},
    access::user::model::{IptUserAccessStandardData, DelUserAccessStandardData},
    spec::model::IptStandardSpecsData,
    keyword::model::{IptStandardKeywordsData, IptStandardKeywordsNames},
    file::model::{IptStandardFilesData, IptStandardFaviconData, DeleteStandardFileData},
};
use crate::models::relate_ref::file::model::UploadFile;
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardMutation;

#[Object]
impl StandardMutation {
    async fn register_standard(
        &self,
        cxt: &Context<'_>,
        args: IptStandardData,
    ) -> ServiceResult<Uuid> {
        use crate::models::standard::service::register::create_standard;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Transfer component ownership to another user
    async fn transfer_standard_ownership(
        &self,
        cxt: &Context<'_>,
        args: ChangeOwnerStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_owner_user;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_standard_owner_user(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Change standard type access
    async fn change_standard_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_standard_type_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn put_standard_update(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        args: IptUpdateStandardData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::service::update::update_standard_data;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            &args,
            conn
        )
    }

    async fn delete_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
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
        args: IptCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::set_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_company_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_access_standard(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::del_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn set_user_access_standard(
        &self,
        cxt: &Context<'_>,
        args: IptUserAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::user::manage::set_user_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_user_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_user_access_standard(
        &self,
        cxt: &Context<'_>,
        args: DelUserAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::user::manage::del_user_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_user_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }
    // End Manage access component

    async fn add_standard_specs(
        &self,
        cxt: &Context<'_>,
        args: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::add::add_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_standard_specs(
        &self,
        cxt: &Context<'_>,
        args: IptStandardSpecsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::spec::service::delete::del_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn add_standard_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::add::add_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn add_standard_keywords_by_names(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsNames,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::add::add_keywords_by_names;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_keywords_by_names(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_standard_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::delete::del_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standard_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn upload_standard_files(
        &self,
        cxt: &Context<'_>,
        args: IptStandardFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use crate::models::standard::file::service::add::add_standard_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_files(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn upload_standard_favicon(
        &self,
        cxt: &Context<'_>,
        args: IptStandardFaviconData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::standard::file::service::add::add_standard_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_favicon(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_standard_file(
        &self,
        cxt: &Context<'_>,
        args: DeleteStandardFileData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::file::service::delete::delete_standard_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_standard_file(
            &logged_user_uuid,
            &args,
            conn
        )
    }
}
