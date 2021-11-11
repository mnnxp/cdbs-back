use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::company::model::{CompanyAndRelatedData, ShowCompanyShort};
use crate::models::company;
use crate::models::company::member::model::CompanyMemberAndRelatedData;
use crate::models::company::member::role::model::RoleMemberAndRelatedData;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::company_represent::service as company_represent;
use crate::models::relate_ref::language::get_set_language;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct CompanyQuery;

#[Object]
impl CompanyQuery {
    async fn companies(
        &self,
        cxt: &Context<'_>,
        companies_uuids: Option<Vec<Uuid>>,
        user_uuid: Option<Uuid>,
        favorite: Option<bool>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        use company::service::list::get_companies;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let companies_uuids: Vec<Uuid> = companies_uuids.unwrap_or_default();

        let favorite: bool = favorite.unwrap_or(false);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_companies(
            &logged_user_uuid,
            &companies_uuids,
            &user_uuid,
            &favorite,
            (&limit, &offset),
            &get_set_language(cxt),
            conn,
        )
    }

    async fn company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyAndRelatedData> {
        use company::service::list::find_by_uuid;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        find_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    async fn company_represents(
        &self,
        cxt: &Context<'_>,
        company_uuid: Option<Uuid>,
        represents_uuids: Option<Vec<Uuid>>,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        // authorization check
        crate::models::user::access::logged::check_authorized(cxt)?;

        // todo!(check access)

        let conn: &PooledConnection = &get_conn(cxt)?;

        // Representative offices are selected by company uuid or by representative uuid
        match (company_uuid, represents_uuids) {
            (Some(company_uuid), None) => {
                company_represent::list::get_by_company_uuid(
                    &company_uuid,
                    &get_set_language(cxt),
                    conn,
                )
            }
            (None, Some(represents_uuids)) => {
                company_represent::list::get_represent_by_uuids(
                    &represents_uuids,
                    &get_set_language(cxt),
                    conn,
                )
            }
            _ => Err(crate::errors::ServiceError::BadRequest(
                "You need to choose a company or a representative company".to_string(),
            )),
        }
    }

    async fn company_members(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        use company::member::service::list::get_by_company_uuid;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_by_company_uuid(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    async fn company_roles(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<RoleMemberAndRelatedData>> {
        use company::member::role::service::list::get_roles_for_company;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        get_roles_for_company(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn
        )
    }
}
