// use crate::cli_args::Opt;
use crate::errors::ServiceResult;
use crate::jwt::model::{Claims, Token};
use crate::models::company::company_represent::model::ShowCompanyRepresent;
use crate::models::company::company_represent::service as company_represent;
use crate::models::company::model::ShowCompany;
use crate::models::company::service as company;
use crate::models::component::component_modification::model::ComponentModification;
use crate::models::component::component_modification::service as component_modification;
// use crate::models::component::license as component_license;
// use crate::models::component::license::model::LicenseComponent;
use crate::models::component::model::Component;
use crate::models::component::model::ComponentAndRelatedData;
// use crate::models::component::param::model::ParamComponent;
// use crate::models::component::param as component_param;
// use crate::models::component::component_fav::model::ComponentFav;
// use crate::models::component::component_fav as component_fav;
use crate::models::component::component_modification::param::model::ParamModification;
use crate::models::component::component_modification::param as component_modification_param;
use crate::models::component::component_modification::set_of_files_program::model::SetOfFilesProgram;
use crate::models::component::component_modification::set_of_files_program as component_modification_set_of_files_program;
use crate::models::component::component_modification::file_to_set_modification::model::FileToSetModification;
use crate::models::component::component_modification::file_to_set_modification as component_modification_file_to_set_modification;
// use crate::models::component::spec as component_spec;
// use crate::models::component::keyword as component_keyword;
use crate::models::component::supplier::model::SupplierComponent;
use crate::models::component::supplier as component_supplier;
use crate::models::component::service as component;
use crate::models::user::model::{SlimUser, ShowUser};
use crate::models::user::service::token::model::UserToken;
use crate::models::user::notification::model::Notification;
use crate::models::user::notification::service as notification;
use crate::models::user::service as user;
use crate::models::standard::model::ShowStandard;
use crate::models::standard as standard;
// use crate::models::relate_ref::file::model::{ShowFile, FileData, SlimFile};
use crate::models::relate_ref::extension::model::Extension;
use crate::models::relate_ref::extension as extension;
use crate::models::relate_ref::license::model::License;
use crate::models::relate_ref::license as license;
use crate::models::relate_ref::param::model::ParamTranslateList;
use crate::models::relate_ref::param as param;
use crate::models::relate_ref::file::model::ShowFile;
use crate::models::relate_ref::file::service as file;
use crate::models::relate_ref::keyword::model::Keyword;
use crate::models::relate_ref::keyword as keyword;
use crate::models::relate_ref::language::model::Language;
use crate::models::relate_ref::language as language;
use crate::models::relate_ref::program::model::Program;
use crate::models::relate_ref::program as program;
use crate::models::relate_ref::spec::model::SpecTranslateList;
use crate::models::relate_ref::spec as spec;
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

    async fn show_tokens(
        &self,
        context: &Context<'_>
    ) -> ServiceResult<Vec<UserToken>> {
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
        user::token::show_tokens(
            context,
            auth_uuid_user,
        )
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

    async fn delete_token(
        &self,
        context: &Context<'_>,
        token: String,
    ) -> ServiceResult<String> {
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
        let deactivated_tokens = format!(
            "removed {} token.",
            // deactivate all user token
            user::token::delete_user_token(
                context,
                token.as_str(),
                auth_uuid_user,
            )?
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

    async fn logout(&self, context: &Context<'_> ) -> ServiceResult<String> {
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
        uuid_components: Option<Vec<String>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Component>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let target_uuid_components = match uuid_components {
            Some(vec_uuid) => {
                vec_uuid.into_iter()
                    .map(|x| Uuid::parse_str(x.as_str()).unwrap())
                    .collect()
            },
            None => Vec::new(),
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component::list::find_components(context, target_uuid_components, limit, offset)
    }

    async fn component(
        &self,
        context: &Context<'_>,
        uuid_component: String,
    ) -> ServiceResult<ComponentAndRelatedData> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        component::list::find_uuid_component(context, Uuid::parse_str(&uuid_component)?)
    }

    async fn component_modification(
        &self,
        context: &Context<'_>,
        uuid_component: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ComponentModification>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

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
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_license: Vec<i32> = id_license.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        license::service::list::get_licenses(context, id_license, limit, offset)
    }

    // async fn license_component(
    //     &self,
    //     context: &Context<'_>,
    //     id_license: Option<i32>,
    //     uuid_component: Option<String>,
    //     limit: Option<i32>,
    //     offset: Option<i32>,
    // ) -> ServiceResult<Vec<LicenseComponent>> {
    //     // authorization check
    //     crate::models::user::util::check_authorized(context)?;
    //
    //     let id_license: i32 = id_license.unwrap_or(0);
    //     let uuid_component = match uuid_component {
    //         None => Uuid::nil(),
    //         Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
    //     };
    //     let limit: i32 = limit.unwrap_or(100);
    //     let offset: i32 = offset.unwrap_or(0);
    //
    //     component_license::service::list::get_licenses_component(
    //         context,
    //         id_license,
    //         uuid_component,
    //         limit,
    //         offset,
    //     )
    // }

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

    // async fn param_component(
    //     &self,
    //     context: &Context<'_>,
    //     id_param: Option<i32>,
    //     uuid_component: Option<String>,
    //     limit: Option<i32>,
    //     offset: Option<i32>,
    // ) -> ServiceResult<Vec<ParamComponent>> {
    //     // authorization check
    //     crate::models::user::util::check_authorized(context)?;
    //
    //     let id_param: i32 = id_param.unwrap_or(0);
    //     let uuid_component = match uuid_component {
    //         None => Uuid::nil(),
    //         Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
    //     };
    //     let limit: i32 = limit.unwrap_or(100);
    //     let offset: i32 = offset.unwrap_or(0);
    //
    //     component_param::service::list::get_params_component(
    //         context,
    //         id_param,
    //         uuid_component,
    //         limit,
    //         offset,
    //     )
    // }

    async fn param_modification(
        &self,
        context: &Context<'_>,
        id_param: Option<i32>,
        uuid_modification: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ParamModification>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_param: i32 = id_param.unwrap_or(0);
        let uuid_modification = match uuid_modification {
            None => Uuid::nil(),
            Some(uuid_modification) => Uuid::parse_str(&uuid_modification)?,
        };
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_modification_param::service::list::get_params_modification(
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
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let uuid_company = match uuid_company {
            None => Uuid::nil(),
            Some(uuid_company) => Uuid::parse_str(&uuid_company)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        company::list::get_companies(
            context,
            uuid_company,
            limit,
            offset
        )
    }

    async fn company_represents(
        &self,
        context: &Context<'_>,
        uuid_company: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowCompanyRepresent>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let uuid_company = match uuid_company {
            None => Uuid::nil(),
            Some(uuid_company) => Uuid::parse_str(&uuid_company)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        company_represent::list::get_company_represents(
            context,
            uuid_company,
            limit,
            offset
        )
    }

    async fn standards(
        &self,
        context: &Context<'_>,
        uuid_standard: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowStandard>> {
        // authorization check
        let auth_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        let target_uuid_standard = match uuid_standard {
            //if no standard is specified, get all standards the user has access
            None => Uuid::nil(),
            // if the standard was specified, you need to check the access right
            Some(uuid_standard) => {
                let target_uuid_standard = Uuid::parse_str(&uuid_standard)?;

                // access check for user
                if standard::util::get_default_access_standard(
                    context,
                    target_uuid_standard,
                )? < 3 {
                    debug!("start access check for user");
                    standard::util::check_standard_access(
                        context,
                        crate::models::user::get_auth_uuid_user(context, false)?,
                        target_uuid_standard,
                        2
                    )?;
                }
                target_uuid_standard
            },
        };

        standard::service::list::get_standards(
            context,
            auth_uuid_user,
            target_uuid_standard,
            limit,
            offset
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

    // async fn component_specs(
    //     &self,
    //     context: &Context<'_>,
    //     uuid_component: Option<String>,
    //     limit: Option<i32>,
    //     offset: Option<i32>,
    // ) -> ServiceResult<Vec<SpecTranslateList>> {
    //     // authorization check
    //     crate::models::user::util::check_authorized(context)?;
    //
    //     let uuid_component = match uuid_component {
    //         None => Uuid::nil(),
    //         Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
    //     };
    //     let limit: i32 = limit.unwrap_or(100);
    //     let offset: i32 = offset.unwrap_or(0);
    //
    //     component_spec::service::list::find_all_component_specs(
    //         context,
    //         uuid_component,
    //         limit,
    //         offset,
    //     )
    // }

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

    // async fn component_keywords(
    //     &self,
    //     context: &Context<'_>,
    //     uuid_component: Option<String>,
    //     limit: Option<i32>,
    //     offset: Option<i32>,
    // ) -> ServiceResult<Vec<Keyword>> {
    //     // authorization check
    //     crate::models::user::util::check_authorized(context)?;
    //
    //     let uuid_component = match uuid_component {
    //         None => Uuid::nil(),
    //         Some(uuid_component) => Uuid::parse_str(&uuid_component)?,
    //     };
    //     let limit: i32 = limit.unwrap_or(100);
    //     let offset: i32 = offset.unwrap_or(0);
    //
    //     component_keyword::service::list::find_all_component_keywords(
    //         context,
    //         uuid_component,
    //         limit,
    //         offset,
    //     )
    // }

    async fn suppliers(
        &self,
        context: &Context<'_>,
        uuid_component: String,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SupplierComponent>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let uuid_component: Uuid = Uuid::parse_str(&uuid_component.to_string()).unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_supplier::service::list::find_all_component_suppliers(context, uuid_component, limit, offset)
    }

    async fn extensions(
        &self,
        context: &Context<'_>,
        id_extension: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Extension>> {
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let id_extension: Vec<i32> = id_extension.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        extension::service::list::get_extensions(context, id_extension, limit, offset)
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

    async fn set_of_files_program(
        &self,
        context: &Context<'_>,
        uuid_modification: String,
        id_program: Option<i32>,
        // id_set: Option<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<SetOfFilesProgram>> {
        use component_modification_set_of_files_program::service::list::get_set_files_modification;
        // authorization check
        crate::models::user::util::check_authorized(context)?;

        let uuid_modification = Uuid::parse_str(&uuid_modification.to_string()).unwrap_or_default();
        let id_program: i32 = id_program.unwrap_or(0);
        // let id_set: i32 = id_set.unwrap_or_(0);
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        get_set_files_modification(context, uuid_modification, id_program, limit, offset)
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

    // async fn component_fav(
    //     &self,
    //     context: &Context<'_>,
    //     uuid_component: String,
    //     limit: Option<i32>,
    //     offset: Option<i32>,
    // ) -> ServiceResult<Vec<ComponentFav>> {
    //     use component_fav::service::list::find_all_component_favorites;
    //
    //     // authorization check
    //     crate::models::user::util::check_authorized(context)?;
    //
    //     let uuid_component: Uuid = Uuid::parse_str(&uuid_component.to_string()).expect("Uuid bad or not found.");
    //     let limit: i32 = limit.unwrap_or(100);
    //     let offset: i32 = offset.unwrap_or(0);
    //
    //     find_all_component_favorites(context, uuid_component, limit, offset)
    // }
}
