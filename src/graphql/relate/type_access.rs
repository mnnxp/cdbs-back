use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::{
    language::get_set_language,
    type_access::model::TypeAccessTranslateList,
    type_access::service::list::get_type_access,
};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};

use super::attributes::IptPaginate;

#[derive(Default)]
pub struct TypeAccessQuery;
// #[derive(Default)]
// pub struct TypeAccessMutation;

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
            &get_set_language(cxt),
            &p,
            conn,
        )
    }
}

// #[Object]
// impl TypeAccessMutation {
//     /// Adds a new type access.
//     /// Returns an error with the type access ID if it already exists.
//     async fn register_type_access(
//         &self,
//         cxt: &Context<'_>,
//         args: IptTypeAccessTranslateListData,
//     ) -> ServiceResult<TypeAccessTranslateList> {
//         check_authorized(cxt)?;
//         let conn: &mut PooledConnection = &mut get_conn(cxt)?;
//         create_type_access(&args, conn)
//     }
// }
