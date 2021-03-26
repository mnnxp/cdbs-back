use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::jwt::model::{DecodedToken, Token};
use crate::models::user::model::{LoggedUser, User, UserData, SlimUser};
use crate::models::user::service as user;
use crate::models::user::service::token::ClaimsResponse;
use crate::models::user_represet::model::{
    UserRepreset,
    UserRepresetData,
    SlimUserRepreset
};
use crate::models::user_represet::service as user_represet;
use crate::models::file::model::{SlimFile, File, FileData};
use crate::models::file::service as file;
use crate::models::component::model::{
    Component,
    ComponentData,
    SlimComponent
};
use crate::models::component::service as component;
use crate::models::component_modification::model::{
    ComponentModification,
    ComponentModificationData,
    SlimComponentModification
};
use crate::models::component_modification::service as component_modification;
use diesel::PgConnection;
use juniper::Context as JuniperContext;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct Context {
    pub opt: Opt,
    pub db: Arc<PooledConnection>,
    pub user: LoggedUser,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(token: DecodedToken, user: LoggedUser, pool: PooledConnection, opt: Opt) -> Self {
        Self {
            opt,
            token,
            user,
            db: Arc::new(pool),
        }
    }
}

pub(crate) struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<User>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        //crate::user::has_role(&context.user, 'user')?;

        user::list::find_all_users(&context, limit, offset)
    }

    pub fn user_represet(
        context: &Context,
        uuid_user_search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<UserRepreset>> {
        let uuid_user_search = match uuid_user_search {
            None => Uuid::nil(),
            Some(uuid_user_search) => Uuid::parse_str(&uuid_user_search)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        user_represet::list::show(&context, uuid_user_search, limit, offset)
    }

    pub fn generate_token(context: &Context) -> ServiceResult<Token> {
        user::token::generate(&context)
    }

    pub fn decode_token(context: &Context) -> ServiceResult<&ClaimsResponse> {
        user::token::decode(&context)
    }

    pub fn files(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<File>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        file::list::find_all_files(&context, limit, offset)
    }

    pub fn components(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Component>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component::list::find_all_components(&context, limit, offset)
    }

    pub fn component_modification(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ComponentModification>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_modification::list::find_all_component_modification(&context, limit, offset)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register_user(context: &Context, data: UserData) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &PgConnection = &context.db;

        Ok(create_user(data, conn)?)
    }

    pub fn register_file(context: &Context, data: FileData) -> ServiceResult<SlimFile> {
        use crate::models::file::service::register::create_file;
        let conn: &PgConnection = &context.db;

        Ok(create_file(data, conn)?)
    }

    pub fn register_component(context: &Context, data: ComponentData) -> ServiceResult<SlimComponent> {
        use crate::models::component::service::register::create_component;
        let conn: &PgConnection = &context.db;

        Ok(create_component(data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
