use super::attributes::IptPaginate;
use crate::auth::token::logged::check_authorized;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::license::{
    model::{License, LicenseData},
    service::list::get_licenses,
    service::register::create_license,
};
use crate::models::search::order::Paginate;
use async_graphql::{self, Context, Object};

#[derive(Default)]
pub struct LicenseQuery;
#[derive(Default)]
pub struct LicenseMutation;

#[Object]
impl LicenseQuery {
    /// Returns a list of available licenses.
    /// If a filter for licenses is not specified, then all existing ones are aggregated.
    async fn licenses(
        &self,
        ctx: &Context<'_>,
        license_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<License>> {
        check_authorized(ctx)?; // authorization check
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_licenses(&license_ids.unwrap_or_default(), &p, conn)
    }
}

#[Object]
impl LicenseMutation {
    /// Adds a new license.
    /// Returns an error with the license ID if it already exists.
    async fn register_license(
        &self,
        ctx: &Context<'_>,
        args: LicenseData,
    ) -> ServiceResult<License> {
        // todo!(check owned company)
        check_authorized(ctx)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        create_license(&args, conn)
    }
}
