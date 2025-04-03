use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::graphql::service_model::{IptServiceData, IptUpdateServiceData};
use crate::models::supplier_service::service::update::change_service_status;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::user::model::IptUserData;
use crate::models::user::service::register::create_user;
use crate::models::supplier_service::{
    spec::model::IptServiceSpecsData,
    spec::service::{add::add_service_specs, delete::del_service_specs},
    keyword::model::{IptServiceKeywordsData, IptServiceKeywordsNames},
    keyword::service::add::{add_service_keywords, add_keywords_by_names},
    keyword::service::delete::del_service_keywords,
    file::model::{IptServiceFilesData, DeleteServiceFileData},
    file::service::{add::add_service_files, delete::delete_service_file},
    service::{register::create_service, update::update_service_data},
    service::delete::del_service_data,
    access::company::manage::set_company_access_service,
    param::model::{IptServiceParamsData, DelServiceParamData},
    param::service::{change::put_service_params, delete::del_service_params},
};
use crate::models::supplier_service::access::{
    model::{ChangeOwnerService, ChangeTypeAccessService},
    company::model::{IptCompanyAccessServiceData, DelCompanyAccessServiceData},
    company::manage::del_company_access_service,
    user::model::{IptUserAccessServiceData, DelUserAccessServiceData},
    user::manage::{set_user_access_service, del_user_access_service},
    manage::{change_service_owner_user, change_service_type_access},
};
use crate::models::search::model::ExtraOptions;
use crate::models::relate_ref::file::model::UploadFile;
use async_graphql::{self, Context, Object};
use uuid::Uuid;

use super::service_model::IptServiceStatusArg;

#[derive(Default)]
pub struct ServiceMutation;

#[Object]
impl ServiceMutation {
    /// Creates a service, returns the UUID of the new service
    async fn service_request(
        &self,
        cxt: &Context<'_>,
        args: IptServiceData,
        new_user: Option<IptUserData>,
    ) -> ServiceResult<Uuid> {
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        let options = match new_user {
            Some(nu) => {
                let mut user_data = nu;
                if user_data.password.is_empty() {
                    user_data.password = format!("{:.7}", blake3::hash(user_data.username.as_bytes()));
                }
                ExtraOptions::by_slim_user(cxt, &create_user(user_data, conn)?)
            },
            None => ExtraOptions::from_cxt(cxt, false)?
        };
        create_service(&args, &options, conn)
    }

    /// Transfers ownership of a service to another user
    async fn transfer_service_ownership(
        &self,
        cxt: &Context<'_>,
        args: ChangeOwnerService,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        change_service_owner_user(&args, &options, conn)
    }

    /// Changes the default access to a service
    async fn change_service_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessService,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        change_service_type_access(&args, &options, conn)
    }

    /// Changes the default status to a service
    async fn change_service_status(
        &self,
        cxt: &Context<'_>,
        args: IptServiceStatusArg,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        change_service_status(&args, &options, conn)
    }

    /// Updates the service's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_service_update(
        &self,
        cxt: &Context<'_>,
        service_uuid: Uuid,
        args: IptUpdateServiceData,
    ) -> ServiceResult<usize> {
        // checking authorization
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        update_service_data(&service_uuid, &args, &options, conn)
    }

    /// Deletes a service and its associated data.
    /// Returns the UUID of the removed service.
    async fn delete_service(
        &self,
        cxt: &Context<'_>,
        service_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
        // checking authorization
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_service_data(&service_uuid, &logged_user_uuid, conn)
    }

    /// Adds new parameters with values ​​for a component service.
    /// Updates the values ​​of existing service parameters if the provided parameter names already exist.
    async fn put_service_params(
        &self,
        cxt: &Context<'_>,
        args: IptServiceParamsData,
    ) -> ServiceResult<usize> {
        // checking authorization
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        put_service_params(&args, &options, conn)
    }

    /// Deletes parameters of a component service
    async fn delete_service_params(
        &self,
        cxt: &Context<'_>,
        args: DelServiceParamData,
    ) -> ServiceResult<usize> {
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_service_params(&args, &logged_user_uuid, conn)
    }

    /// Sets access to a service for a company.
    /// This access applies to all members of the company according to their roles.
    async fn set_company_access_service(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        set_company_access_service(&args, &logged_user_uuid, conn)
    }

    /// Removes access to a service for a company
    async fn delete_company_access_service(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_company_access_service(&args, &logged_user_uuid, conn)
    }

    /// Sets access to a service for a user
    async fn set_user_access_service(
        &self,
        cxt: &Context<'_>,
        args: IptUserAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        set_user_access_service(&args, &logged_user_uuid, conn)
    }

    /// Removes access to a service for a user
    async fn delete_user_access_service(
        &self,
        cxt: &Context<'_>,
        args: DelUserAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_user_access_service(&args, &logged_user_uuid, conn)
    }

    /// Adds a service connection to directory sections
    async fn add_service_specs(
        &self,
        cxt: &Context<'_>,
        args: IptServiceSpecsData,
    ) -> ServiceResult<i32> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        add_service_specs(&args, &logged_user_uuid, conn)
    }

    /// Removes a service's association with catalogs
    async fn delete_service_specs(
        &self,
        cxt: &Context<'_>,
        args: IptServiceSpecsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_service_specs(&args, &logged_user_uuid, conn)
    }

    /// Adds keywords to a service by IDs
    async fn add_service_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptServiceKeywordsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        add_service_keywords(&args, &logged_user_uuid, conn)
    }

    /// Adds keywords to a service by words
    async fn add_service_keywords_by_names(
        &self,
        cxt: &Context<'_>,
        args: IptServiceKeywordsNames,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        add_keywords_by_names(&args, &logged_user_uuid, conn)
    }

    /// Removes keywords from a service
    async fn delete_service_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptServiceKeywordsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        del_service_keywords(&args, &logged_user_uuid, conn)
    }

    /// Creates preliminary files information for a service.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_service_files(
        &self,
        cxt: &Context<'_>,
        args: IptServiceFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        add_service_files(&args, &logged_user_uuid, conn)
    }

    /// Deletes a file of a service
    async fn delete_service_file(
        &self,
        cxt: &Context<'_>,
        args: DeleteServiceFileData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        delete_service_file(&args, &logged_user_uuid, conn)
    }
}
