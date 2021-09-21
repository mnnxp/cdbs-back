use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use async_graphql::{self, Context, Object};

use crate::models::component;
use crate::models::component::component_fav::model::{ComponentFav, IptComponentFavData};
use crate::models::component::component_modification::file_to_set_modification as component_modification_file_to_set_modification;
use crate::models::component::component_modification::file_to_set_modification::model::{
    FileToSetModification, IptFileToSetModificationData,
};
use crate::models::component::component_modification::model::{
    IptComponentModificationData, SlimComponentModification,
};
use crate::models::component::component_modification::param as component_modification_param;
use crate::models::component::component_modification::param::model::{
    IptParamModificationData, ParamModification,
};
use crate::models::component::component_modification::set_of_files_program as component_modification_set_of_files_program;
use crate::models::component::component_modification::set_of_files_program::model::{
    IptSetOfFilesProgramData, SetOfFilesProgram,
};
use crate::models::component::keyword as component_keyword;
use crate::models::component::keyword::model::IptComponentKeywordData;
use crate::models::component::license::model::IptComponentLicenseData;
use crate::models::component::model::{IptComponentData, SlimComponent};
use crate::models::component::param as component_param;
use crate::models::component::param::model::{IptParamComponentData, ParamComponent};
use crate::models::component::spec as component_spec;
use crate::models::component::spec::model::IptComponentSpecData;
use crate::models::component::file as component_file;
use crate::models::component::file::model::{IptComponentFileData, DelComponentFileData};
use crate::models::component::supplier as component_supplier;
use crate::models::component::supplier::model::{IptSupplierComponentData, SupplierComponent};
use crate::models::relate_ref::file::model::UploadFile;

#[derive(Default)]
pub struct ComponentMutation;

#[Object]
impl ComponentMutation {
    async fn register_component(
        &self,
        cxt: &Context<'_>,
        data: IptComponentData,
    ) -> ServiceResult<SlimComponent> {
        use component::service::register::create_component;
        let conn: &PooledConnection = &get_conn(cxt)?;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(create_component(logged_user_uuid, data, conn)?)
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

    async fn add_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<bool> {
        use component::license::service::add::add_component_license;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_license(data, conn)?)
    }

    async fn delete_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<i32> {
        use component::license::service::delete::del_component_license;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(del_component_license(data, conn)?)
    }

    async fn add_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecData,
    ) -> ServiceResult<i32> {
        use component_spec::service::add::add_component_specs;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_specs(data, conn)?)
    }

    async fn delete_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecData,
    ) -> ServiceResult<i32> {
        use component_spec::service::delete::del_component_specs;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(del_component_specs(data, conn)?)
    }

    async fn add_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordData,
    ) -> ServiceResult<i32> {
        use component_keyword::service::add::add_component_keywords;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(add_component_keywords(data, conn)?)
    }

    async fn delete_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordData,
    ) -> ServiceResult<i32> {
        use component_keyword::service::delete::del_component_keywords;
        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::check_authorized(cxt)?;

        Ok(del_component_keywords(data, conn)?)
    }

    async fn upload_component_files(
        &self,
        cxt: &Context<'_>,
        data: IptComponentFileData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_file::service::add::add_component_files;

        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        add_component_files(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_file(
        &self,
        cxt: &Context<'_>,
        data: DelComponentFileData,
    ) -> ServiceResult<bool> {
        use component_file::service::delete::delete_component_file;

        let conn: &PooledConnection = &get_conn(cxt)?;

        crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(delete_component_file(
            // &logged_user_uuid,
            &data,
            conn
        )?)
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

    async fn register_component_modification(
        &self,
        cxt: &Context<'_>,
        data: IptComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use component::component_modification::service::register::create_component_modification;
        let conn: &PooledConnection = &get_conn(cxt)?;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        Ok(create_component_modification(data, logged_user_uuid, conn)?)
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
}
