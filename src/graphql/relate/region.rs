use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::region::{
    model::{IptRegionTranslateListData, RegionTranslateList, IptRegionArg, RegionArg},
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
        args: Option<IptRegionArg>,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        let arguments = match args {
            Some(x) => RegionArg::from(x),
            None => RegionArg::default(),
        };

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_regions(&arguments, &get_set_language(cxt), conn)
    }
}

#[Object]
impl RegionMutation {
    async fn register_region(
        &self,
        cxt: &Context<'_>,
        args: IptRegionTranslateListData,
    ) -> ServiceResult<RegionTranslateList> {
        check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_region(&args, conn)
    }
}
