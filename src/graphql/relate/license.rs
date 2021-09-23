use async_graphql::{self, Context, Object};

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::relate_ref::license;
use crate::models::relate_ref::license::model::{License, LicenseData};
use crate::models::user;

#[derive(Default)]
pub struct LicenseQuery;
#[derive(Default)]
pub struct LicenseMutation;

#[Object]
impl LicenseQuery {
    async fn licenses(
        &self,
        cxt: &Context<'_>,
        license_id: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<License>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let license_id: Vec<i32> = license_id.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        license::service::list::get_licenses(cxt, license_id, limit, offset)
    }
}

#[Object]
impl LicenseMutation {
    async fn register_license(
        &self,
        cxt: &Context<'_>,
        data: LicenseData,
    ) -> ServiceResult<License> {
        use license::service::register::create_license;
        let conn: &PooledConnection = &get_conn(cxt)?;

        // todo!(check owned company)
        crate::models::user::check_authorized(cxt)?;

        create_license(data, conn)
    }
}
