use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::company::certificate::model::IptCompanyCertificateData;
use crate::models::company::company_represent::model::{
    IptCompanyRepresentData, SlimCompanyRepresent,
};
use crate::models::company::model::{IptCompanyData, SlimCompany};
use crate::models::relate_ref::file::model::IptPreliminaryFileData;

#[derive(Default)]
pub struct CompanyMutation;

#[Object]
impl CompanyMutation {
    async fn register_company(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyData,
    ) -> ServiceResult<SlimCompany> {
        use crate::models::company::service::register::create_company;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_company(logged_uuid_user, data, conn)?)
    }

    async fn upload_company_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptCompanyCertificateData,
        file_data: IptPreliminaryFileData,
    ) -> ServiceResult<String> {
        use crate::models::company::certificate::service::add::add_certificate;
        // let pool = get_pool(cxt)?;
        // let conn = pool.get().unwrap();
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(add_certificate(
            logged_uuid_user,
            cert_data,
            file_data,
            conn
        )?)
    }

    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        let target_uuid_company = Uuid::parse_str(&data.uuid_company.to_string())?;

        crate::models::company::util::check_company_access(
            &logged_uuid_user,
            &target_uuid_company,
            3,
            conn,
        )?;

        crate::models::company::util::check_is_supplier(&target_uuid_company, conn)?;

        Ok(create_company_represent(data.into(), conn)?)
    }

    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        uuid_company: String,
        uuid_company_represent: String,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        let target_uuid_company = Uuid::parse_str(&uuid_company)?;
        let target_uuid_company_represent = Uuid::parse_str(&uuid_company_represent)?;

        crate::models::company::util::check_company_access(
            &logged_uuid_user,
            &target_uuid_company,
            3,
            conn,
        )?;

        Ok(delete_company_represent(
            target_uuid_company,
            target_uuid_company_represent,
            conn,
        )?)
    }
}
