use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::type_access;
use crate::models::relate_ref::type_access::model::{IptTypeAccessTranslateListData, TypeAccessTranslateList};
use crate::models::user;

#[derive(Default)]
pub struct TypeAccessQuery;
#[derive(Default)]
pub struct TypeAccessMutation;

#[Object]
impl TypeAccessQuery {
    async fn type_accesss(
        &self,
        cxt: &Context<'_>,
        type_access_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<TypeAccessTranslateList>> {
        // authorization check
        // user::util::check_authorized(cxt)?;

        let type_access_id: Vec<i32> = type_access_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        type_access::service::list::get_type_accesss(
            type_access_id,
            limit,
            offset,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }
}

#[Object]
impl TypeAccessMutation {
    async fn register_type_access(
        &self,
        cxt: &Context<'_>,
        data: IptTypeAccessTranslateListData,
    ) -> ServiceResult<TypeAccessTranslateList> {
        use type_access::service::register::create_type_access;
        let conn: &PooledConnection = &get_conn(cxt)?;

        user::check_authorized(cxt)?;

        create_type_access(data, conn)
    }
}
