use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::service_model::{IptServiceData, IptUpdateServiceData};
use crate::models::relate_ref::file::model::UploadFile;
use crate::models::search::model::ExtraOptions;
use crate::models::supplier_service::access::{
    company::manage::del_company_access_service,
    company::model::{DelCompanyAccessServiceData, IptCompanyAccessServiceData},
    manage::{change_service_owner_user, change_service_type_access},
    model::{ChangeOwnerService, ChangeTypeAccessService},
    user::manage::{del_user_access_service, set_user_access_service},
    user::model::{DelUserAccessServiceData, IptUserAccessServiceData},
};
use crate::models::supplier_service::service::update::change_service_status;
use crate::models::supplier_service::{
    access::company::manage::set_company_access_service,
    file::model::{DeleteServiceFileData, IptServiceFilesData},
    file::service::{add::add_service_files, delete::delete_service_file},
    keyword::model::{IptServiceKeywordsData, IptServiceKeywordsNames},
    keyword::service::add::{add_keywords_by_names, add_service_keywords},
    keyword::service::delete::del_service_keywords,
    param::model::{DelServiceParamData, IptServiceParamsData},
    param::service::{change::put_service_params, delete::del_service_params},
    service::delete::del_service_data,
    service::{register::create_service, update::update_service_data},
    spec::model::IptServiceSpecsData,
    spec::service::{add::add_service_specs, delete::del_service_specs},
};
use crate::models::user::model::IptUserData;
use crate::models::user::service::register::create_user;
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
        ctx: &Context<'_>,
        args: IptServiceData,
        new_user: Option<IptUserData>,
    ) -> ServiceResult<Uuid> {
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        let options = match new_user {
            Some(nu) => {
                let mut user_data = nu;
                if user_data.password.is_empty() {
                    user_data.password =
                        format!("{:.7}", blake3::hash(user_data.username.as_bytes()));
                }
                ExtraOptions::by_slim_user(ctx, &create_user(user_data, conn)?)
            }
            None => ExtraOptions::from_ctx(ctx, false)?,
        };
        create_service(&args, &options, conn)
    }

    /// Transfers ownership of a service to another user
    async fn transfer_service_ownership(
        &self,
        ctx: &Context<'_>,
        args: ChangeOwnerService,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        change_service_owner_user(&args, &options, conn)
    }

    /// Changes the default access to a service
    async fn change_service_access(
        &self,
        ctx: &Context<'_>,
        args: ChangeTypeAccessService,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        change_service_type_access(&args, &options, conn)
    }

    /// Changes the default status to a service
    async fn change_service_status(
        &self,
        ctx: &Context<'_>,
        args: IptServiceStatusArg,
    ) -> ServiceResult<bool> {
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        change_service_status(&args, &options, conn)
    }

    /// Updates the service's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_service_update(
        &self,
        ctx: &Context<'_>,
        service_uuid: Uuid,
        args: IptUpdateServiceData,
    ) -> ServiceResult<usize> {
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        update_service_data(&service_uuid, &args, &options, conn)
    }

    /// Deletes a service and its associated data.
    /// Returns the UUID of the removed service.
    async fn delete_service(&self, ctx: &Context<'_>, service_uuid: Uuid) -> ServiceResult<Uuid> {
        // checking authorization
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_service_data(&service_uuid, &logged_user_uuid, conn)
    }

    /// Adds new parameters with values ​​for a component service.
    /// Updates the values ​​of existing service parameters if the provided parameter names already exist.
    async fn put_service_params(
        &self,
        ctx: &Context<'_>,
        args: IptServiceParamsData,
    ) -> ServiceResult<usize> {
        // checking authorization
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        put_service_params(&args, &options, conn)
    }

    /// Deletes parameters of a component service
    async fn delete_service_params(
        &self,
        ctx: &Context<'_>,
        args: DelServiceParamData,
    ) -> ServiceResult<usize> {
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_service_params(&args, &logged_user_uuid, conn)
    }

    /// Sets access to a service for a company.
    /// This access applies to all members of the company according to their roles.
    async fn set_company_access_service(
        &self,
        ctx: &Context<'_>,
        args: IptCompanyAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting company uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        set_company_access_service(&args, &logged_user_uuid, conn)
    }

    /// Removes access to a service for a company
    async fn delete_company_access_service(
        &self,
        ctx: &Context<'_>,
        args: DelCompanyAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting company uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_company_access_service(&args, &logged_user_uuid, conn)
    }

    /// Sets access to a service for a user
    async fn set_user_access_service(
        &self,
        ctx: &Context<'_>,
        args: IptUserAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        set_user_access_service(&args, &logged_user_uuid, conn)
    }

    /// Removes access to a service for a user
    async fn delete_user_access_service(
        &self,
        ctx: &Context<'_>,
        args: DelUserAccessServiceData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_user_access_service(&args, &logged_user_uuid, conn)
    }

    /// Adds a service connection to directory sections
    async fn add_service_specs(
        &self,
        ctx: &Context<'_>,
        args: IptServiceSpecsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        add_service_specs(&args, &logged_user_uuid, conn)
    }

    /// Removes a service's association with catalogs
    async fn delete_service_specs(
        &self,
        ctx: &Context<'_>,
        args: IptServiceSpecsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_service_specs(&args, &logged_user_uuid, conn)
    }

    /// Adds keywords to a service by IDs
    async fn add_service_keywords(
        &self,
        ctx: &Context<'_>,
        args: IptServiceKeywordsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        add_service_keywords(&args, &logged_user_uuid, conn)
    }

    /// Adds keywords to a service by words
    async fn add_service_keywords_by_names(
        &self,
        ctx: &Context<'_>,
        args: IptServiceKeywordsNames,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        add_keywords_by_names(&args, &logged_user_uuid, conn)
    }

    /// Removes keywords from a service
    async fn delete_service_keywords(
        &self,
        ctx: &Context<'_>,
        args: IptServiceKeywordsData,
    ) -> ServiceResult<usize> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        del_service_keywords(&args, &logged_user_uuid, conn)
    }

    /// Creates preliminary files information for a service.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_service_files(
        &self,
        ctx: &Context<'_>,
        args: IptServiceFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        add_service_files(&args, &logged_user_uuid, &extract_client_domain(ctx), conn)
    }

    /// Deletes a file of a service
    async fn delete_service_file(
        &self,
        ctx: &Context<'_>,
        args: DeleteServiceFileData,
    ) -> ServiceResult<bool> {
        // checking authorization and getting user uuid
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        delete_service_file(&args, &logged_user_uuid, conn)
    }
}
