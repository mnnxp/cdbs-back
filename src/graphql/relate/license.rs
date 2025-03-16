use async_graphql::{self, Context, Object};
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::search::order::Paginate;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::license::{
    model::{License, LicenseData},
    service::list::get_licenses,
    service::register::create_license,
};
use super::attributes::IptPaginate;

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
        cxt: &Context<'_>,
        license_ids: Option<Vec<i32>>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<License>> {
        check_authorized(cxt)?; // authorization check
        let p = paginate.map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_licenses(&license_ids.unwrap_or_default(), &p, conn)
    }
}

#[Object]
impl LicenseMutation {
    /// Adds a new license.
    /// Returns an error with the license ID if it already exists.
    async fn register_license(
        &self,
        cxt: &Context<'_>,
        args: LicenseData,
    ) -> ServiceResult<License> {
        // todo!(check owned company)
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_license(&args, conn)
    }
}
