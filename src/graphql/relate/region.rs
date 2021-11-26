use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::region::{
    model::{IptRegionTranslateListData, RegionTranslateList},
    service::list::get_regions,
    service::register::create_region,
};
use crate::models::relate_ref::language::get_set_language;

#[derive(Default)]
pub struct RegionQuery;
#[derive(Default)]
pub struct RegionMutation;

#[Object]
impl RegionQuery {
    async fn regions(
        &self,
        cxt: &Context<'_>,
        region_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        // authorization check
        // user::util::check_authorized(cxt)?;

        let region_id: Vec<i32> = region_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_regions(
            &region_id,
            &limit,
            &offset,
            &get_set_language(cxt),
            conn,
        )
    }
}

#[Object]
impl RegionMutation {
    async fn register_region(
        &self,
        cxt: &Context<'_>,
        data: IptRegionTranslateListData,
    ) -> ServiceResult<RegionTranslateList> {
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_region(&data, conn)
    }
}
