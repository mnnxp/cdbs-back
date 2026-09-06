use crate::auth::token::logged::check_authorized;
use crate::auth::AuthContext;
use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::relate::attributes::IptPaginate;
use crate::models::company;
use crate::models::company::{
    company_represent::model::{
        CompanyRepresentAndRelatedData, CompanyRepresentsArg, IptCompanyRepresentsArg,
    },
    company_represent::representation_type::model::RepresentationTypeTranslateList,
    company_type::model::CompanyTypeTranslateList,
    member::model::CompanyMemberAndRelatedData,
    member::role::model::RoleMemberAndRelatedData,
    model::{CompaniesArg, CompanyAndRelatedData, IptCompaniesArg, ShowCompanyShort},
};
use crate::models::relate_ref::{language::get_set_language, spec::model::SpecTranslateList};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::Paginate;
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
        ctx: &Context<'_>,
        args: Option<IptCompaniesArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        use company::service::list::get_companies;

        // authorization check
        let options = ExtraOptions::from_ctx(ctx, false)?;
        let arguments: CompaniesArg = match args {
            Some(x) => CompaniesArg::by_arg(x),
            None => CompaniesArg::by_lang(),
        };
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_companies(&arguments, &p, &options, conn)
    }

    /// Returns basic and associated company data by UUID.
    async fn company(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyAndRelatedData> {
        use company::service::list::find_by_uuid;
        debug!("Query company: {}", company_uuid);
        // authorization check
        let options = ExtraOptions::from_ctx(ctx, false)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        find_by_uuid(&company_uuid, &options, conn)
    }

    /// Returns the supplier company information and associated UUID data.
    /// Does not require an authorization token, but only works for public companies with supplier status.
    async fn supplier_company(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<CompanyAndRelatedData> {
        use company::service::list::get_supplier_by_uuid;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_supplier_by_uuid(&company_uuid, conn)
    }

    /// Returns information about company representative offices.
    async fn company_represents(
        &self,
        ctx: &Context<'_>,
        args: IptCompanyRepresentsArg,
        // sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        use company::company_represent::service::list::get_represents;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let arguments = CompanyRepresentsArg::by_arg(args, get_set_language(ctx));
        // let s = sort.map(|s| Sort::parsing(TableName::CompanyRepresentRef, &s.by_field, s.as_desc))
        //     .unwrap_or(Sort::set_by_table(TableName::CompanyRepresentRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;
        get_represents(&logged_user_uuid, &arguments, &p, conn)
    }

    /// Returns aggregated data about company (community) members.
    async fn company_members(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyMemberAndRelatedData>> {
        use company::member::service::list::get_by_company_uuid;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_by_company_uuid(
            &logged_user_uuid,
            &company_uuid,
            get_set_language(ctx),
            conn,
        )
    }

    /// Returns aggregated role data for company members.
    async fn company_roles(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
    ) -> ServiceResult<Vec<RoleMemberAndRelatedData>> {
        use company::member::role::service::list::get_roles_for_company;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_roles_for_company(
            &logged_user_uuid,
            &company_uuid,
            get_set_language(ctx),
            conn,
        )
    }

    /// Returns a list of company types.
    async fn company_types(
        &self,
        ctx: &Context<'_>,
    ) -> ServiceResult<Vec<CompanyTypeTranslateList>> {
        use company::relate::company_type::service::list::get_types_for_company;

        // authorization check
        check_authorized(ctx)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_types_for_company(get_set_language(ctx), conn)
    }

    /// Returns a list of directories associated with the company by UUID.
    async fn company_specs(
        &self,
        ctx: &Context<'_>,
        company_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::company::spec::service::list::get_company_specs;

        // authorization check
        let logged_user_uuid = AuthContext::from_graphql(ctx)?.user_uuid();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_company_specs(
            &logged_user_uuid,
            &company_uuid,
            get_set_language(ctx),
            &p,
            conn,
        )
    }

    /// Returns a list of types of representative offices (divisions) of companies.
    async fn company_represent_types(
        &self,
        ctx: &Context<'_>,
    ) -> ServiceResult<Vec<RepresentationTypeTranslateList>> {
        use company::company_represent::representation_type::service::list::get_types_for_represent;

        // authorization check
        check_authorized(ctx)?;

        let conn: &mut PooledConnection = &mut get_conn(ctx)?;

        get_types_for_represent(get_set_language(ctx), conn)
    }
}
