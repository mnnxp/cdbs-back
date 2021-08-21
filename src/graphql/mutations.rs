use crate::errors::ServiceResult;
use crate::database::{get_conn, PooledConnection};
use crate::models::company::company_represent::model::{IptCompanyRepresentData, SlimCompanyRepresent};
use crate::models::company::model::{SlimCompany, CompanyData, IptCompanyData,};
use crate::models::user::model::{SlimUser, IptUserData};
use crate::models::user::notification::model::{Notification, NotificationData, SlimNotification};
use crate::models::component::component_modification::model::{SlimComponentModification, IptComponentModificationData};
use crate::models::component::license::model::{LicenseComponent, IptLicenseComponentData};
use crate::models::component::model::{SlimComponent, ComponentData, IptComponentData};
use crate::models::component::param::model::{ParamComponent, IptParamComponentData};
use crate::models::component::param as component_param;
use crate::models::component::component_modification::param::model::{ParamModification, IptParamModificationData};
use crate::models::component::component_modification::param as component_modification_param;
use crate::models::component::component_modification::set_of_files_program::model::{SetOfFilesProgram, IptSetOfFilesProgramData};
use crate::models::component::component_modification::set_of_files_program as component_modification_set_of_files_program;
use crate::models::component::component_modification::file_to_set_modification::model::{FileToSetModification, IptFileToSetModificationData};
use crate::models::component::component_modification::file_to_set_modification as component_modification_file_to_set_modification;
use crate::models::component as component;
use crate::models::component::spec::model::{SpecComponent, IptSpecComponentData};
use crate::models::component::spec as component_spec;
use crate::models::component::keyword::model::{KeywordComponent, IptKeywordComponentData};
use crate::models::component::keyword as component_keyword;
use crate::models::component::supplier::model::{SupplierComponent, IptSupplierComponentData};
use crate::models::component::supplier as component_supplier;
use crate::models::standard::model::{SlimStandard, IptStandardData, StandardData};
use crate::models::relate_ref::extension::model::{Extension, IptExtensionData};
use crate::models::relate_ref::extension as extension;
use crate::models::relate_ref::license::model::{License, LicenseData};
use crate::models::relate_ref::license as license;
use crate::models::relate_ref::param::model::{ParamTranslateList, IptParamTranslateListData};
use crate::models::relate_ref::param as param;
use crate::models::relate_ref::program::model::{Program, IptProgramData};
use crate::models::relate_ref::program as program;
use crate::models::relate_ref::keyword::model::{Keyword, IptKeywordData};
use crate::models::relate_ref::keyword as keyword;
// use crate::models::relate_ref::file::model::{ShowFile, SlimFile};
use async_graphql::Context;
// use async_graphql::{
//     dataloader::DataLoader, Context, EmptySubscription, FieldResult, Schema,
// };

use uuid::Uuid;

// use diesel::PgConnection;
// use diesel::pg::PgConnection;

