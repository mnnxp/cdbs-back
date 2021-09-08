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
use crate::models::user::model::{ShowUser, SlimUser, TargetUser};
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::service as user;
use crate::models::user::service::token::model::UserToken;
// use crate::models::relate_ref::file::model::SlimFile;
use crate::models::relate_ref::file;
use async_graphql::Context;
use uuid::Uuid;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // get user info by id
    async fn users(
        &self,
        context: &Context<'_>,
        uuid: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowUser>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let uuid_user_create = match uuid {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        user::list::get_users(context, uuid_user_create, limit, offset)
    }

    // return SlimUser data auth user
    async fn myself(&self, context: &Context<'_>) -> ServiceResult<SlimUser> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;
        // get the token of the authorized user
        let token_data = user::token::token_from_context(context)?;
        // decode token
        let token_data = user::token::decode(&token_data)?;
        // get SlimUser from jwt
        user::token::get_slim_user(token_data)
    }

    async fn show_tokens(&self, context: &Context<'_>) -> ServiceResult<Vec<UserToken>> {
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
        user::token::show_tokens(context, auth_uuid_user)
    }

    async fn get_token(&self, context: &Context<'_>) -> ServiceResult<Token> {
        user::token::update(context, false)
    }

    async fn update_token(&self, context: &Context<'_>) -> ServiceResult<Token> {
        user::token::update(context, true)
    }

    async fn decode_token(&self, context: &Context<'_>) -> ServiceResult<Claims> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;
        let token = user::token::token_from_context(context)?;
        user::token::decode(&token)
    }

    async fn delete_token(&self, context: &Context<'_>, token: String) -> ServiceResult<String> {
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
        let deactivated_tokens = format!(
            "removed {} token.",
            // deactivate all user token
            user::token::delete_user_token(context, token.as_str(), auth_uuid_user,)?
        );
        Ok(deactivated_tokens)
    }

    async fn delete_all_tokens(&self, context: &Context<'_>) -> ServiceResult<String> {
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
        let deactivated_tokens = format!(
            "removed {} tokens.",
            // deactivate all user token
            user::token::delete_all_tokens(
                context,
                auth_uuid_user,
            )?
        );
        Ok(deactivated_tokens)
    }

    async fn logout(&self, context: &Context<'_>) -> ServiceResult<String> {
        // removed user token
        user::logout(context)
    }

    async fn notifications(
        &self,
        context: &Context<'_>,
        id_notification: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Notification>> {
        let id_notification: i32 = id_notification.unwrap_or(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        notification::list::get_notifications(
            context,
            id_notification,
            auth_uuid_user,
            limit,
            offset,
        )
    }

    async fn components(
        &self,
        context: &Context<'_>,
        uuid_components: Option<Vec<String>>,
    ) -> ServiceResult<Vec<ShowComponentShort>> {
        // authorization check
        let target_uuid_user: Uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        let mut target_uuids_components: Vec<Uuid> = Vec::new();
        if let Some(vec_string) = uuid_components {
            for x in vec_string.iter() {
                target_uuids_components.push(Uuid::parse_str(x).unwrap());
            }
        };

        component::list::find_components(
            context,
            &target_uuids_components,
            &target_uuid_user
        )
    }

    async fn component(
        &self,
        context: &Context<'_>,
        uuid_component: String,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        let target_user_uuid: Uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        component::list::find_uuid_component(
            context,
            &Uuid::parse_str(&uuid_component)?,
            &target_user_uuid,
        )
    }

    async fn licenses(
        &self,
        context: &Context<'_>,
        id_license: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<License>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_license: Vec<i32> = id_license.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        license::service::list::get_licenses(context, id_license, limit, offset)
    }

    async fn param(
        &self,
        context: &Context<'_>,
        id_param: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamTranslateList>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_param: Vec<i32> = id_param.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        param::service::list::get_params(context, id_param, limit, offset)
    }

    async fn companies(
        &self,
        context: &Context<'_>,
        companies_uuids: Option<Vec<String>>,
    ) -> ServiceResult<Vec<ShowCompanyShort>> {
        // authorization check
        let target_user_uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        let mut target_companies_uuids = Vec::new();
        if let Some(vec_string) = companies_uuids {
            for x in vec_string.iter() {
                target_companies_uuids.push(Uuid::parse_str(x).unwrap());
            }
        };

        // todo!(need set check limit length vec)

        company::list::find_companies(
            context,
            &target_companies_uuids,
            &target_user_uuid,
        )
    }

    async fn company(
        &self,
        context: &Context<'_>,
        company_uuid: String,
    ) -> ServiceResult<CompanyAndRelatedData> {
        // authorization check
        let target_user_uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        company::list::find_by_uuid(
            context,
            &Uuid::parse_str(&company_uuid)?,
            &target_user_uuid,
        )
    }

    async fn company_represents(
        &self,
        context: &Context<'_>,
        company_uuid: Option<String>,
        represents_uuids: Option<Vec<String>>
    ) -> ServiceResult<Vec<CompanyRepresentAndRelatedData>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        // todo!(check access)

        // Representative offices are selected by company uuid or by representative uuid
        match (company_uuid, represents_uuids) {
            (Some(company_uuid), None) => {
                company_represent::list::get_by_company_uuid(
                    context,
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
                    context,
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
        context: &Context<'_>,
        standards_uuids: Option<Vec<String>>,
    ) -> ServiceResult<Vec<ShowStandardShort>> {
        // authorization check
        let target_user_uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        let mut target_standards_uuids = Vec::new();
        if let Some(vec_string) = standards_uuids {
            for x in vec_string.iter() {
                target_standards_uuids.push(Uuid::parse_str(x).unwrap());
            }
        };

        standard::service::list::find_by_uuids(
            context,
            &target_standards_uuids,
            &target_user_uuid,
        )
    }

    async fn standard(
        &self,
        context: &Context<'_>,
        standard_uuid: String,
    ) -> ServiceResult<StandardAndRelatedData> {
        // authorization check
        let target_user_uuid = crate::models::user::get_auth_uuid_user(context, true)?;

        standard::service::list::find_by_uuid(
            context,
            &Uuid::parse_str(&standard_uuid)?,
            &target_user_uuid,
        )
    }

    async fn language(
        &self,
        context: &Context<'_>,
        id_lang: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Language>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_lang: Vec<i32> = id_lang.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        language::service::list::get_languages(context, id_lang, limit, offset)
    }

    async fn specs(
        &self,
        context: &Context<'_>,
        id_spec: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SpecTranslateList>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_spec: Vec<i32> = id_spec.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        spec::service::list::get_specs(context, id_spec, limit, offset)
    }

    async fn keywords(
        &self,
        context: &Context<'_>,
        id_keyword: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Keyword>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_keyword: Vec<i32> = id_keyword.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        keyword::service::list::get_keywords(context, id_keyword, limit, offset)
    }

    async fn programs(
        &self,
        context: &Context<'_>,
        id_program: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Program>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_program: Vec<i32> = id_program.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        program::service::list::get_programs(context, id_program, limit, offset)
    }

    async fn files_set_modification(
        &self,
        context: &Context<'_>,
        id_set: Option<i32>,
        // id_set: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<FileToSetModification>> {
        use component_modification_file_to_set_modification::service::list::get_files_set_modification;
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_set: i32 = id_set.unwrap_or(0);
        // let id_set: i32 = id_set.unwrap_or_(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_files_set_modification(context, id_set, limit, offset)
    }

    async fn presigned_url(
        &self,
        context: &Context<'_>,
        uuid_file: String,
    ) -> ServiceResult<String> {
        let pool = get_pool(context)?;

        // authorization check
        let target_user =
            TargetUser::from(&crate::models::user::get_auth_uuid_user(context, true)?);

        let target_uuid_file = Uuid::parse_str(&uuid_file).unwrap();

        Ok(file::service::list::get_url_file_by_uuid(target_user, target_uuid_file, pool).await?)
    }
}
