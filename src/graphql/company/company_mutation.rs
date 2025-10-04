use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::company::{
    access::model::ChangeTypeAccessCompany,
    access::role_access::model::{DelRoleAccessData, IptRoleAccessData},
    certificate::model::{
        DelCompanyCertificateData, IptCompanyCertificateData, IptUpdateCompanyCertificateData,
    },
    company_represent::model::{IptCompanyRepresentData, IptUpdateCompanyRepresentData},
    member::model::{DelCompanyMemberData, IptCompanyMemberData},
    member::role::model::{DelRoleMemberData, IptRoleMemberData, IptUpdateNameRoleData},
    model::{IptCompanyData, IptUpdateCompanyData},
    spec::model::IptCompanySpecsData,
    supplier_component::model::DelCompanyOfSuppliersData,
};
use crate::models::component::supplier::model::IptSupplierComponentData;
use crate::models::relate_ref::file::model::UploadFile;
use crate::models::user::access::logged::get_logged_user_uuid;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct CompanyMutation;

#[Object]
impl CompanyMutation {
    /// Creates a company, returns the UUID of the new company.
    async fn register_company(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyData,
    ) -> ServiceResult<Uuid> {
        use crate::models::company::service::register::create_company;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_company(&logged_user_uuid, &args, conn)
    }

    /// Updates baic company data. Returns the number of successful changes.
    async fn put_company_update(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        args: IptUpdateCompanyData,
    ) -> ServiceResult<usize> {
        use crate::models::company::service::update::update_company_by_uuid;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_company_by_uuid(&logged_user_uuid, &company_uuid, &args, conn)
    }

