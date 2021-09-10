use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::errors::ServiceResult;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::company_represent::service as company_represent;
use crate::models::company::model::{CompanyAndRelatedData, ShowCompanyShort};
use crate::models::company::service as company;
use crate::models::user;

#[derive(Default)]
pub struct CompanyQuery;

#[Object]
impl CompanyQuery {
    async fn companies(
        &self,
        cxt: &Context<'_>,
        companies_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        let mut target_companies_uuids = Vec::new();
        for x in companies_uuids.iter() {
            target_companies_uuids.push(Uuid::parse_str(x).unwrap());
        }

        // todo!(need set check limit length vec)

        company::list::find_companies(cxt, &target_companies_uuids, &logged_uuid_user)
    }

    async fn company(
        &self,
        cxt: &Context<'_>,
        company_uuid: String,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        company::list::find_by_uuid(cxt, &Uuid::parse_str(&company_uuid)?, &logged_uuid_user)
    }

    async fn company_represents(
        &self,
        cxt: &Context<'_>,
        company_uuid: Option<String>,
        represents_uuids: Option<Vec<String>>,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        // todo!(check access)

        // Representative offices are selected by company uuid or by representative uuid
        match (company_uuid, represents_uuids) {
            (Some(company_uuid), None) => {
                company_represent::list::get_by_company_uuid(cxt, &Uuid::parse_str(&company_uuid)?)
            }
            (None, Some(represents_uuids)) => {
                let mut target_represents_uuids = Vec::new();
                for x in represents_uuids.iter() {
                    target_represents_uuids.push(Uuid::parse_str(x).unwrap());
                }

                company_represent::list::get_represent_by_uuids(cxt, &target_represents_uuids)
            }
            _ => Err(crate::errors::ServiceError::BadRequest(
                "You need to choose a company or a representative company".to_string(),
            )),
        }
    }
}
