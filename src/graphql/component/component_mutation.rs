use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::component;
use crate::models::component::component_fav::model::{ComponentFav, IptComponentFavData};
use crate::models::component::keyword as component_keyword;
use crate::models::component::keyword::model::IptComponentKeywordData;
use crate::models::component::license::model::IptComponentLicenseData;
use crate::models::component::model::{IptComponentData, SlimComponent};
use crate::models::component::param as component_param;
use crate::models::component::param::model::{IptComponentParamData, DelComponentParamData};
use crate::models::component::spec as component_spec;
use crate::models::component::spec::model::IptComponentSpecData;
use crate::models::component::file as component_file;
use crate::models::component::file::model::{IptComponentFileData, DelComponentFileData};
use crate::models::component::supplier as component_supplier;
use crate::models::component::supplier::model::{
    IptSupplierComponentData, DelSupplierToComponentData
};
use crate::models::component::standard as component_standard;
use crate::models::component::standard::model::{
    IptStandardToComponentData, DelStandardToComponentData
};
use crate::models::component::component_modification;
use crate::models::component::component_modification::modification_file_from_fileset::model::{
    IptModificationFileFromFilesetData, DelModificationFileFromFilesetData,
};
use crate::models::component::component_modification::model::{
    IptComponentModificationData, SlimComponentModification, DelComponentModificationData
};
use crate::models::component::component_modification::param::model::{
    IptModificationParamData, DelModificationParamData,
};
use crate::models::component::component_modification::file::model::{
    IptModificationFileData, DelModificationFileData
};
use crate::models::component::component_modification::fileset_for_program as fileset_program;
use crate::models::component::component_modification::fileset_for_program::model::{
    IptFilesetProgramData, FilesetProgram, DelFilesetProgramData,
};
use crate::models::relate_ref::file::model::UploadFile;

use async_graphql::{self, Context, Object};
use uuid::Uuid;

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

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_component(logged_user_uuid, data, conn)
    }

    async fn delete_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<SlimComponent> {
        use component::service::delete::del_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component(
            &logged_user_uuid,
            &component_uuid,
            conn
        )
    }

    async fn put_component_params(
        &self,
        cxt: &Context<'_>,
        data: IptComponentParamData,
    ) -> ServiceResult<i32> {
        use component_param::service::change::put_component_params;

        crate::models::user::check_authorized(cxt)?;
        // let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        put_component_params(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_params(
        &self,
        cxt: &Context<'_>,
        data: DelComponentParamData,
    ) -> ServiceResult<i32> {
        use component_param::service::delete::del_component_params;

        crate::models::user::check_authorized(cxt)?;
        // let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_params(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<bool> {
        use component::license::service::add::add_component_license;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_license(data, conn)
    }

    async fn delete_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<i32> {
        use component::license::service::delete::del_component_license;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_license(data, conn)
    }

    async fn add_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecData,
    ) -> ServiceResult<i32> {
        use component_spec::service::add::add_component_specs;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_specs(data, conn)
    }

    async fn delete_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecData,
    ) -> ServiceResult<i32> {
        use component_spec::service::delete::del_component_specs;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_specs(data, conn)
    }

    async fn add_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordData,
    ) -> ServiceResult<i32> {
        use component_keyword::service::add::add_component_keywords;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_keywords(data, conn)
    }

    async fn delete_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordData,
    ) -> ServiceResult<i32> {
        use component_keyword::service::delete::del_component_keywords;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_keywords(data, conn)
    }

    async fn upload_component_files(
        &self,
        cxt: &Context<'_>,
        data: IptComponentFileData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_file::service::add::add_component_files;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

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


        crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_component_file(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_supplier_component(
        &self,
        cxt: &Context<'_>,
        data: IptSupplierComponentData,
    ) -> ServiceResult<bool> {
        use component_supplier::service::add::add_component_supplier;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_supplier(&data, conn)
    }

    async fn delete_suppliers_component(
        &self,
        cxt: &Context<'_>,
        data: DelSupplierToComponentData,
    ) -> ServiceResult<i32> {
        use component_supplier::service::delete::del_suppliers_component;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_suppliers_component(&data,conn)
    }

    async fn add_standard_to_component(
        &self,
        cxt: &Context<'_>,
        data: IptStandardToComponentData,
    ) -> ServiceResult<bool> {
        use component_standard::service::add::add_standard_to_component;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_to_component(&data,conn)
    }

    async fn delete_standards_component(
        &self,
        cxt: &Context<'_>,
        data: DelStandardToComponentData,
    ) -> ServiceResult<i32> {
        use component_standard::service::delete::del_standards_component;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standards_component(&data,conn)
    }

    async fn register_component_modification(
        &self,
        cxt: &Context<'_>,
        data: IptComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use component_modification::service::register::create_component_modification;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_component_modification(data, logged_user_uuid, conn)
    }

    async fn delete_component_modification(
        &self,
        cxt: &Context<'_>,
        data: DelComponentModificationData,
    ) -> ServiceResult<SlimComponentModification> {
        use component_modification::service::delete::del_component_modification;

        // checking authorization and getting user uuid
        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_modification(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_modification_params(
        &self,
        cxt: &Context<'_>,
        data: IptModificationParamData,
    ) -> ServiceResult<i32> {
        use component_modification::param::service::change::put_modification_params;

        crate::models::user::check_authorized(cxt)?;
        // let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        put_modification_params(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_modification_params(
        &self,
        cxt: &Context<'_>,
        data: DelModificationParamData,
    ) -> ServiceResult<i32> {
        use component_modification::param::service::delete::del_modification_params;

        crate::models::user::check_authorized(cxt)?;
        // let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_modification_params(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn upload_modification_files(
        &self,
        cxt: &Context<'_>,
        data: IptModificationFileData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_modification::file::service::add::add_modification_files;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_modification_files(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_modification_file(
        &self,
        cxt: &Context<'_>,
        data: DelModificationFileData,
    ) -> ServiceResult<bool> {
        use component_modification::file::service::delete::delete_modification_file;

        crate::models::user::check_authorized(cxt)?;
        // let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_modification_file(
            // &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn register_modification_fileset(
        &self,
        cxt: &Context<'_>,
        data: IptFilesetProgramData,
    ) -> ServiceResult<FilesetProgram> {
        use fileset_program::service::add::create_modification_fileset;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_modification_fileset(data, conn)
    }

    async fn delete_modification_fileset(
        &self,
        cxt: &Context<'_>,
        data: DelFilesetProgramData,
    ) -> ServiceResult<bool> {
        use fileset_program::service::delete::del_modification_fileset;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_modification_fileset(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn upload_files_to_fileset(
        &self,
        cxt: &Context<'_>,
        data: IptModificationFileFromFilesetData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_modification::modification_file_from_fileset::service::add::add_files_of_modification_set;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_files_of_modification_set(
            &logged_user_uuid,
            data,
            conn
        )
    }

    async fn delete_files_from_fileset(
        &self,
        cxt: &Context<'_>,
        data: DelModificationFileFromFilesetData,
    ) -> ServiceResult<bool> {
        use component_modification::modification_file_from_fileset::service::delete::del_file_from_fileset;

        let logged_user_uuid = crate::models::user::get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_file_from_fileset(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_favorite(
        &self,
        cxt: &Context<'_>,
        data: IptComponentFavData,
    ) -> ServiceResult<ComponentFav> {
        use component::component_fav::service::add::add_component_favorite;

        crate::models::user::check_authorized(cxt)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_favorite(data, conn)
    }
}