    /// Changes the type of access to the company.
    /// Returns true if the change was successful, and false if the specified access is already installed.
    async fn change_company_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessCompany,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::manage::change_company_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_company_type_access(&logged_user_uuid, &args, conn)
    }

    /// Deletes the company and its associated data. Returns the UUID of the remote company.
    async fn delete_company(&self, cxt: &Context<'_>, company_uuid: Uuid) -> ServiceResult<Uuid> {
        use crate::models::company::service::delete::del_company;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        del_company(&logged_user_uuid, &company_uuid, conn)
    }

    /// Updates the company avatar. Returns a structure with a pre-signed URL for uploading an image file.
    async fn upload_company_favicon(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        filename: String,
    ) -> ServiceResult<UploadFile> {
        use crate::models::company::relate::favicon::update_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_favicon(&logged_user_uuid, &company_uuid, &filename, conn)
    }

    /// Uploading a new company certificate. Returns a structure with a pre-signed URL for uploading a certificate file.
    async fn upload_company_certificate(
        &self,
        cxt: &Context<'_>,
        cert_data: IptCompanyCertificateData,
    ) -> ServiceResult<UploadFile> {
        use crate::models::company::certificate::service::add::add_certificate;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_certificate(&logged_user_uuid, &cert_data, conn)
    }

    /// Updates a company certificate description.
    /// Returns true if the change was successful, and false if the certificate description is already installed.
    async fn update_company_certificate(
        &self,
        cxt: &Context<'_>,
        args: IptUpdateCompanyCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::company::certificate::service::update::update_certificate_description;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_certificate_description(&logged_user_uuid, &args, conn)
    }

    /// Removes a company certificate.
    async fn delete_company_certificate(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyCertificateData,
    ) -> ServiceResult<bool> {
        use crate::models::company::certificate::service::delete::del_certificate;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_certificate(&logged_user_uuid, &args, conn)
    }

    /// Adds company connections to specified directory sections.
    /// Returns the number of successful connections.
    /// And an error will be returned if all connections have already been added.
    async fn add_company_specs(
        &self,
        cxt: &Context<'_>,
        args: IptCompanySpecsData,
    ) -> ServiceResult<i32> {
        use crate::models::company::spec::service::add::add_company_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_company_specs(&logged_user_uuid, &args, conn)
    }

    /// Removes a company's association with catalogs
    async fn delete_company_specs(
        &self,
        cxt: &Context<'_>,
        args: IptCompanySpecsData,
    ) -> ServiceResult<usize> {
        use crate::models::company::spec::service::delete::del_company_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_company_specs(&logged_user_uuid, &args, conn)
    }

    /// Adding information about the company's representative office.
    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyRepresentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::company_represent::service::register::create_company_represent;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_company_represent(&logged_user_uuid, &args, conn)
    }

    /// Updating information about the company's representative office.
    /// Returns the number of successful changes.
    /// And an error will be returned if all sent data has already been sent.
    async fn update_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
        args: IptUpdateCompanyRepresentData,
    ) -> ServiceResult<usize> {
        use crate::models::company::company_represent::service::update::update_company_represent_by_uuid;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_company_represent_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &company_represent_uuid,
            &args,
            conn,
        )
    }

    /// Deletes information about the company's representative office.
    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
        company_represent_uuid: Uuid,
    ) -> ServiceResult<bool> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_company_represent(
            &logged_user_uuid,
            &company_uuid,
            &company_represent_uuid,
            conn,
        )
    }

    /// Adds a company (community) member.
    /// A company member will have authorized access to all company components and standards.
    /// Returns an error if this user is already a member of the company.
    async fn add_company_member(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::add::add_company_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_company_member(&logged_user_uuid, &args, conn)
    }

    /// Changes the role type of a company member.
    async fn change_role_member(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::change::change_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_role_member(&logged_user_uuid, &args, conn)
    }

    /// Removes a company member.
    /// After deleting, the user will not have access to closed company objects.
    async fn delete_company_member(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyMemberData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::service::delete::del_company_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_company_member(&logged_user_uuid, &args, conn)
    }

    /// Creates a new role in the specified company.
    async fn register_company_role(
        &self,
        cxt: &Context<'_>,
        args: IptRoleMemberData,
    ) -> ServiceResult<i32> {
        use crate::models::company::member::role::service::register::create_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_role_member(&logged_user_uuid, &args, conn)
    }

    /// Updates the name of the specified role for company members.
    async fn change_name_role_company(
        &self,
        cxt: &Context<'_>,
        args: IptUpdateNameRoleData,
    ) -> ServiceResult<bool> {
        use crate::models::company::member::role::service::update::change_name_role_company;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_name_role_company(&logged_user_uuid, &args, conn)
    }

    /// Removes the role of company members.
    async fn delete_company_role(
        &self,
        cxt: &Context<'_>,
        args: DelRoleMemberData,
    ) -> ServiceResult<usize> {
        use crate::models::company::member::role::service::delete::del_role_member;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_role_member(&logged_user_uuid, &args, conn)
    }

    /// Adds access rights to the company member role.
    async fn add_access_role(
        &self,
        cxt: &Context<'_>,
        args: IptRoleAccessData,
    ) -> ServiceResult<bool> {
        use crate::models::company::access::role_access::service::register::create_role_access;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_role_access(&logged_user_uuid, &args, conn)
    }

    /// Removes access rights of the company member role.
    async fn delete_access_role(
        &self,
        cxt: &Context<'_>,
        args: DelRoleAccessData,
    ) -> ServiceResult<usize> {
        use crate::models::company::access::role_access::service::delete::del_role_access;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_role_access(&logged_user_uuid, &args, conn)
    }

    /// Adds company to suppliers list a component.
    async fn add_component_supplier(
        &self,
        cxt: &Context<'_>,
        args: IptSupplierComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::add::add_company_to_suppliers;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_company_to_suppliers(&logged_user_uuid, &args, conn)
    }

    /// Sets the company as the primary supplier of the component.
    async fn set_company_owner_supplier(
        &self,
        cxt: &Context<'_>,
        args: IptSupplierComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::add::set_company_owner_supplier;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_company_owner_supplier(&logged_user_uuid, &args, conn)
    }

    /// Removes a company from the list of suppliers.
    async fn delete_supplier_company(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyOfSuppliersData,
    ) -> ServiceResult<bool> {
        use crate::models::company::supplier_component::delete::del_company_of_suppliers;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_company_of_suppliers(&logged_user_uuid, &args, conn)
    }
}
