pub mod company_mutation;
pub mod company_query;

pub use company_mutation::*;
pub use company_query::*;

use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::models::company::certificate::model::CompanyCertificateAndFile;
use crate::models::company::company_fav::model::CompanyFav;
use crate::models::company::company_fav::util::check_subscriber_by_uuid;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::company_type::model::CompanyTypeTranslateList;
use crate::models::company::member::model::CompanyMemberAndRelatedData;
use crate::models::company::member::role::model::RoleMemberAndRelatedData;
use crate::models::company::model::{CompanyAndRelatedData, ShowCompanyShort};
use crate::models::relate_ref::file::model::DownloadFile;
use crate::models::relate_ref::region::model::RegionTranslateList;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::relate_ref::type_access::model::TypeAccessTranslateList;
use crate::models::search::model::ExtraOptions;
use crate::models::user::model::ShowUserShort;
use async_graphql::{Context, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Full company information and related data
#[Object]
impl CompanyAndRelatedData {
    /// Company UUID on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Company name
    async fn orgname(&self) -> &String {
        &self.orgname
    }

    /// Abbreviated name
    async fn shortname(&self) -> &String {
        &self.shortname
    }

    /// TIN or other tax identifier of the company
    async fn inn(&self) -> &String {
        &self.inn
    }

    /// Phone number
    async fn phone(&self) -> &String {
        &self.phone
    }

    /// Company e-mail
    async fn email(&self) -> &String {
        &self.email
    }

    /// Company Description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Company address
    async fn address(&self) -> &String {
        &self.address
    }

    /// Company website
    async fn site_url(&self) -> &String {
        &self.site_url
    }

    /// Main time zone
    async fn time_zone(&self) -> &String {
        &self.time_zone
    }

    /// Data on the profile that owns the company
    async fn owner_user(&self, ctx: &Context<'_>) -> ServiceResult<ShowUserShort> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        ShowUserShort::get_without_check_by_uuid(&self.user_uuid, &options.domain, conn)
    }

    /// Data for displaying the company logo
    async fn image_file(&self, ctx: &Context<'_>) -> ServiceResult<DownloadFile> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        DownloadFile::get_by_file_uuid(&self.image_file_uuid, &options.domain, conn)
    }

    /// Main company region
    async fn region(&self, ctx: &Context<'_>) -> ServiceResult<RegionTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        RegionTranslateList::get_region_by_id(self.region_id, options.set_lang_id, conn)
    }

    /// Data on the company's representative offices
    async fn company_represents(
        &self,
        ctx: &Context<'_>,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        CompanyRepresentAndRelatedData::get_by_company_uuid(&self.uuid, options.set_lang_id, conn)
    }

    /// Type of company/society organization
    async fn company_type(&self, ctx: &Context<'_>) -> ServiceResult<CompanyTypeTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        CompanyTypeTranslateList::get_company_type_by_id(
            self.company_type_id,
            options.set_lang_id,
            conn,
        )
    }

    /// List of certificates and competencies of the companies
    async fn company_certificates(
        &self,
        ctx: &Context<'_>,
    ) -> ServiceResult<Vec<CompanyCertificateAndFile>> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        CompanyCertificateAndFile::from_company(&self.uuid, &options.domain, conn)
    }

    /// List of catalogs monitored by the company
    async fn company_specs(&self, ctx: &Context<'_>) -> ServiceResult<Vec<SpecTranslateList>> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        SpecTranslateList::for_company_uuid(&self.uuid, options.set_lang_id, conn)
    }

    /// Type of access to company profile
    async fn type_access(&self, ctx: &Context<'_>) -> ServiceResult<TypeAccessTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        TypeAccessTranslateList::get_type_access_by_id(
            self.type_access_id,
            options.set_lang_id,
            conn,
        )
    }

    /// Supplier status (within the platform)
    async fn is_supplier(&self) -> &bool {
        &self.is_supplier
    }

    /// E-mail confirmation result flag
    async fn is_email_verified(&self) -> &bool {
        &self.is_email_verified
    }

    /// Number of people who have added the company to their bookmarks
    async fn subscribers(&self, ctx: &Context<'_>) -> ServiceResult<i32> {
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        CompanyFav::get_count_followers_by_uuid(&self.uuid, conn)
    }

    /// Flag of company presence in user's bookmarks
    async fn is_followed(&self, ctx: &Context<'_>) -> bool {
        AuthContext::from_graphql(ctx)
            .and_then(|auth| {
                let mut conn = get_conn(ctx)?;
                check_subscriber_by_uuid(&self.uuid, &auth.user_uuid(), &mut conn)
            })
            .unwrap_or(false)
    }

    /// Date of creation of the company profile
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date of update of the company's basic data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

/// Abbreviated company data
#[Object]
impl ShowCompanyShort {
    /// Company UUID on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Abbreviated name
    async fn shortname(&self) -> &String {
        &self.shortname
    }

    /// TIN or other tax identifier of the company
    async fn inn(&self) -> &String {
        &self.inn
    }

    /// Company Description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Data for displaying the company logo
    async fn image_file(&self, ctx: &Context<'_>) -> ServiceResult<DownloadFile> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        DownloadFile::get_by_file_uuid(&self.image_file_uuid, &options.domain, conn)
    }

    /// Main region of the company's activity
    async fn region(&self, ctx: &Context<'_>) -> ServiceResult<RegionTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        RegionTranslateList::get_region_by_id(self.region_id, options.set_lang_id, conn)
    }

    /// Type of company/community organization
    async fn company_type(&self, ctx: &Context<'_>) -> ServiceResult<CompanyTypeTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        CompanyTypeTranslateList::get_company_type_by_id(
            self.company_type_id,
            options.set_lang_id,
            conn,
        )
    }

    /// Type of access to company profile
    async fn type_access(&self, ctx: &Context<'_>) -> ServiceResult<TypeAccessTranslateList> {
        let options = ExtraOptions::from_ctx(ctx, true)?;
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        TypeAccessTranslateList::get_type_access_by_id(
            self.type_access_id,
            options.set_lang_id,
            conn,
        )
    }

    /// Supplier status (within the platform)
    async fn is_supplier(&self) -> &bool {
        &self.is_supplier
    }

    /// Flag of company presence in user's bookmarks
    async fn is_followed(&self, ctx: &Context<'_>) -> bool {
        AuthContext::from_graphql(ctx)
            .and_then(|auth| {
                let mut conn = get_conn(ctx)?;
                check_subscriber_by_uuid(&self.uuid, &auth.user_uuid(), &mut conn)
            })
            .unwrap_or(false)
    }

    /// Date of creation of the company profile
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date of update of the company's basic data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}

/// Company (community) member data
#[Object]
impl CompanyMemberAndRelatedData {
    /// Company UUID
    async fn company_uuid(&self) -> &Uuid {
        &self.company_uuid
    }

    /// User info
    async fn user(&self, ctx: &Context<'_>) -> ShowUserShort {
        let conn: &mut PooledConnection = &mut get_conn(ctx).expect("Error get conn to DB");
        ShowUserShort::get_without_check_by_uuid(&self.user_uuid, &extract_client_domain(ctx), conn)
            .expect("Failed get user short data")
    }

    /// User's role in the company (access rights are granted based on the role)
    async fn company_role(&self) -> &RoleMemberAndRelatedData {
        &self.company_role
    }

    /// Activity flag of the company member
    async fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// Date the user was added to the company
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date the user's role or activity was changed
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
}
