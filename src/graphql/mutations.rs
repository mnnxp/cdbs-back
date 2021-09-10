use crate::errors::ServiceResult;
use crate::database::{get_conn, get_pool, PooledConnection};
use crate::models::company::company_represent::model::{IptCompanyRepresentData, SlimCompanyRepresent};
use crate::models::company::model::{SlimCompany, IptCompanyData,};
use crate::models::user::model::{SlimUser, IptUserData, TargetUser};
use crate::models::user::notification::model::{Notification, NotificationData, SlimNotification};
use crate::models::component::component_modification::model::{SlimComponentModification, IptComponentModificationData};
use crate::models::component::license::model::{LicenseComponent, IptLicenseComponentData};
use crate::models::component::model::{SlimComponent, IptComponentData};
use crate::models::component::param::model::{ParamComponent, IptParamComponentData};
use crate::models::component::param as component_param;
use crate::models::component::component_fav::model::{ComponentFav, IptComponentFavData};
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
use crate::models::standard::model::{SlimStandard, IptStandardData};
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
use crate::models::relate_ref::file::model::IptPreliminaryFileData;
use crate::models::relate_ref::file as file;
use crate::storage::backblaze::b2_types::UploadUrl;
use async_graphql::Context;
use uuid::Uuid;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn register_user(
        &self,
        cxt: &Context<'_>,
        data: IptUserData,
    ) -> ServiceResult<SlimUser> {
        use crate::models::user::service::register::create_user;
        let conn: &PooledConnection = &get_conn(cxt)?;

        Ok(create_user(data, conn)?)
    }

    async fn register_notification(
        &self,
        cxt: &Context<'_>,
        data: NotificationData,
    ) -> ServiceResult<SlimNotification> {
        use crate::models::user::notification::service::register::create_notification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_notification(data, logged_uuid_user, conn)?)
    }

    async fn delete_notification(
        &self,
        cxt: &Context<'_>,
        id_notification: i32,
    ) -> ServiceResult<Notification> {
        use crate::models::user::notification::service::delete::delete_notification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(delete_notification(
            logged_uuid_user,
            id_notification,
            conn,
        )?)
    }

    async fn register_component(
        &self,
        cxt: &Context<'_>,
        data: IptComponentData,
    ) -> ServiceResult<SlimComponent> {
        use component::service::register::create_component;
        let conn: &PooledConnection = &get_conn(cxt)?;

        // checking authorization and getting user uuid
        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_component(
            logged_uuid_user,
            data,
            conn
        )?)
    }

    async fn register_component_modification(
        &self,
        cxt: &Context<'_>,
        data: IptComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use component::component_modification::service::register::create_component_modification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_component_modification(data, logged_uuid_user, conn)?)
    }

    async fn register_license(
        &self,
        cxt: &Context<'_>,
        data: LicenseData,
    ) -> ServiceResult<License> {
        use license::service::register::create_license;
        let conn: &PooledConnection = &get_conn(cxt)?;

        // todo!(check owned company)
        crate::models::user::check_authorized(cxt)?;

        Ok(create_license(data, conn)?)
    }

    async fn register_license_component(
        &self,
        cxt: &Context<'_>,
        data: IptLicenseComponentData,
    ) -> ServiceResult<LicenseComponent> {
        use component::license::service::add::create_license_component;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_license_component(data, conn)?)
    }

    async fn register_param(
        &self,
        cxt: &Context<'_>,
        data: IptParamTranslateListData,
    ) -> ServiceResult<ParamTranslateList> {
        use param::service::register::create_param;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_param(data, conn)?)
    }

    async fn register_param_component(
        &self,
        cxt: &Context<'_>,
        data: IptParamComponentData,
    ) -> ServiceResult<ParamComponent> {
        use component_param::service::add::create_param_component;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_param_component(data, conn)?)
    }

    async fn register_param_modification(
        &self,
        cxt: &Context<'_>,
        data: IptParamModificationData,
    ) -> ServiceResult<ParamModification> {
        use component_modification_param::service::add::create_param_modification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_param_modification(data, conn)?)
    }

    async fn register_company(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyData,
    ) -> ServiceResult<SlimCompany> {
        use crate::models::company::service::register::create_company;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_company(
            logged_uuid_user,
            data,
            conn
        )?)
    }

    async fn register_company_represent(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyRepresentData,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::register::create_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        let target_uuid_company = Uuid::parse_str(&data.uuid_company.to_string())?;

        crate::models::company::util::check_company_access(
            &logged_uuid_user,
            &target_uuid_company,
            3,
            conn,
        )?;

        crate::models::company::util::check_is_supplier(
            &target_uuid_company,
            conn
        )?;

        Ok(create_company_represent(data.into(), conn)?)
    }

    async fn delete_company_represent(
        &self,
        cxt: &Context<'_>,
        uuid_company: String,
        uuid_company_represent: String,
    ) -> ServiceResult<SlimCompanyRepresent> {
        use crate::models::company::company_represent::service::delete::delete_company_represent;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        let target_uuid_company = Uuid::parse_str(&uuid_company)?;
        let target_uuid_company_represent = Uuid::parse_str(&uuid_company_represent)?;

        crate::models::company::util::check_company_access(
            &logged_uuid_user,
            &target_uuid_company,
            3,
            conn,
        )?;

        Ok(delete_company_represent(
            target_uuid_company,
            target_uuid_company_represent,
            conn
        )?)
    }

    async fn register_standard(
        &self,
        cxt: &Context<'_>,
        data: IptStandardData,
    ) -> ServiceResult<SlimStandard> {
        use crate::models::standard::service::register::create_standard;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_uuid_user = crate::models::user::get_logged_uuid_user(cxt, true)?;

        Ok(create_standard(
            logged_uuid_user,
            data,
            conn
        )?)
    }

    async fn add_component_spec(
        &self,
        cxt: &Context<'_>,
        data: IptSpecComponentData,
    ) -> ServiceResult<SpecComponent> {
        use component_spec::service::add::add_component_spec;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_spec(data, conn)?)
    }

    async fn register_keyword(
        &self,
        cxt: &Context<'_>,
        data: IptKeywordData,
    ) -> ServiceResult<Keyword> {
        use keyword::service::register::create_keyword;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_keyword(data, conn)?)
    }

    async fn add_keyword_component(
        &self,
        cxt: &Context<'_>,
        data: IptKeywordComponentData,
    ) -> ServiceResult<KeywordComponent> {
        use component_keyword::service::add::add_component_keyword;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_keyword(data, conn)?)
    }

    async fn add_supplier_component(
        &self,
        cxt: &Context<'_>,
        data: IptSupplierComponentData,
    ) -> ServiceResult<SupplierComponent> {
        use component_supplier::service::add::add_component_supplier;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_supplier(data, conn)?)
    }

    async fn register_extension(
        &self,
        cxt: &Context<'_>,
        data: IptExtensionData,
    ) -> ServiceResult<Extension> {
        use extension::service::register::create_extension;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_extension(data, conn)?)
    }

    async fn register_program(
        &self,
        cxt: &Context<'_>,
        data: IptProgramData,
    ) -> ServiceResult<Program> {
        use program::service::register::create_program;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_program(data, conn)?)
    }

    async fn register_set_files_modification(
        &self,
        cxt: &Context<'_>,
        data: IptSetOfFilesProgramData,
    ) -> ServiceResult<SetOfFilesProgram> {
        use component_modification_set_of_files_program::service::add::create_set_file_modification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(create_set_file_modification(data, conn)?)
    }

    async fn add_file_to_set_modification(
        &self,
        cxt: &Context<'_>,
        data: IptFileToSetModificationData,
    ) -> ServiceResult<FileToSetModification> {
        use component_modification_file_to_set_modification::service::add::add_file_to_set_modification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_file_to_set_modification(data, conn)?)
    }


    async fn add_component_favorite(
        &self,
        cxt: &Context<'_>,
        data: IptComponentFavData,
    ) -> ServiceResult<ComponentFav> {
        use component::component_fav::service::add::add_component_favorite;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_favorite(data, conn)?)
    }

    async fn upload_favicon(
        &self,
        cxt: &Context<'_>,
        file_data: IptPreliminaryFileData,
    ) -> ServiceResult<UploadUrl> {
        use crate::models::user::service::upload::favicon::update_favicon;
        let pool = get_pool(cxt)?;

        let target_user = TargetUser::from(
            &crate::models::user::get_logged_uuid_user(cxt, true)?
        );

        Ok(update_favicon(
            target_user,
            file_data,
            pool
        ).await?)
    }

    async fn upload_completed(
        &self,
        cxt: &Context<'_>,
        file_id: String,
    ) -> ServiceResult<i32> {
        let pool = get_pool(cxt)?;
        // let conn = pool.get().unwrap();

        let target_user = TargetUser::from(
            &crate::models::user::get_logged_uuid_user(cxt, true)?
        );

        Ok(file::service::update::confirm_upload(
            &target_user,
            &file_id,
            pool
        ).await?)
    }
}
