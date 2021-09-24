use async_graphql::{self, Context, Object};
use uuid::Uuid;

use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::company::certificate::model::IptCompanyCertificateData;
use crate::models::company::company_represent::model::{IptCompanyRepresentData, SlimCompanyRepresent};
use crate::models::company::model::{IptCompanyData, SlimCompany};
use crate::models::relate_ref::file::model::UploadFile;

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

        create_company(logged_user_uuid, data, conn)
    }

    async fn upload_company_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptCompanyCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::company::certificate::service::add::add_certificate;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        add_certificate(
            &logged_user_uuid,
            &cert_data,
            conn
        )
    }

    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        crate::models::company::util::check_company_access(
            &logged_user_uuid,
            &data.company_uuid,
            3,
            conn,
        )?;

        crate::models::company::util::check_is_supplier(&data.company_uuid, conn)?;

        create_company_represent(data.into(), conn)
    }

    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        crate::models::company::util::check_company_access(
            &logged_user_uuid,
            &company_uuid,
            3,
            conn,
        )?;

        delete_company_represent(
            company_uuid,
            company_represent_uuid,
            conn,
        )
    }
}
