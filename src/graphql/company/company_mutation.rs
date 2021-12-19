use crate::database::{get_pool, get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::company::{
    model::{IptCompanyData, IptUpdateCompanyData},
    access::model::ChangeTypeAccessCompany,
    access::role_access::model::{IptRoleAccessData, DelRoleAccessData},
    certificate::model::{IptCompanyCertificateData, IptUpdateCompanyCertificateData, DelCompanyCertificateData},
    spec::model::IptCompanySpecData,
    company_represent::model::{IptCompanyRepresentData, IptUpdateCompanyRepresentData},
    member::model::{IptCompanyMemberData, DelCompanyMemberData},
    member::role::model::{IptRoleMemberData, IptUpdataNameRoleData, DelRoleMemberData},
    supplier_component::model::DelCompanyOfSuppliersData,
};
use crate::models::component::supplier::model::IptSupplierComponentData;
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
        args: IptCompanyData,
    ) -> ServiceResult<Uuid> {
        use crate::models::company::service::register::create_company;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &PooledConnection = &get_conn(cxt)?;

        create_company(&logged_user_uuid, &args, conn)
    }

    async fn put_company_update(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        args: IptUpdateCompanyData,
    ) -> ServiceResult<i32> {
        use crate::models::company::service::update::update_company_by_uuid;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_company_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &args,
            conn
        )
    }

    /// Change company type access
    async fn change_company_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessCompany,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::manage::change_company_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_company_type_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
        use crate::models::company::service::delete::del_company;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        del_company(
            &logged_user_uuid,
            &company_uuid,
            conn
        )
    }

    async fn upload_company_favicon(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        filename: String,
    ) -> ServiceResult<UploadFile> {
        use crate::models::company::relate::favicon::update_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_favicon(
            &logged_user_uuid,
            &company_uuid,
            &filename,
            conn
        )
    }

    async fn upload_company_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptCompanyCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::company::certificate::service::add::add_certificate;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

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
        args: IptUpdateCompanyCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::company::certificate::service::update::update_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_certificate_description(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_certificate(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::company::certificate::service::delete::del_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let pool = get_pool(cxt)?;

        del_certificate_description(
            &logged_user_uuid,
            &args,
            &pool
        ).await
    }

    async fn add_company_specs(
        &self,
        cxt: &Context<'_>,
        args: IptCompanySpecData,
    ) -> ServiceResult<i32> {
        use crate::models::company::spec::service::add::add_company_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_specs(
        &self,
        cxt: &Context<'_>,
        args: IptCompanySpecData,
    ) -> ServiceResult<i32> {
        use crate::models::company::spec::service::delete::del_company_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyRepresentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::company_represent::service::register::create_company_represent;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_company_represent(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn update_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
        args: IptUpdateCompanyRepresentData,
    ) -> ServiceResult<i32> {
        use crate::models::company::company_represent::service::update::update_company_represent_by_uuid;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_company_represent_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &company_represent_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

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
        args: IptCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::add::add_company_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_member(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Change access role for member
    async fn change_role_member(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::change::change_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_role_member(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_member(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::delete::del_company_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_member(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn register_company_role(
        &self,
        cxt: &Context<'_>,
        args: IptRoleMemberData,
    ) -> ServiceResult<i32> {
        use crate::models::company::member::role::service::register::create_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_role_member(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn change_name_role_company(
        &self,
        cxt: &Context<'_>,
        args: IptUpdataNameRoleData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::role::service::update::change_name_role_company;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_name_role_company(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_company_role(
        &self,
        cxt: &Context<'_>,
        args: DelRoleMemberData,
    ) -> ServiceResult<i32> {
        use crate::models::company::member::role::service::delete::del_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_role_member(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn add_access_role(
        &self,
        cxt: &Context<'_>,
        args: IptRoleAccessData,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::role_access::service::register::create_role_access;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_role_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    async fn delete_access_role(
        &self,
        cxt: &Context<'_>,
        args: DelRoleAccessData,
    ) -> ServiceResult<i32> {
        use crate::models::company::access::role_access::service::delete::del_role_access;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_role_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Add company to suppliers list component
    async fn add_component_supplier(
        &self,
        cxt: &Context<'_>,
        args: IptSupplierComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::add::add_company_to_suppliers;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_company_to_suppliers(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Set company as main supplier component
    async fn set_company_owner_supplier(
        &self,
        cxt: &Context<'_>,
        args: IptSupplierComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::add::set_company_owner_supplier;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_company_owner_supplier(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Delete company of suppliers list
    async fn delete_supplier_company(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyOfSuppliersData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::delete::del_company_of_suppliers;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_of_suppliers(
            &logged_user_uuid,
            &args,
            conn
        )
    }
}
