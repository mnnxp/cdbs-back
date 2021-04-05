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

        crate::models::user::has_supplier(&context.user, 1)?;

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
        uuid_user_create_search: Option<String>,
        uuid_component_search: Option<String>,
        uuid_component_modification_search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<File>> {
        let uuid_user_create_search = match uuid_user_create_search {
            None => Uuid::nil(),
            Some(uuid_user_create_search) => Uuid::parse_str(&uuid_user_create_search)?,
        };
        let uuid_component_search = match uuid_component_search {
            None => Uuid::nil(),
            Some(uuid_component_search) => Uuid::parse_str(&uuid_component_search)?,
        };
        let uuid_component_modification_search = match uuid_component_modification_search {
            None => Uuid::nil(),
            Some(uuid_component_modification_search) => Uuid::parse_str(&uuid_component_modification_search)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        file::list::show(&context, uuid_user_create_search, uuid_component_search,
            uuid_component_modification_search, limit, offset)
    }

    pub fn components(
        context: &Context,
        uuid_component_search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Component>> {
        let uuid_component_search = match uuid_component_search {
            None => Uuid::nil(),
            Some(uuid_component_search) => Uuid::parse_str(&uuid_component_search)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component::list::show(&context, uuid_component_search, limit, offset)
    }

    pub fn component_modification(
        context: &Context,
        uuid_component_search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ComponentModification>> {
        let uuid_component_search = match uuid_component_search {
            None => Uuid::nil(),
            Some(uuid_component_search) => Uuid::parse_str(&uuid_component_search)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_modification::list::show(&context, uuid_component_search, limit, offset)
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

    pub fn register_user_represet(
        context: &Context, data: UserRepresetData
    ) -> ServiceResult<SlimUserRepreset> {
        use crate::models::user_represet::service::register::create_user_represet;
        let conn: &PgConnection = &context.db;

        crate::models::user::has_supplier(&context.user, 1)?;
        crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;

        Ok(create_user_represet(data, conn)?)
    }

    pub fn register_file(context: &Context, data: FileData) -> ServiceResult<SlimFile> {
        use crate::models::file::service::register::create_file;
        let conn: &PgConnection = &context.db;

        crate::models::user::verify_uuid_user(&context.user, data.uuid_user_create)?;

        Ok(create_file(data, conn)?)
    }

    pub fn register_component(context: &Context, data: ComponentData) -> ServiceResult<SlimComponent> {
        use crate::models::component::service::register::create_component;
        let conn: &PgConnection = &context.db;

        crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;

        if data.is_standard != 0 {
            crate::models::user::has_supplier(&context.user, 1)?;
        }

        Ok(create_component(data, conn)?)
    }

    pub fn register_component_modification(context: &Context, data: ComponentModificationData) -> ServiceResult<SlimComponentModification> {
        use crate::models::component_modification::service::register::create_component_modification;
        let conn: &PgConnection = &context.db;

        let uuid_user = context.user.as_ref().clone().unwrap().uuid;
        let component_parent_uuid = data.uuid_component;

        Ok(create_component_modification(data, uuid_user, component_parent_uuid, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
