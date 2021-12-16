use crate::database::{get_conn, PooledConnection};
use crate::errors::ServiceResult;
use crate::models::user::access::logged::get_logged_user_uuid;
use crate::models::component::{
    model::{IptComponentData, IptUpdateComponentData},
    access::model::{ChangeOwnerComponent, ChangeTypeAccessComponent},
    access::company::model::{
        IptCompanyAccessComponentData, DelCompanyAccessComponentData
    },
    access::user::model::{IptUserAccessComponentData, DelUserAccessComponentData},
    keyword as component_keyword,
    keyword::model::{IptComponentKeywordsData, IptComponentKeywordsNames},
    license::model::IptComponentLicenseData,
    param as component_param,
    param::model::{IptComponentParamData, DelComponentParamData},
    spec as component_spec,
    spec::model::IptComponentSpecsData,
    file as component_file,
    file::model::{IptComponentFileData, DelComponentFileData},
    supplier as component_supplier,
    supplier::model::DelSuppliersComponentData,
    standard as component_standard,
    standard::model::{IptStandardToComponentData, DelStandardToComponentData},
    component_modification,
    component_modification::{
        modification_file_from_fileset::model::{
            IptModificationFileFromFilesetData, DelModificationFileFromFilesetData
        },
        model::{IptComponentModificationData, IptUpdateComponentModificationData, DelComponentModificationData},
        param::model::{IptModificationParamData, DelModificationParamData},
        file::model::{IptModificationFileData, DelModificationFileData},
        fileset_for_program as fileset_program,
        fileset_for_program::model::{IptFilesetProgramData, DelFilesetProgramData},
    },
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
    ) -> ServiceResult<Uuid> {
        use crate::models::component::service::register::create_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    /// Transfer component ownership to another user
    async fn transfer_component_ownership(
        &self,
        cxt: &Context<'_>,
        data: ChangeOwnerComponent,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::manage::change_component_owner_user;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_component_owner_user(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    /// Change component type access
    async fn change_component_access(
        &self,
        cxt: &Context<'_>,
        data: ChangeTypeAccessComponent,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::manage::change_component_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        change_component_type_access(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_component_update(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        data: IptUpdateComponentData,
    ) -> ServiceResult<i32> {
        use crate::models::component::service::update::update_component_by_uuid;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_component_by_uuid(
            &logged_user_uuid,
            &component_uuid,
            &data,
            conn
        )
    }

    async fn delete_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
        use crate::models::component::service::delete::del_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component(
            &logged_user_uuid,
            &component_uuid,
            conn
        )
    }

    // Start Manage access component
    async fn set_company_access_component(
        &self,
        cxt: &Context<'_>,
        data: IptCompanyAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::company::manage::set_company_access_component;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_company_access_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_company_access_component(
        &self,
        cxt: &Context<'_>,
        data: DelCompanyAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::company::manage::del_company_access_component;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_company_access_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn set_user_access_component(
        &self,
        cxt: &Context<'_>,
        data: IptUserAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::user::manage::set_user_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        set_user_access_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_user_access_component(
        &self,
        cxt: &Context<'_>,
        data: DelUserAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::user::manage::del_user_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_user_access_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }
    // End Manage access component

    async fn put_component_params(
        &self,
        cxt: &Context<'_>,
        data: IptComponentParamData,
    ) -> ServiceResult<i32> {
        use component_param::service::change::put_component_params;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        put_component_params(
            &logged_user_uuid,
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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_params(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<bool> {
        use crate::models::component::license::service::add::add_component_license;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_license(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_license(
        &self,
        cxt: &Context<'_>,
        data: IptComponentLicenseData,
    ) -> ServiceResult<i32> {
        use crate::models::component::license::service::delete::del_component_license;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_license(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecsData,
    ) -> ServiceResult<i32> {
        use component_spec::service::add::add_component_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_specs(
        &self,
        cxt: &Context<'_>,
        data: IptComponentSpecsData,
    ) -> ServiceResult<i32> {
        use component_spec::service::delete::del_component_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_specs(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordsData,
    ) -> ServiceResult<usize> {
        use component_keyword::service::add::add_component_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_component_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_component_keywords_by_names(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordsNames,
    ) -> ServiceResult<usize> {
        use crate::models::component::keyword::service::add::add_keywords_by_names;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_keywords_by_names(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_keywords(
        &self,
        cxt: &Context<'_>,
        data: IptComponentKeywordsData,
    ) -> ServiceResult<i32> {
        use component_keyword::service::delete::del_component_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_component_keywords(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn upload_component_files(
        &self,
        cxt: &Context<'_>,
        data: IptComponentFileData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_file::service::add::add_component_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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
        arg: DelComponentFileData,
    ) -> ServiceResult<bool> {
        use component_file::service::delete::delete_component_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_component_file(
            &logged_user_uuid,
            &arg,
            conn
        )
    }

    async fn delete_suppliers_component(
        &self,
        cxt: &Context<'_>,
        data: DelSuppliersComponentData,
    ) -> ServiceResult<i32> {
        use component_supplier::service::delete::del_suppliers_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_suppliers_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn add_standard_to_component(
        &self,
        cxt: &Context<'_>,
        data: IptStandardToComponentData,
    ) -> ServiceResult<bool> {
        use component_standard::service::add::add_standard_to_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_standard_to_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_standards_component(
        &self,
        cxt: &Context<'_>,
        data: DelStandardToComponentData,
    ) -> ServiceResult<i32> {
        use component_standard::service::delete::del_standards_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_standards_component(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn register_component_modification(
        &self,
        cxt: &Context<'_>,
        data: IptComponentModificationData,
    ) -> ServiceResult<Uuid> {
        use component_modification::service::register::create_component_modification;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_component_modification(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn put_component_modification_update(
        &self,
        cxt: &Context<'_>,
        component_modification_uuid: Uuid,
        data: IptUpdateComponentModificationData,
    ) -> ServiceResult<i32> {
        use component_modification::service::update::update_modification_data;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        update_modification_data(
            &logged_user_uuid,
            &component_modification_uuid,
            &data,
            conn
        )
    }

    async fn delete_component_modification(
        &self,
        cxt: &Context<'_>,
        data: DelComponentModificationData,
    ) -> ServiceResult<Uuid> {
        use component_modification::service::delete::del_component_modification;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        put_modification_params(
            &logged_user_uuid,
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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_modification_params(
            &logged_user_uuid,
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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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
        arg: DelModificationFileData,
    ) -> ServiceResult<bool> {
        use component_modification::file::service::delete::delete_modification_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        delete_modification_file(
            &logged_user_uuid,
            &arg,
            conn
        )
    }

    async fn register_modification_fileset(
        &self,
        cxt: &Context<'_>,
        arg: IptFilesetProgramData,
    ) -> ServiceResult<Uuid> {
        use fileset_program::service::add::create_modification_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        create_modification_fileset(
            &logged_user_uuid,
            &arg,
            conn
        )
    }

    async fn delete_modification_fileset(
        &self,
        cxt: &Context<'_>,
        data: DelFilesetProgramData,
    ) -> ServiceResult<bool> {
        use fileset_program::service::delete::del_modification_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

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

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        add_files_of_modification_set(
            &logged_user_uuid,
            &data,
            conn
        )
    }

    async fn delete_files_from_fileset(
        &self,
        cxt: &Context<'_>,
        data: DelModificationFileFromFilesetData,
    ) -> ServiceResult<bool> {
        use component_modification::modification_file_from_fileset::service::delete::del_file_from_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &PooledConnection = &get_conn(cxt)?;

        del_file_from_fileset(
            &logged_user_uuid,
            &data,
            conn
        )
    }
}
