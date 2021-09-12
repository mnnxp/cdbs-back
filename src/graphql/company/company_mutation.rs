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

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(create_company(logged_user_uuid, data, conn)?)
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

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(add_certificate(
            logged_user_uuid,
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

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let target_company_uuid = Uuid::parse_str(&data.company_uuid.to_string())?;

        crate::models::company::util::check_company_access(
            &logged_user_uuid,
            &target_company_uuid,
            3,
            conn,
        )?;

        crate::models::company::util::check_is_supplier(&target_company_uuid, conn)?;

        Ok(create_company_represent(data.into(), conn)?)
    }

    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: String,
        company_uuid_represent: String,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let target_company_uuid = Uuid::parse_str(&company_uuid)?;
        let target_company_represent_uuid = Uuid::parse_str(&company_uuid_represent)?;

        crate::models::company::util::check_company_access(
            &logged_user_uuid,
            &target_company_uuid,
            3,
            conn,
        )?;

        Ok(delete_company_represent(
            target_company_uuid,
            target_company_represent_uuid,
            conn,
        )?)
    }
}
