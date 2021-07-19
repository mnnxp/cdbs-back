use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::jwt::model::{DecodedToken, Token};
use crate::models::user::model::{LoggedUser, ShowUser, SlimUser, UserData};
use crate::models::user::service as user;
use crate::models::user::service::token::ClaimsResponse;
// use crate::models::file::model::{ShowFile, FileData, SlimFile};
use crate::models::company::company_represent::model::{
    CompanyRepresentData, ShowCompanyRepresent, SlimCompanyRepresent,
};
use crate::models::company::company_represent::service as company_represent;
use crate::models::company::model::{CompanyData, ShowCompany, SlimCompany};
use crate::models::company::service as company;
use crate::models::component::component_modification::model::{
    ComponentModificationData, ShowComponentModification, SlimComponentModification,
};
use crate::models::component::component_modification::service as component_modification;
use crate::models::component::license as component_license;
use crate::models::component::license::model::{
    License, LicenseData, LicenseToComponent, LicenseToComponentData,
};
use crate::models::component::model::{
    ComponentData, ComponentDataQuery, ShowComponent, SlimComponent,
};
use crate::models::component::param as component_param;
use crate::models::component::param::model::{Param, ParamData, ParamToModel, ParamToModelData};
use crate::models::component::service as component;
use crate::models::file::model::ShowFile;
use crate::models::file::service as file;
use crate::models::standard::model::{ShowStandard, SlimStandard, StandardData};
use crate::models::standard::service as standard;
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

        // crate::models::user::has_supplier(&context.user, 1)?;

        user::list::get_users(&context, uuid_user_create, limit, offset)
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

        file::list::get_files(
            &context,
            uuid_user_create,
            uuid_component,
            uuid_component_modification,
            limit,
            offset,
        )
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

        component::list::get_components(&context, uuid_component, limit, offset)
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

        component_modification::list::get_component_modifications(
            &context,
            uuid_component,
            limit,
            offset,
        )
    }

    pub fn licenses(
        context: &Context,
        id_license: Option<Vec<i32>>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<License>> {
        let id_license: Vec<i32> = id_license.unwrap_or_default();
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        component_license::service::list::get_licenses(&context, id_license, limit, offset)
    }

    pub fn license_component(
        context: &Context,
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
            &context,
            id_license,
            uuid_component,
            limit,
            offset,
        )
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

        component_param::service::list::get_params(&context, id_param, limit, offset)
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

        component_param::service::list_component::get_params_component(
            &context,
            id_param,
            uuid_component,
            limit,
            offset,
        )
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

        component_param::service::list_modification::get_params_modification(
            &context,
            id_param,
            uuid_modification,
            limit,
            offset,
        )
    }

    pub fn companies(
        context: &Context,
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

        company::list::get_companies(&context, uuid_company, limit, offset)
    }

    pub fn company_represents(
        context: &Context,
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

        company_represent::list::get_company_represents(&context, uuid_company, limit, offset)
    }

    pub fn standards(
        context: &Context,
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

        standard::list::get_standards(&context, uuid_standard, limit, offset)
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

    // todo!(receive files via MultipartField or MultipartData)
    // pub fn register_files(context: &Context, data: MultipartField) -> ServiceResult<SlimFile> {
    //     use crate::models::file::service::register::create_file;
    //     let conn: &PgConnection = &context.db;
    //
    //     // crate::models::user::verify_uuid_user(&context.user, data.uuid_user_create)?;
    //     crate::models::user::hash_authorized(&context.user)?;
    //
    //     Ok(create_file(data, conn)?)
    // }

    pub fn register_component(
        context: &Context,
        data: ComponentDataQuery,
    ) -> ServiceResult<SlimComponent> {
        use crate::models::component::service::register::create_component;
        let conn: &PgConnection = &context.db;

        // crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;
        crate::models::user::hash_authorized(&context.user)?;

        // if data.is_standard != 0 {
        //     crate::models::user::has_supplier(&context.user, 1)?;
        // }

        let user_uuid = context.user.0.as_ref().unwrap().uuid;
        let uuid_component_parent = Uuid::parse_str(&data.uuid_component_parent)?;
        let commentchange = String::new();

        let component_data = ComponentData {
            uuid_component_parent: (uuid_component_parent),
            name: (data.name),
            description: (data.description),
            uuid_user: (user_uuid),
            id_type_access: (data.id_type_access),
            id_component_type: (data.id_component_type),
            id_actual_status: (data.id_actual_status),
            is_standard: (data.is_standard),
        };

        Ok(create_component(component_data, conn)?)
    }

    pub fn register_component_modification(
        context: &Context,
        data: ComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use crate::models::component::component_modification::service::register::create_component_modification;
        let conn: &PgConnection = &context.db;

        let uuid_user = context.user.as_ref().clone().unwrap().uuid;
        // let component_parent_uuid = data.uuid_component;

        Ok(create_component_modification(data, uuid_user, conn)?)
    }

    pub fn register_license(context: &Context, data: LicenseData) -> ServiceResult<License> {
        use crate::models::component::license::service::register::create_license;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_license(data, conn)?)
    }

    pub fn register_license_component(
        context: &Context,
        data: LicenseToComponentData,
    ) -> ServiceResult<LicenseToComponent> {
        use crate::models::component::license::service::add_to_component::create_license_component;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_license_component(data, conn)?)
    }

    pub fn register_param(context: &Context, data: ParamData) -> ServiceResult<Param> {
        use crate::models::component::param::service::register::create_param;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param(data, conn)?)
    }

    pub fn register_param_component(
        context: &Context,
        data: ParamToModelData,
    ) -> ServiceResult<ParamToModel> {
        use crate::models::component::param::service::add_to_component::create_param_component;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param_component(data, conn)?)
    }

    pub fn register_param_modification(
        context: &Context,
        data: ParamToModelData,
    ) -> ServiceResult<ParamToModel> {
        use crate::models::component::param::service::add_to_modification::create_param_modification;
        let conn: &PgConnection = &context.db;

        crate::models::user::hash_authorized(&context.user)?;

        Ok(create_param_modification(data, conn)?)
    }

    pub fn register_company(context: &Context, data: CompanyData) -> ServiceResult<SlimCompany> {
        use crate::models::company::service::register::create_company;
        let conn: &PgConnection = &context.db;

        // crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;
        crate::models::user::hash_authorized(&context.user)?;

        // if data.is_standard != 0 {
        //     crate::models::user::has_supplier(&context.user, 1)?;
        // }

        let user_uuid = context.user.0.as_ref().unwrap().uuid;

        let company_data = CompanyData {
            orgname: (data.orgname),
            shortname: (data.shortname),
            inn: (data.inn),
            phone: (data.phone),
            email: (data.email),
            description: (data.description),
            address: (data.address),
            site_url: (data.site_url),
            time_zone: (data.time_zone),
            uuid_user: (user_uuid),
            uuid_image_file: (data.uuid_image_file),
            id_region: (data.id_region),
            id_type_org: (data.id_type_org),
        };

        Ok(create_company(company_data, conn)?)
    }

    pub fn register_company_represent(
        context: &Context,
        data: CompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;
        let conn: &PgConnection = &context.db;

        crate::models::company::util::check_is_supplier(data.uuid_company, conn)?;
        crate::models::company::util::check_company_access(
            &context.user, data.uuid_company, 3, conn,
        )?;

        Ok(create_company_represent(data, conn)?)
    }

    pub fn delete_company_represent(
        context: &Context,
        uuid_company: Uuid,
        uuid_company_represent: Uuid,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PgConnection = &context.db;

        crate::models::company::util::check_company_access(
            &context.user, uuid_company, 3, conn,
        )?;

        Ok(delete_company_represent(uuid_company, uuid_company_represent, conn)?)
    }

    pub fn register_standard(context: &Context, data: StandardData) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::register::create_standard;
        let conn: &PgConnection = &context.db;

        // crate::models::user::verify_uuid_user(&context.user, data.uuid_user)?;
        crate::models::user::hash_authorized(&context.user)?;

        // if data.is_standard != 0 {
        //     crate::models::user::has_supplier(&context.user, 1)?;
        // }

        let user_uuid = context.user.0.as_ref().unwrap().uuid;

        let standard_data = StandardData {
            uuid_standard_parent: (data.uuid_standard_parent),
            classifier: (data.classifier),
            name: (data.name),
            description: (data.description),
            specified_tolerance: (data.specified_tolerance),
            technical_committee: (data.technical_committee),
            publication_at: (data.publication_at),
            uuid_image_file: (data.uuid_image_file),
            uuid_user: (user_uuid),
            uuid_company: (data.uuid_company),
            id_type_access: (data.id_type_access),
            id_standard_status: (data.id_standard_status),
            id_region: (data.id_region),
        };

        Ok(create_standard(standard_data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
