use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::graphql::handler::extract_client_domain;
use crate::graphql::relate::attributes::IptPaginate;
use crate::graphql::standard_model::{
    IptStandardFilesArg, IptStandardsArg, ShowStandardShort, StandardAndRelatedData,
};
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, language::get_set_language,
    spec::model::SpecTranslateList,
};
use crate::models::search::model::ExtraOptions;
use crate::models::search::order::Paginate;
use crate::models::standard::{
    access::company::model::CompanyAccessStandardAndRelatedData,
    access::user::model::UserAccessStandardAndRelatedData,
    model::{StandardFilesArg, StandardsArg},
    relate::standard_status::model::StandardStatusTranslateList,
};
use crate::models::user::access::logged::{check_authorized, get_logged_user_uuid};
use async_graphql::{self, Context, Object};
use uuid::Uuid;

#[derive(Default)]
pub struct StandardQuery;

#[Object]
impl StandardQuery {
    /// Returns brief information about standards with filter by:
    /// UUIDs, company, user, favorite (for self or other user).
    async fn standards(
        &self,
        cxt: &Context<'_>,
        args: Option<IptStandardsArg>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        use crate::models::standard::service::list::get_standard;

        // authorization check
        let options = ExtraOptions::from_cxt(cxt, false)?;
        let arguments = StandardsArg::by_arg(args);
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_standard(&arguments, &options, &p, conn)
    }

    /// Returns complete information about the standard by UUID.
    async fn standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        // sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<StandardAndRelatedData> {
        use crate::models::standard::service::list::find_by_uuid;
        let options = ExtraOptions::from_cxt(cxt, false)?;
        // let s = sort.map(|s| Sort::parsing(TableName::StandardRef, &s.by_field, s.as_desc))
        //     .unwrap_or(Sort::set_by_table(TableName::StandardRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        find_by_uuid(&standard_uuid, &options, &p, conn)
    }

    /// Returns pre-signed URLs and other information for downloading standard files.
    async fn standard_files(
        &self,
        cxt: &Context<'_>,
        args: IptStandardFilesArg,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<DownloadFile>> {
        use crate::models::standard::file::service::list::get_standard_files;
        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let arguments: StandardFilesArg = args.into();
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_standard_files(&logged_user_uuid, &arguments, &p, &extract_client_domain(cxt),  conn)
    }

    /// Returns an array of catalogs associated with standard
    async fn standard_specs(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        use crate::models::standard::spec::service::list::get_standard_specs;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_standard_specs(
            &logged_user_uuid,
            &standard_uuid,
            get_set_language(cxt),
            &p,
            conn,
        )
    }

    /// Returns array of keywords associated with the standard.
    async fn standard_keywords(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
        paginate: Option<IptPaginate>,
    ) -> ServiceResult<Vec<Keyword>> {
        use crate::models::standard::keyword::service::list::get_standard_keywords;

        // authorization check
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;
        get_standard_keywords(&logged_user_uuid, &standard_uuid, &p, conn)
    }

    /// Returns a list of companies that have access to a standard.
    async fn get_companies_list_access_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Vec<CompanyAccessStandardAndRelatedData>> {
        use crate::models::standard::access::company::manage::get_companies_list_access_standard;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_companies_list_access_standard(
            &logged_user_uuid,
            &standard_uuid,
            get_set_language(cxt),
            conn,
        )
    }

    /// Returns a list of users who have access to a standard.
    async fn get_users_list_access_standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: Uuid,
    ) -> ServiceResult<Vec<UserAccessStandardAndRelatedData>> {
        use crate::models::standard::access::user::manage::get_users_list_access_standard;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_users_list_access_standard(
            &logged_user_uuid,
            &standard_uuid,
            get_set_language(cxt),
            conn,
        )
    }

    /// Returns a list of available states (statuses) for standards.
    /// Filtering by standard actual status IDs is available.
    async fn standard_statuses(
        &self,
        cxt: &Context<'_>,
        filter: Option<Vec<i32>>,
    ) -> ServiceResult<Vec<StandardStatusTranslateList>> {
        use crate::models::standard::relate::standard_status::service::list::get_standard_statuses;

        check_authorized(cxt)?; // checking authorization

        let filter: Vec<i32> = filter.unwrap_or_default();
        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        get_standard_statuses(&filter, get_set_language(cxt), conn)
    }
}
