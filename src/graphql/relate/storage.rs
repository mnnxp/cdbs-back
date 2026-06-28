use crate::auth::AuthContext;
use crate::database::{get_conn, get_pool, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::file::ShowFileRelatedData;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::relate::attributes::IptPaginate;
use crate::models::relate_ref::file::{
    model::DownloadFile,
    service::delete::delete_file_with_check_by_uuid,
    service::list::{get_revisions_by_file_uuid, get_url_by_file_uuid},
    service::update::{confirm_upload, set_active_revision_by_uuid},
};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StorageQuery;
#[derive(Default)]
pub struct StorageMutation;

#[Object]
impl StorageQuery {
    /// Returns a pre-signed URL for downloading a file from storage.
    /// Works only if the file supports versioning, is associated with:
    /// a component, a modification of a component, a set of files, or a standard.
    async fn presigned_url(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<DownloadFile> {
        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_url_by_file_uuid(
            &logged_user_uuid,
            &file_uuid,
            &extract_client_domain(cxt),
            conn,
        )
    }

    /// Returns information about all revisions (versions) of a file.
    async fn show_file_revisions(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowFileRelatedData>> {
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        get_revisions_by_file_uuid(
            &file_uuid,
            &AuthContext::from_graphql(cxt)?.user_uuid(),
            &p,
            &extract_client_domain(cxt),
            conn,
        )
    }
}

#[Object]
impl StorageMutation {
    /// Sets a file as successfully uploaded to the storage.
    /// After successful uploaded is confirmed, the file will be processed.
    async fn upload_completed(
        &self,
        cxt: &Context<'_>,
        file_uuids: Vec<Uuid>,
    ) -> ServiceResult<usize> {
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        if file_uuids.is_empty() {
            return Ok(0); // <-- Not found uuids, just return 0
        }

        let pool = get_pool(cxt)?;

        confirm_upload(&logged_user_uuid, &file_uuids, &pool).await
    }

    /// Sets a specified file revision (versions) as active.
    async fn change_active_file_revision(
        &self,
        cxt: &Context<'_>,
        file_uuid: Uuid,
    ) -> ServiceResult<bool> {
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_active_revision_by_uuid(&logged_user_uuid, &file_uuid, conn)
    }

    /// Removes the specified revision (version) of a file.
    /// If the active revision of a file is deleted, other revisions of the file will not show.
    /// After deleting the active revision without activating the other one,
    /// uploading a new file with the same name will be the solution to view other (inactive) revisions of the file.
    async fn delete_file(&self, cxt: &Context<'_>, file_uuid: Uuid) -> ServiceResult<bool> {
        let logged_user_uuid = AuthContext::from_graphql(cxt)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_file_with_check_by_uuid(&logged_user_uuid, &file_uuid, conn)
    }
}
