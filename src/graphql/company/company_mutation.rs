use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::company::model::{
    IptCompanyData, IptUpdateCompanyData, SlimCompany
};
use crate::models::company::access::role_access::model::{IptRoleAccessData, DelRoleAccessData};
use crate::models::company::certificate::model::{IptCompanyCertificateData, IptUpdateCompanyCertificateData};
use crate::models::company::spec::model::IptCompanySpecData;
use crate::models::company::company_represent::model::{
    IptCompanyRepresentData, IptUpdateCompanyRepresentData, SlimCompanyRepresent
};
use crate::models::company::member::model::{
    IptCompanyMemberData, SlimCompanyMember, DelCompanyMemberData
};
use crate::models::company::member::role::model::{
    IptRoleMemberData, IptUpdataNameRoleData, DelRoleMemberData
};
use crate::models::relate_ref::file::model::UploadFile;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

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

        create_company(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_company_update(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        data: IptUpdateCompanyData,
    ) -> ServiceResult<i32> {
        use crate::models::company::service::update::update_company_by_uuid;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        update_company_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &data,
            conn
        )
    }

    async fn delete_company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<SlimCompany> {
        use crate::models::company::service::delete::del_company;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company(
            &logged_user_uuid,
            &company_uuid,
            conn
        )
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

    /// Update company certificate description
    async fn update_company_certificate(
        &self,
        cxt: &Context<'_>,
        data: IptUpdateCompanyCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::company::certificate::service::update::update_certificate_description;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        update_certificate_description(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_company_specs(
        &self,
        cxt: &Context<'_>,
        data: IptCompanySpecData,
    ) -> ServiceResult<i32> {
        use crate::models::company::spec::service::add::add_company_specs;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_company_specs(
        &self,
        cxt: &Context<'_>,
        data: IptCompanySpecData,
    ) -> ServiceResult<i32> {
        use crate::models::company::spec::service::delete::del_company_specs;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_company_represent(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn update_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
        data: IptUpdateCompanyRepresentData,
    ) -> ServiceResult<i32> {
        use crate::models::company::company_represent::service::update::update_company_represent_by_uuid;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_company_represent_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &company_represent_uuid,
            &data,
            conn
        )
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

        delete_company_represent(
            &logged_user_uuid,
            &company_uuid,
            &company_represent_uuid,
            conn,
        )
    }

    async fn add_company_member(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyMemberData,
    ) -> ServiceResult<SlimCompanyMember> {
        use crate::models::company::member::service::add::add_company_member;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_member(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    /// Change access role for member
    async fn change_role_member(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::change::change_role_member;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_role_member(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_company_member(
        &self,
        cxt: &Context<'_>,
        data: DelCompanyMemberData,
    ) -> ServiceResult<SlimCompanyMember> {
        use crate::models::company::member::service::delete::del_company_member;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_member(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn register_company_role(
        &self,
        cxt: &Context<'_>,
        data: IptRoleMemberData,
    ) -> ServiceResult<i32> {
        use crate::models::company::member::role::service::register::create_role_member;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_role_member(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn change_name_role_company(
        &self,
        cxt: &Context<'_>,
        data: IptUpdataNameRoleData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::role::service::update::change_name_role_company;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_name_role_company(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_company_role(
        &self,
        cxt: &Context<'_>,
        data: DelRoleMemberData,
    ) -> ServiceResult<i32> {
        use crate::models::company::member::role::service::delete::del_role_member;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_role_member(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_access_role(
        &self,
        cxt: &Context<'_>,
        data: IptRoleAccessData,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::role_access::service::register::create_role_access;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_role_access(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_access_role(
        &self,
        cxt: &Context<'_>,
        data: DelRoleAccessData,
    ) -> ServiceResult<i32> {
        use crate::models::company::access::role_access::service::delete::del_role_access;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_role_access(
            &logged_user_uuid,
            &data,
            conn
        )
    }
}
