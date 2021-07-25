// use crate::cli_args::Opt;
use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::company::company_represent::model::ShowCompanyRepresent;
use crate::models::company::company_represent::service as company_represent;
use crate::models::company::model::ShowCompany;
use crate::models::company::service as company;
use crate::models::component::component_modification::model::ShowComponentModification;
use crate::models::component::component_modification::service as component_modification;
use crate::models::component::license as component_license;
use crate::models::component::license::model::{License, LicenseToComponent};
use crate::models::component::model::ShowComponent;
use crate::models::component::param as component_param;
use crate::models::component::param::model::{Param, ParamToModel};
use crate::models::component::service as component;
use crate::models::user::model::{SlimUser, ShowUser};
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::service as user;
// use crate::models::file::model::{ShowFile, FileData, SlimFile};
use crate::models::file::model::ShowFile;
use crate::models::file::service as file;
use crate::models::standard::model::ShowStandard;
use crate::models::standard::service as standard;
use async_graphql::Context;
// use crate::database::PooledConnection;
// use diesel::PgConnection;

// use std::sync::Arc;
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
        let uuid_user_create = match uuid {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        user::list::get_users(context, uuid_user_create, limit, offset)
    }

    async fn myself( &self, context: &Context<'_>) -> ServiceResult<SlimUser> {
        let token_data = user::token::token_from_context(context)?;

        let token_data = user::token::decode(&token_data)?;

        user::token::get_slim_user(token_data)
    }

    async fn update_token( &self, context: &Context<'_>) -> ServiceResult<Token> {
        user::token::update(context)
    }

    async fn decode_token( &self, context: &Context<'_>) -> ServiceResult<Claims> {
        let token = user::token::token_from_context(context)?;
        user::token::decode(&token)
    }

    async fn logout( &self ) -> ServiceResult<String> {
        // todo!(deacticate user token)
        // user::token::deactive_all(context, target_uuid_user)
        Ok("Good Luck".to_owned())
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

        let uuid_user = crate::models::user::get_uuid_user(context)?;

        notification::list::get_notifications(context, id_notification, uuid_user, limit, offset)
    }

    async fn files(
        &self,
        context: &Context<'_>,
        uuid_user: Option<String>,
        uuid_component: Option<String>,
        uuid_component_modification: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowFile>> {
        let uuid_user_create = match uuid_user {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };
        let uuid_component = match uuid_component {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };
        let uuid_component_modification = match uuid_component_modification {
            None => Uuid::nil(),
            Some(uuid) => Uuid::parse_str(&uuid)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        file::list::get_files(
            context,
            uuid_user_create,
            uuid_component,
            uuid_component_modification,
            limit,
            offset,
        )
    }

    async fn components(
        &self,
        context: &Context<'_>,
        uuid_component: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowComponent>> {
        let uuid_component = match uuid_component {
            None => Uuid::nil(),
            Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component::list::get_components(context, uuid_component, limit, offset)
    }

    async fn component_modification(
        &self,
        context: &Context<'_>,
        uuid_component: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowComponentModification>> {
        let uuid_component = match uuid_component {
            None => Uuid::nil(),
            Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_modification::list::get_component_modifications(
            context,
            uuid_component,
            limit,
            offset,
        )
    }

    async fn licenses(
        &self,
        context: &Context<'_>,
        id_license: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<License>> {
        let id_license: Vec<i32> = id_license.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_license::service::list::get_licenses(context, id_license, limit, offset)
    }

    async fn license_component(
        &self,
        context: &Context<'_>,
        id_license: Option<i32>,
        uuid_component: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<LicenseToComponent>> {
        let id_license: i32 = id_license.unwrap_or(0);
        let uuid_component = match uuid_component {
            None => Uuid::nil(),
            Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_license::service::list_component::get_licenses_component(
            context,
            id_license,
            uuid_component,
            limit,
            offset,
        )
    }

    async fn param(
        &self,
        context: &Context<'_>,
        id_param: Vec<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Param>> {
        let id_param: Vec<i32> = id_param;
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_param::service::list::get_params(context, id_param, limit, offset)
    }

    async fn param_component(
        &self,
        context: &Context<'_>,
        id_param: Option<i32>,
        uuid_component: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamToModel>> {
        let id_param: i32 = id_param.unwrap_or(0);
        let uuid_component = match uuid_component {
            None => Uuid::nil(),
            Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_param::service::list_component::get_params_component(
            context,
            id_param,
            uuid_component,
            limit,
            offset,
        )
    }

    async fn param_modification(
        &self,
        context: &Context<'_>,
        id_param: Option<i32>,
        uuid_modification: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamToModel>> {
        let id_param: i32 = id_param.unwrap_or(0);
        let uuid_modification = match uuid_modification {
            None => Uuid::nil(),
            Some(uuid_modification) => Uuid::parse_str(&uuid_modification)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_param::service::list_modification::get_params_modification(
            context,
            id_param,
            uuid_modification,
            limit,
            offset,
        )
    }

    async fn companies(
        &self,
        context: &Context<'_>,
        uuid_company: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowCompany>> {
        let uuid_company = match uuid_company {
            None => Uuid::nil(),
            Some(uuid_company) => Uuid::parse_str(&uuid_company)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        company::list::get_companies(context, uuid_company, limit, offset)
    }

    async fn company_represents(
        &self,
        context: &Context<'_>,
        uuid_company: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowCompanyRepresent>> {
        let uuid_company = match uuid_company {
            None => Uuid::nil(),
            Some(uuid_company) => Uuid::parse_str(&uuid_company)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        company_represent::list::get_company_represents(context, uuid_company, limit, offset)
    }

    async fn standards(
        &self,
        context: &Context<'_>,
        uuid_standard: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowStandard>> {
        let uuid_standard = match uuid_standard {
            None => Uuid::nil(),
            Some(uuid_standard) => Uuid::parse_str(&uuid_standard)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        standard::list::get_standards(context, uuid_standard, limit, offset)
    }
}