// use crate::database::Pool;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn register_user(
        &self,
        context: &Context<'_>,
        data: IptUserData,
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &PooledConnection = &get_conn(context)?;

        Ok(create_user(data.into(), conn)?)
    }

    async fn register_notification(
        &self,
        context: &Context<'_>,
        data: NotificationData,
    ) -> ServiceResult<SlimNotification> {
        use crate::models::user::notification::service::register::create_notification;
        let conn: &PooledConnection = &get_conn(context)?;

        let uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        Ok(create_notification(data, uuid_user, conn)?)
    }

    async fn delete_notification(
        &self,
        context: &Context<'_>,
        id_notification: i32,
    ) -> ServiceResult<Notification> {
        use crate::models::user::notification::service::delete::delete_notification;
        let conn: &PooledConnection = &get_conn(context)?;

        let uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        Ok(delete_notification(
            uuid_user,
            id_notification,
            conn,
        )?)
    }

    async fn register_component(
        &self,
        context: &Context<'_>,
        data: IptComponentData,
    ) -> ServiceResult<SlimComponent> {
        use component::service::register::create_component;
        let conn: &PooledConnection = &get_conn(context)?;

        // checking authorization and getting user uuid
        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let uuid_component_parent = Uuid::parse_str(&data.uuid_component_parent)?;

        let component_data = ComponentData {
            uuid_component_parent: (uuid_component_parent),
            name: (data.name),
            description: (data.description),
            uuid_user: (logged_uuid_user),
            id_type_access: (data.id_type_access),
            id_component_type: (data.id_component_type),
            id_actual_status: (data.id_actual_status),
            is_standard: (data.is_standard),
        };

        Ok(create_component(component_data, conn)?)
    }

    async fn register_component_modification(
        &self,
        context: &Context<'_>,
        data: IptComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use component::component_modification::service::register::create_component_modification;
        let conn: &PooledConnection = &get_conn(context)?;

        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        Ok(create_component_modification(data, logged_uuid_user, conn)?)
    }

    async fn register_license(
        &self,
        context: &Context<'_>,
        data: LicenseData,
    ) -> ServiceResult<License> {
        use license::service::register::create_license;
        let conn: &PooledConnection = &get_conn(context)?;

        // todo!(check owned company)
        crate::models::user::check_authorized(context)?;

        Ok(create_license(data, conn)?)
    }

    async fn register_license_component(
        &self,
        context: &Context<'_>,
        data: IptLicenseComponentData,
    ) -> ServiceResult<LicenseComponent> {
        use component::license::service::add::create_license_component;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_license_component(data, conn)?)
    }

    async fn register_param(
        &self,
        context: &Context<'_>,
        data: IptParamTranslateListData,
    ) -> ServiceResult<ParamTranslateList> {
        use param::service::register::create_param;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_param(data, conn)?)
    }

    async fn register_param_component(
        &self,
        context: &Context<'_>,
        data: IptParamComponentData,
    ) -> ServiceResult<ParamComponent> {
        use component_param::service::add::create_param_component;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_param_component(data, conn)?)
    }

    async fn register_param_modification(
        &self,
        context: &Context<'_>,
        data: IptParamModificationData,
    ) -> ServiceResult<ParamModification> {
        use component_modification_param::service::add::create_param_modification;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_param_modification(data, conn)?)
    }

    async fn register_company(
        &self,
        context: &Context<'_>,
        data: IptCompanyData,
    ) -> ServiceResult<SlimCompany> {
        use crate::models::company::service::register::create_company;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let target_uuid_image_file = Uuid::parse_str(&data.uuid_image_file)?;

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
            uuid_user: (logged_uuid_user),
            uuid_image_file: (target_uuid_image_file),
            id_region: (data.id_region),
            id_type_org: (data.id_type_org),
        };

        Ok(create_company(company_data, conn)?)
    }

    async fn register_company_represent(
        &self,
        context: &Context<'_>,
        data: IptCompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;
        let conn: &PooledConnection = &get_conn(context)?;

        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let target_uuid_company = Uuid::parse_str(&data.uuid_company.to_string())?;

        crate::models::company::util::check_company_access(
            logged_uuid_user,
            target_uuid_company,
            3,
            conn,
        )?;

        crate::models::company::util::check_is_supplier(
            target_uuid_company,
            conn
        )?;

        Ok(create_company_represent(data.into(), conn)?)
    }

    async fn delete_company_represent(
        &self,
        context: &Context<'_>,
        uuid_company: String,
        uuid_company_represent: String,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PooledConnection = &get_conn(context)?;

        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let target_uuid_company = Uuid::parse_str(&uuid_company)?;
        let target_uuid_company_represent = Uuid::parse_str(&uuid_company_represent)?;

        crate::models::company::util::check_company_access(
            logged_uuid_user, target_uuid_company, 3, conn,
        )?;

        Ok(delete_company_represent(
            target_uuid_company,
            target_uuid_company_represent,
            conn
        )?)
    }

    async fn register_standard(
        &self,
        context: &Context<'_>,
        data: IptStandardData,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::register::create_standard;
        let conn: &PooledConnection = &get_conn(context)?;

        let logged_uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;

        let target_uuid_standard_parent = Uuid::parse_str(&data.uuid_standard_parent.to_string())?;
        let target_uuid_image_file = Uuid::parse_str(&data.uuid_image_file.to_string())?;
        let target_uuid_company = Uuid::parse_str(&data.uuid_company.to_string())?;

        crate::models::company::util::check_company_access(
            logged_uuid_user, target_uuid_company, 3, conn,
        )?;

        crate::models::company::util::check_is_supplier(target_uuid_company, conn)?;

        let standard_data = StandardData {
            uuid_standard_parent: (target_uuid_standard_parent),
            classifier: (data.classifier),
            name: (data.name),
            description: (data.description),
            specified_tolerance: (data.specified_tolerance),
            technical_committee: (data.technical_committee),
            publication_at: (data.publication_at),
            uuid_image_file: (target_uuid_image_file),
            uuid_user: (logged_uuid_user),
            uuid_company: (target_uuid_company),
            id_type_access: (data.id_type_access),
            id_standard_status: (data.id_standard_status),
            id_region: (data.id_region),
        };

        Ok(create_standard(standard_data, conn)?)
    }

    async fn add_component_spec(
        &self,
        context: &Context<'_>,
        data: IptSpecComponentData,
    ) -> ServiceResult<SpecComponent> {
        use component_spec::service::add::add_component_spec;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(add_component_spec(data, conn)?)
    }

    async fn register_keyword(
        &self,
        context: &Context<'_>,
        data: IptKeywordData,
    ) -> ServiceResult<Keyword> {
        use keyword::service::register::create_keyword;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_keyword(data, conn)?)
    }

    async fn add_keyword_component(
        &self,
        context: &Context<'_>,
        data: IptKeywordComponentData,
    ) -> ServiceResult<KeywordComponent> {
        use component_keyword::service::add::add_component_keyword;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(add_component_keyword(data, conn)?)
    }

    async fn add_supplier_component(
        &self,
        context: &Context<'_>,
        data: IptSupplierComponentData,
    ) -> ServiceResult<SupplierComponent> {
        use component_supplier::service::add::add_component_supplier;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(add_component_supplier(data, conn)?)
    }

    async fn register_extension(
        &self,
        context: &Context<'_>,
        data: IptExtensionData,
    ) -> ServiceResult<Extension> {
        use extension::service::register::create_extension;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_extension(data, conn)?)
    }

    async fn register_program(
        &self,
        context: &Context<'_>,
        data: IptProgramData,
    ) -> ServiceResult<Program> {
        use program::service::register::create_program;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_program(data, conn)?)
    }

    async fn register_set_files_modification(
        &self,
        context: &Context<'_>,
        data: IptSetOfFilesProgramData,
    ) -> ServiceResult<SetOfFilesProgram> {
        use component_modification_set_of_files_program::service::add::create_set_file_modification;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(create_set_file_modification(data, conn)?)
    }

    async fn add_file_to_set_modification(
        &self,
        context: &Context<'_>,
        data: IptFileToSetModificationData,
    ) -> ServiceResult<FileToSetModification> {
        use component_modification_file_to_set_modification::service::add::add_file_to_set_modification;
        let conn: &PooledConnection = &get_conn(context)?;

        crate::models::user::check_authorized(context)?;

        Ok(add_file_to_set_modification(data, conn)?)
    }

    // Upload images for profile picture
    // todo!(receive files via MultipartField or MultipartData)
    // pub async fn upload_favicon(
    //     &self,
    //     context: &Context<'_>,
    //     // payload: Multipart,
    // ) -> ServiceResult<ShowFile> {
    //     let conn: &PooledConnection = &get_conn(context)?;
    //
    //     let uuid_user = crate::models::user::get_auth_uuid_user(context, true)?;
    //
    //     // TODO: add search for parent file by name in table file_ref
    //     let uuid_file_parent = Uuid::parse_str("3706d1a1-80ae-4367-be39-af7091373811")?;
    //
    //     // let respond_slim_file = file::register(
    //     //     payload, user_uuid, addiction_table,
    //     //     uuid_addiction, uuid_file_parent, conn
    //     // ).await?;
    //
    //     let upload_favicon_file = crate::models::user::upload::favicon(
    //         context
    //     ).await?;
    //
    //     Ok(upload_favicon_file)
    // }
}
