use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::standard::model::{
    ShowStandardShort, StandardAndRelatedData, StandardsArg, StandardsQueryArg,
};
use crate::models::standard::access::company::model::CompanyAccessStandardAndRelatedData;
use crate::models::standard::access::user::model::UserAccessStandardAndRelatedData;
use crate::models::relate_ref::language::get_set_language;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardQuery;

#[Object]
impl StandardQuery {
    async fn standards(
        &self,
        cxt: &Context<'_>,
        arguments: Option<StandardsQueryArg>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        use crate::models::standard::service::list::get_standard;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: StandardsArg = match arguments {
            Some(args) => StandardsArg::from(args),
            None => StandardsArg::default(),
        };

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_standard(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<StandardAndRelatedData> {
        use crate::models::standard::service::list::find_by_uuid;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        find_by_uuid(
            &logged_user_uuid,
            &standard_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn get_companies_list_access_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessStandardAndRelatedData>> {
        use crate::models::standard::access::company::manage::get_companies_list_access_standard;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_companies_list_access_standard(
            &logged_user_uuid,
            &standard_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    async fn get_users_list_access_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessStandardAndRelatedData>> {
        use crate::models::standard::access::user::manage::get_users_list_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_users_list_access_standard(
            &logged_user_uuid,
            &standard_uuid,
            &get_set_language(cxt),
            conn
        )
    }
}
