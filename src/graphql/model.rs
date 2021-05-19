use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::jwt::model::{DecodedToken, Token};
use crate::models::user::model::{LoggedUser, ShowUser, UserData, SlimUser};
use crate::models::user::service as user;
use crate::models::user::service::token::ClaimsResponse;
use crate::models::user_represent::model::{
    ShowUserRepresent, UserRepresentData, SlimUserRepresent
};
use crate::models::user_represent::service as user_represent;
// use crate::models::file::model::{ShowFile, FileData, SlimFile};
use crate::models::file::model::ShowFile;
use crate::models::file::service as file;
use crate::models::component::model::{
    ShowComponent, ComponentData, ComponentDataQuery, SlimComponent
};
use crate::models::component::service as component;
use crate::models::component_modification::model::{
    ShowComponentModification, ComponentModificationData, SlimComponentModification
};
use crate::models::component_modification::service as component_modification;
use crate::models::param::model::{Param, ParamData, ParamToModel, ParamToModelData};
use crate::models::param::service as param;
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
    ) -> ServiceResult<Vec<ShowUser>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        crate::models::user::has_supplier(&context.user, 1)?;

        user::list::find_all_users(&context, limit, offset)
    }

    pub fn user_represent(
        context: &Context,
        uuid_user: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<ShowUserRepresent>> {
        let uuid_user = match uuid_user {
            None => Uuid::nil(),
            Some(uuid_user) => Uuid::parse_str(&uuid_user)?,
        };

        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        user_represent::list::show(&context, uuid_user, limit, offset)
    }

    pub fn generate_token(context: &Context) -> ServiceResult<Token> {
        user::token::generate(&context)
    }

    pub fn decode_token(context: &Context) -> ServiceResult<&ClaimsResponse> {
        user::token::decode(&context)
    }

    pub fn files(
        context: &Context,
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

        file::list::show(&context, uuid_user_create, uuid_component,
            uuid_component_modification, limit, offset)
    }

    pub fn components(
        context: &Context,
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

        component::list::show(&context, uuid_component, limit, offset)
    }

    pub fn component_modification(
        context: &Context,
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

        component_modification::list::show(&context, uuid_component, limit, offset)
    }

    pub fn param(
        context: &Context,
        id_param: Vec<i32>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Param>> {
        let id_param: Vec<i32> = id_param;
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        param::list::show(&context, id_param, limit, offset)
    }

    pub fn param_component(
        context: &Context,
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

        param::list_component::show_component(&context, id_param, uuid_component, limit, offset)
    }

    pub fn param_modification(
        context: &Context,
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

        param::list_modification::show_modification(&context, id_param, uuid_modification, limit, offset)
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

    pub fn register_user_represent(
        context: &Context, data: UserRepresentData
    ) -> ServiceResult<SlimUserRepresent> {
        use crate::models::user_represent::service::register::create_user_represent;
        let conn: &PgConnection = &context.db;

        crate::models::user::has_supplier(&context.user, 1)?;
        crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;

        Ok(create_user_represent(data, conn)?)
    }

    // pub fn register_file(context: &Context, data: FileData) -> ServiceResult<SlimFile> {
    //     use crate::models::file::service::register::create_file;
    //     let conn: &PgConnection = &context.db;
    //
    //     // crate::models::user::verify_uuid_user(&context.user, data.uuid_user_create)?;
    //     crate::models::user::hash_authorized(&context.user)?;
    //
    //     Ok(create_file(data, conn)?)
    // }

    pub fn register_component(context: &Context, data: ComponentDataQuery) -> ServiceResult<SlimComponent> {
        use crate::models::component::service::register::create_component;
        let conn: &PgConnection = &context.db;

        // crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;
        crate::models::user::hash_authorized(&context.user)?;

        if data.is_standard != 0 {
            crate::models::user::has_supplier(&context.user, 1)?;
        }

        let user_uuid = context.user.0.as_ref().unwrap().uuid;
        let uuid_component_parent = Uuid::parse_str(&data.uuid_component_parent)?;
        let commentchange = String::new();

        let component_data = ComponentData {
            is_standard: (data.is_standard),
            commentchange: (commentchange),
            id_type_access: (data.id_type_access),
            is_delete: (0),
            id_component_type: (data.id_component_type),
            id_actual_status: (data.id_actual_status),
            uuid_component_parent: (uuid_component_parent),
            comment: (data.comment),
            uuid_user: (user_uuid),
            name: (data.name),
        };

        Ok(create_component(component_data, conn)?)
    }

    pub fn register_component_modification(context: &Context, data: ComponentModificationData) -> ServiceResult<SlimComponentModification> {
        use crate::models::component_modification::service::register::create_component_modification;
        let conn: &PgConnection = &context.db;

        let uuid_user = context.user.as_ref().clone().unwrap().uuid;
        // let component_parent_uuid = data.uuid_component;

        Ok(create_component_modification(data, uuid_user, conn)?)
    }

    pub fn register_param(context: &Context, data: ParamData) -> ServiceResult<Param> {
        use crate::models::param::service::register::create_param;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param(data, conn)?)
    }

    pub fn register_param_component(context: &Context, data: ParamToModelData) -> ServiceResult<ParamToModel> {
        use crate::models::param::service::add_to_component::create_param_component;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param_component(data, conn)?)
    }

    pub fn register_param_modification(context: &Context, data: ParamToModelData) -> ServiceResult<ParamToModel> {
        use crate::models::param::service::add_to_modification::create_param_modification;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param_modification(data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
