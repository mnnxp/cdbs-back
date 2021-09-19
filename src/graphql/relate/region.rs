use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::region;
use crate::models::relate_ref::region::model::{IptRegionTranslateListData, RegionTranslateList};
use crate::models::user;

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
        user::util::check_authorized(cxt)?;

        let region_id: Vec<i32> = region_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        region::service::list::get_regions(cxt, region_id, limit, offset)
    }
}

#[Object]
impl RegionMutation {
    async fn register_region(
        &self,
        cxt: &Context<'_>,
        data: IptRegionTranslateListData,
    ) -> ServiceResult<RegionTranslateList> {
        use region::service::register::create_region;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_region(data, conn)?)
    }
}
