// use crate::cli_args::Opt;
use crate::database::get_pool;
use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::company::model::{CompanyAndRelatedData, ShowCompanyShort};
use crate::models::company::service as company;
use crate::models::company::company_represent::model::CompanyRepresentAndRelatedData;
use crate::models::company::company_represent::service as company_represent;
use crate::models::component::component_modification::file_to_set_modification as component_modification_file_to_set_modification;
use crate::models::component::component_modification::file_to_set_modification::model::FileToSetModification;
use crate::models::component::model::{ComponentAndRelatedData, ShowComponentShort};
use crate::models::component::service as component;
use crate::models::standard::model::{StandardAndRelatedData, ShowStandardShort};
use crate::models::standard as standard;
use crate::models::relate_ref::keyword;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::language;
use crate::models::relate_ref::language::model::Language;
use crate::models::relate_ref::license;
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::param;
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::models::relate_ref::program;
use crate::models::relate_ref::program::model::Program;
use crate::models::relate_ref::spec;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::user::model::{UserAndRelatedData, ShowUserShort, SlimUser, TargetUser};
use crate::models::user as user;
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::service::token::model::UserToken;
// use crate::models::relate_ref::file::model::SlimFile;
use crate::models::relate_ref::file;
use async_graphql::Context;
use uuid::Uuid;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by uuid
    async fn users(
        &self,
        cxt: &Context<'_>,
        users_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowUserShort>> {
        // authorization check
        user::get_logged_uuid_user(cxt, true)?;

        let mut target_users_uuids = Vec::new();
        for x in users_uuids.iter() {
            target_users_uuids.push(Uuid::parse_str(x).unwrap());
        }

        user::service::list::find_users_by_uuids(
            cxt,
            &target_users_uuids
        )
    }

    async fn user(
        &self,
        cxt: &Context<'_>,
        user_uuid: String,
    ) -> ServiceResult<UserAndRelatedData> {
        // authorization check
        let logged_uuid_user: Uuid = user::get_logged_uuid_user(cxt, true)?;

        user::service::list::find_user_by_uuid(
            cxt,
            &Uuid::parse_str(&user_uuid)?,
            &logged_uuid_user,
        )
    }

    // return SlimUser data auth user
    async fn myself(&self, cxt: &Context<'_>) -> ServiceResult<SlimUser> {
        // authorization check
        user::util::check_authorized(cxt)?;
        // get the token of the authorized user
        let token_data = user::service::token::token_from_cxt(cxt)?;
        // decode token
        let token_data = user::service::token::decode(&token_data)?;
        // get SlimUser from jwt
        user::service::token::get_slim_user(token_data)
    }

    async fn show_tokens(&self, cxt: &Context<'_>) -> ServiceResult<Vec<UserToken>> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;
        user::service::token::show_tokens(cxt, logged_uuid_user)
    }

    async fn get_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        user::service::token::update(cxt, false)
    }

    async fn update_token(&self, cxt: &Context<'_>) -> ServiceResult<Token> {
        user::service::token::update(cxt, true)
    }

    async fn decode_token(&self, cxt: &Context<'_>) -> ServiceResult<Claims> {
        // authorization check
        user::util::check_authorized(cxt)?;
        let token = user::service::token::token_from_cxt(cxt)?;
        user::service::token::decode(&token)
    }

    async fn delete_token(&self, cxt: &Context<'_>, token: String) -> ServiceResult<String> {
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;
        let deactivated_tokens = format!(
            "removed {} token.",
            // deactivate all user token
            user::service::token::delete_user_token(cxt, token.as_str(), logged_uuid_user,)?
        );
        Ok(deactivated_tokens)
    }

    async fn delete_all_tokens(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;
        let deactivated_tokens = format!(
            "removed {} tokens.",
            // deactivate all user token
            user::service::token::delete_all_tokens(
                cxt,
                logged_uuid_user,
            )?
        );
        Ok(deactivated_tokens)
    }

    async fn logout(&self, cxt: &Context<'_>) -> ServiceResult<String> {
        // removed user token
        user::service::logout(cxt)
    }

    async fn notifications(
        &self,
        cxt: &Context<'_>,
        id_notification: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Notification>> {
        let id_notification: i32 = id_notification.unwrap_or(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        notification::list::get_notifications(
            cxt,
            id_notification,
            logged_uuid_user,
            limit,
            offset,
        )
    }

    async fn components(
        &self,
        cxt: &Context<'_>,
        components_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // authorization check
        let logged_uuid_user: Uuid = user::get_logged_uuid_user(cxt, true)?;

        let mut target_uuids_components: Vec<Uuid> = Vec::new();
        for x in components_uuids.iter() {
            target_uuids_components.push(Uuid::parse_str(x).unwrap());
        }

        component::list::find_components(
            cxt,
            &target_uuids_components,
            &logged_uuid_user
        )
    }

    async fn component(
        &self,
        cxt: &Context<'_>,
        uuid_component: String,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        let logged_uuid_user: Uuid = user::get_logged_uuid_user(cxt, true)?;

        component::list::find_uuid_component(
            cxt,
            &Uuid::parse_str(&uuid_component)?,
            &logged_uuid_user,
        )
    }

    async fn licenses(
        &self,
        cxt: &Context<'_>,
        id_license: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<License>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_license: Vec<i32> = id_license.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        license::service::list::get_licenses(cxt, id_license, limit, offset)
    }

    async fn param(
        &self,
        cxt: &Context<'_>,
        id_param: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_param: Vec<i32> = id_param.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        param::service::list::get_params(cxt, id_param, limit, offset)
    }

    async fn companies(
        &self,
        cxt: &Context<'_>,
        companies_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        let mut target_companies_uuids = Vec::new();
        for x in companies_uuids.iter() {
            target_companies_uuids.push(Uuid::parse_str(x).unwrap());
        }

        // todo!(need set check limit length vec)

        company::list::find_companies(
            cxt,
            &target_companies_uuids,
            &logged_uuid_user,
        )
    }

    async fn company(
        &self,
        cxt: &Context<'_>,
        company_uuid: String,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        company::list::find_by_uuid(
            cxt,
            &Uuid::parse_str(&company_uuid)?,
            &logged_uuid_user,
        )
    }

    async fn company_represents(
        &self,
        cxt: &Context<'_>,
        company_uuid: Option<String>,
        represents_uuids: Option<Vec<String>>
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        // todo!(check access)

        // Representative offices are selected by company uuid or by representative uuid
        match (company_uuid, represents_uuids) {
            (Some(company_uuid), None) => {
                company_represent::list::get_by_company_uuid(
                    cxt,
                    &Uuid::parse_str(&company_uuid)?,
                )
            },
            (None, Some(represents_uuids)) => {
                let mut target_represents_uuids = Vec::new();
                for x in represents_uuids.iter() {
                    target_represents_uuids.push(
                        Uuid::parse_str(x).unwrap()
                    );
                }

                company_represent::list::get_represent_by_uuids(
                    cxt,
                    &target_represents_uuids,
                )
            },
            _ => Err(
                crate::errors::ServiceError::BadRequest(
                    "You need to choose a company or a representative company".to_string()
                )
            ),
        }
    }

    async fn standards(
        &self,
        cxt: &Context<'_>,
        standards_uuids: Vec<String>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        let mut target_standards_uuids = Vec::new();
        for x in standards_uuids.iter() {
            target_standards_uuids.push(Uuid::parse_str(x).unwrap());
        }

        standard::service::list::find_by_uuids(
            cxt,
            &target_standards_uuids,
            &logged_uuid_user,
        )
    }

    async fn standard(
        &self,
        cxt: &Context<'_>,
        standard_uuid: String,
    ) -> ServiceResult<StandardAndRelatedData> {
        // authorization check
        let logged_uuid_user = user::get_logged_uuid_user(cxt, true)?;

        standard::service::list::find_by_uuid(
            cxt,
            &Uuid::parse_str(&standard_uuid)?,
            &logged_uuid_user,
        )
    }

    async fn language(
        &self,
        cxt: &Context<'_>,
        id_lang: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_lang: Vec<i32> = id_lang.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        language::service::list::get_languages(cxt, id_lang, limit, offset)
    }

    async fn specs(
        &self,
        cxt: &Context<'_>,
        id_spec: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_spec: Vec<i32> = id_spec.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        spec::service::list::get_specs(cxt, id_spec, limit, offset)
    }

    async fn keywords(
        &self,
        cxt: &Context<'_>,
        id_keyword: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_keyword: Vec<i32> = id_keyword.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        keyword::service::list::get_keywords(cxt, id_keyword, limit, offset)
    }

    async fn programs(
        &self,
        cxt: &Context<'_>,
        id_program: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Program>> {
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_program: Vec<i32> = id_program.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        program::service::list::get_programs(cxt, id_program, limit, offset)
    }

    async fn files_set_modification(
        &self,
        cxt: &Context<'_>,
        id_set: Option<i32>,
        // id_set: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FileToSetModification>> {
        use component_modification_file_to_set_modification::service::list::get_files_set_modification;
        // authorization check
        user::util::check_authorized(cxt)?;

        let id_set: i32 = id_set.unwrap_or(0);
        // let id_set: i32 = id_set.unwrap_or_(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_set_modification(cxt, id_set, limit, offset)
    }

    async fn presigned_url(
        &self,
        cxt: &Context<'_>,
        uuid_file: String,
    ) -> ServiceResult<String> {
        let pool = get_pool(cxt)?;

        // authorization check
        let target_user =
            TargetUser::from(&crate::models::user::get_logged_uuid_user(cxt, true)?);

        let target_uuid_file = Uuid::parse_str(&uuid_file).unwrap();

        Ok(file::service::list::get_url_file_by_uuid(target_user, target_uuid_file, pool).await?)
    }
}
