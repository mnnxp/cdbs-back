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
    /// Creates a standard, returns the UUID of the new standard.
    async fn register_standard(
        &self,
        cxt: &Context<'_>,
        args: IptStandardData,
    ) -> ServiceResult<Uuid> {
        use crate::models::standard::service::register::create_standard;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Transfers ownership of a standard to another user.
    async fn transfer_standard_ownership(
        &self,
        cxt: &Context<'_>,
        args: ChangeOwnerStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_owner_user;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_standard_owner_user(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Changes the default access to a standard.
    async fn change_standard_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessStandard,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::manage::change_standard_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_standard_type_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates the standard's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_standard_update(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        args: IptUpdateStandardData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::service::update::update_standard_data;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            &args,
            conn
        )
    }

    /// Deletes a standard and its associated data.
    /// Returns the UUID of the removed standard.
    async fn delete_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
        use crate::models::standard::service::delete::del_standard_data;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_standard_data(
            &logged_user_uuid,
            &standard_uuid,
            conn
        )
    }

    /// Sets access to a standard for a company.
    /// This access applies to all members of the company according to their roles.
    async fn set_company_access_standard(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::set_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_company_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes access to a standard for a company.
    async fn delete_company_access_standard(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::company::manage::del_company_access_standard;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_company_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Sets access to a standard for a user.
    async fn set_user_access_standard(
        &self,
        cxt: &Context<'_>,
        args: IptUserAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::user::manage::set_user_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_user_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes access to a standard for a user.
    async fn delete_user_access_standard(
        &self,
        cxt: &Context<'_>,
        args: DelUserAccessStandardData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::access::user::manage::del_user_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_user_access_standard(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds a standard connection to directory sections.
    async fn add_standard_specs(
        &self,
        cxt: &Context<'_>,
        args: IptStandardSpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::standard::spec::service::add::add_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes a standard's association with directory partitions.
    async fn delete_standard_specs(
        &self,
        cxt: &Context<'_>,
        args: IptStandardSpecsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::spec::service::delete::del_standard_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_standard_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds keywords to a standard by IDs.
    async fn add_standard_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::add::add_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds keywords to a standard by words.
    async fn add_standard_keywords_by_names(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsNames,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::add::add_keywords_by_names;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_keywords_by_names(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes keywords from a standard.
    async fn delete_standard_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptStandardKeywordsData,
    ) -> ServiceResult<usize> {
        use crate::models::standard::keyword::service::delete::del_standard_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_standard_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates preliminary files information for a standard.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_standard_files(
        &self,
        cxt: &Context<'_>,
        args: IptStandardFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use crate::models::standard::file::service::add::add_standard_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_files(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates the main image of the standard.
    /// Returns a structure with a pre-signed URL for uploading an image file.
    async fn upload_standard_favicon(
        &self,
        cxt: &Context<'_>,
        args: IptStandardFaviconData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::standard::file::service::add::add_standard_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_favicon(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Deletes a file of a standard.
    async fn delete_standard_file(
        &self,
        cxt: &Context<'_>,
        args: DeleteStandardFileData,
    ) -> ServiceResult<bool> {
        use crate::models::standard::file::service::delete::delete_standard_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_standard_file(
            &logged_user_uuid,
            &args,
            conn
        )
    }
}
