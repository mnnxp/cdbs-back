use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::company_represent::service as company_represent;
use crate::models::company::model::{CompanyAndRelatedData, ShowCompanyShort};
use crate::models::company::service as company;
use crate::models::user;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct CompanyQuery;

#[Object]
impl CompanyQuery {
    async fn companies(
        &self,
        cxt: &Context<'_>,
        companies_uuids: Vec<Uuid>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        // todo!(need set check limit length vec)

        let conn: &PooledConnection = &get_conn(cxt)?;

        company::list::find_companies(
            &companies_uuids,
            &logged_user_uuid,
            &crate::models::user::get_set_language(cxt),
            conn,
        )
    }

    async fn company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // authorization check
        let logged_user_uuid = user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        company::list::find_by_uuid(
            &company_uuid,
            &logged_user_uuid,
            &crate::models::user::get_set_language(cxt),
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
        user::util::check_authorized(cxt)?;

        // todo!(check access)

        let conn: &PooledConnection = &get_conn(cxt)?;

        // Representative offices are selected by company uuid or by representative uuid
        match (company_uuid, represents_uuids) {
            (Some(company_uuid), None) => {
                company_represent::list::get_by_company_uuid(
                    &company_uuid,
                    &crate::models::user::get_set_language(cxt),
                    conn,
                )
            }
            (None, Some(represents_uuids)) => {
                company_represent::list::get_represent_by_uuids(
                    &represents_uuids,
                    &crate::models::user::get_set_language(cxt),
                    conn,
                )
            }
            _ => Err(crate::errors::ServiceError::BadRequest(
                "You need to choose a company or a representative company".to_string(),
            )),
        }
    }
}
