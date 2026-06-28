use crate::auth::permission::PermissionTranslateList;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::{
    language::get_set_language, type_access::model::TypeAccessTranslateList,
    type_access::service::list::get_type_access,
};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};

use super::attributes::IptPaginate;

#[derive(Default)]
pub struct TypeAccessQuery;

#[Object]
impl TypeAccessQuery {
    /// Returns types access by IDs.
    /// If a filter for types access is not specified, then all existing ones are aggregated.
    async fn types_access(
        &self,
        cxt: &Context<'_>,
        type_access_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_type_access(
            &type_access_ids.unwrap_or_default(),
            get_set_language(cxt),
            &p,
            conn,
        )
    }

    /// Returns permission levels for RBAC.
    async fn permissions(
        &self,
        cxt: &Context<'_>,
        permission_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<PermissionTranslateList>> {
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        // Get type accesses (only 1,2,3 for permissions)
        let type_accesses = get_type_access(
            &permission_ids.unwrap_or_default(),
            get_set_language(cxt),
            &p,
            conn,
        )?;

        // Convert to PermissionTranslateList
        let permissions = type_accesses
            .into_iter()
            .filter(|ta| ta.type_access_id >= 1 && ta.type_access_id <= 3) // Only 1,2,3
            .map(PermissionTranslateList::from)
            .collect();

        Ok(permissions)
    }
}
