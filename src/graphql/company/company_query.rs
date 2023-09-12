use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::company;
use crate::models::company::{
    model::{CompanyAndRelatedData, ShowCompanyShort, CompaniesArg, IptCompaniesArg},
    member::model::CompanyMemberAndRelatedData,
    member::role::model::RoleMemberAndRelatedData,
    company_type::model::CompanyTypeTranslateList,
    spec::model::{IptCompanySpecsArg, CompanySpecsArg},
    company_represent::model::{CompanyRepresentAndRelatedData, IptCompanyRepresentsArg, CompanyRepresentsArg},
    company_represent::representation_type::model::RepresentationTypeTranslateList,
};
use crate::models::user::access::logged::{get_logged_user_uuid, check_authorized};
use crate::models::relate_ref::{
    spec::model::SpecTranslateList,
    language::get_set_language,
};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct CompanyQuery;

#[Object]
impl CompanyQuery {
    /// Returns brief information about companies with filter by:
    /// UUIDs, user (UUID), favorite (for self or other user).
    async fn companies(
        &self,
        cxt: &Context<'_>,
        args: Option<IptCompaniesArg>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        use company::service::list::get_companies;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: CompaniesArg = match args {
            Some(x) => CompaniesArg::from(x),
            None => CompaniesArg::default(),
        };

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_companies(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn,
        )
    }

    /// Returns basic and associated company data by UUID.
    async fn company(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyAndRelatedData> {
        use company::service::list::find_by_uuid;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        find_by_uuid(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn,
        )
    }

    /// Returns information about company representative offices.
    async fn company_represents(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyRepresentsArg,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        use company::company_represent::service::list::get_represents;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: CompanyRepresentsArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_represents(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns aggregated data about company (community) members.
    async fn company_members(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        use company::member::service::list::get_by_company_uuid;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_by_company_uuid(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns aggregated role data for company members.
    async fn company_roles(
        &self,
        cxt: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<RoleMemberAndRelatedData>> {
        use company::member::role::service::list::get_roles_for_company;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_roles_for_company(
            &logged_user_uuid,
            &company_uuid,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns a list of company types.
    async fn company_types(
        &self,
        cxt: &Context<'_>,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        use company::relate::company_type::service::list::get_types_for_company;

        // authorization check
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_types_for_company(
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns a list of directories associated with the company by UUID.
    async fn company_specs(
        &self,
        cxt: &Context<'_>,
        args: IptCompanySpecsArg,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::company::spec::service::list::get_company_specs;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let arguments: CompanySpecsArg = args.into();

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_company_specs(
            &logged_user_uuid,
            &arguments,
            &get_set_language(cxt),
            conn
        )
    }

    /// Returns a list of types of representative offices (divisions) of companies.
    async fn company_represent_types(
        &self,
        cxt: &Context<'_>,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        use company::company_represent::representation_type::service::list::get_types_for_represent;

        // authorization check
        check_authorized(cxt)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_types_for_represent(
            &get_set_language(cxt),
            conn
        )
    }
}
