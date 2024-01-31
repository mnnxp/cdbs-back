use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::check_authorized;
use crate::models::relate_ref::license::{
    model::{License, LicenseData, IptLicenseArg, LicenseArg},
    service::list::get_licenses,
    service::register::create_license,
};

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
        args: Option<IptLicenseArg>,
    ) -> ServiceResult<Vec<License>> {
        check_authorized(cxt)?; // authorization check

        let arguments = match args {
            Some(x) => LicenseArg::from(x),
            None => LicenseArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_licenses(&arguments, conn)
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
