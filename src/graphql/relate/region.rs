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
    /// Returns a list of available regions with a filter by IDs.
    /// If a filter is not specified, then all existing ones are aggregated.
    async fn regions(
        &self,
        cxt: &Context<'_>,
        args: Option<IptRegionArg>,
    ) -> ServiceResult<Vec<RegionTranslateList>> {
        let arguments = match args {
            Some(x) => RegionArg::from(x),
            None => RegionArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_regions(&arguments, &get_set_language(cxt), conn)
    }
}

#[Object]
impl RegionMutation {
    /// Adds a new region name. Returns a ID and region name.
    /// Returns an error with the ID specified in the region if the region already exists.
    async fn register_region(
        &self,
        cxt: &Context<'_>,
        args: IptRegionTranslateListData,
    ) -> ServiceResult<RegionTranslateList> {
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_region(&args, conn)
    }
}
