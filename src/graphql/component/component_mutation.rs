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
    param::model::{IptComponentParamsData, DelComponentParamData},
    spec as component_spec,
    spec::model::IptComponentSpecsData,
    file as component_file,
    file::model::{IptComponentFilesData, IptComponentFaviconData, DelComponentFileData},
    supplier as component_supplier,
    supplier::model::DelSuppliersComponentData,
    standard as component_standard,
    standard::model::{IptStandardToComponentData, DelStandardToComponentData},
    component_modification,
    component_modification::{
        model::{IptComponentModificationData, IptUpdateComponentModificationData, DelComponentModificationData},
        fileset_for_program::file::model::{
            IptModificationFileFromFilesetData, DelModificationFileFromFilesetData
        },
        param::model::{IptModificationParamData, DelModificationParamData},
        file::model::{IptModificationFilesData, DelModificationFileData},
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
    /// Creates a component, returns the UUID of the new component.
    async fn register_component(
        &self,
        cxt: &Context<'_>,
        args: IptComponentData,
    ) -> ServiceResult<Uuid> {
        use crate::models::component::service::register::create_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Transfers ownership of a component to another user.
    async fn transfer_component_ownership(
        &self,
        cxt: &Context<'_>,
        args: ChangeOwnerComponent,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::manage::change_component_owner_user;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_component_owner_user(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Changes the default access to a component.
    async fn change_component_access(
        &self,
        cxt: &Context<'_>,
        args: ChangeTypeAccessComponent,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::manage::change_component_type_access;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        change_component_type_access(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates the component's underlying data by UUID.
    /// Returns the number of successful changes or an error if all the specified data already exists.
    async fn put_component_update(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
        args: IptUpdateComponentData,
    ) -> ServiceResult<usize> {
        use crate::models::component::service::update::update_component_by_uuid;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_component_by_uuid(
            &logged_user_uuid,
            &component_uuid,
            &args,
            conn
        )
    }

    /// Deletes a component and its associated data.
    /// Returns the UUID of the removed component.
    async fn delete_component(
        &self,
        cxt: &Context<'_>,
        component_uuid: Uuid,
    ) -> ServiceResult<Uuid> {
        use crate::models::component::service::delete::del_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component(
            &logged_user_uuid,
            &component_uuid,
            conn
        )
    }

    /// Sets access to a component for a company.
    /// This access applies to all members of the company according to their roles.
    async fn set_company_access_component(
        &self,
        cxt: &Context<'_>,
        args: IptCompanyAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::company::manage::set_company_access_component;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_company_access_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes access to a component for a company.
    async fn delete_company_access_component(
        &self,
        cxt: &Context<'_>,
        args: DelCompanyAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::company::manage::del_company_access_component;

        // checking authorization and getting company uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_company_access_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Sets access to a component for a user.
    async fn set_user_access_component(
        &self,
        cxt: &Context<'_>,
        args: IptUserAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::user::manage::set_user_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        set_user_access_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes access to a component for a user.
    async fn delete_user_access_component(
        &self,
        cxt: &Context<'_>,
        args: DelUserAccessComponentData,
    ) -> ServiceResult<bool> {
        use crate::models::component::access::user::manage::del_user_access_component;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_user_access_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds new parameters with values ​​for a component.
    /// Updates the values ​​of existing component parameters if the provided parameter names already exist.
    async fn put_component_params(
        &self,
        cxt: &Context<'_>,
        args: IptComponentParamsData,
    ) -> ServiceResult<usize> {
        use component_param::service::change::put_component_params;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        put_component_params(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes component parameters.
    /// Returns the number of successfully removed parameters.
    async fn delete_component_params(
        &self,
        cxt: &Context<'_>,
        args: DelComponentParamData,
    ) -> ServiceResult<usize> {
        use component_param::service::delete::del_component_params;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component_params(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds a license to a component.
    async fn add_component_license(
        &self,
        cxt: &Context<'_>,
        args: IptComponentLicenseData,
    ) -> ServiceResult<bool> {
        use crate::models::component::license::service::add::add_component_license;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_license(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes a license for a component.
    async fn delete_component_license(
        &self,
        cxt: &Context<'_>,
        args: IptComponentLicenseData,
    ) -> ServiceResult<usize> {
        use crate::models::component::license::service::delete::del_component_license;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component_license(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds a component connection to directory sections.
    async fn add_component_specs(
        &self,
        cxt: &Context<'_>,
        args: IptComponentSpecsData,
    ) -> ServiceResult<i32> {
        use component_spec::service::add::add_component_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes a component's association with directory partitions.
    async fn delete_component_specs(
        &self,
        cxt: &Context<'_>,
        args: IptComponentSpecsData,
    ) -> ServiceResult<usize> {
        use component_spec::service::delete::del_component_specs;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component_specs(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds keywords to a component by IDs.
    async fn add_component_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptComponentKeywordsData,
    ) -> ServiceResult<usize> {
        use component_keyword::service::add::add_component_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds keywords to a component by words.
    async fn add_component_keywords_by_names(
        &self,
        cxt: &Context<'_>,
        args: IptComponentKeywordsNames,
    ) -> ServiceResult<usize> {
        use crate::models::component::keyword::service::add::add_keywords_by_names;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_keywords_by_names(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes keywords from a component.
    async fn delete_component_keywords(
        &self,
        cxt: &Context<'_>,
        args: IptComponentKeywordsData,
    ) -> ServiceResult<usize> {
        use component_keyword::service::delete::del_component_keywords;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component_keywords(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates preliminary files information for a component.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_component_files(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_file::service::add::add_component_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_files(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates the main image of the component.
    /// Returns a structure with a pre-signed URL for uploading an image file.
    async fn upload_component_favicon(
        &self,
        cxt: &Context<'_>,
        args: IptComponentFaviconData,
    ) -> ServiceResult<UploadFile> {
        use component_file::service::add::add_component_favicon;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_component_favicon(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Deletes a file of a component.
    async fn delete_component_file(
        &self,
        cxt: &Context<'_>,
        args: DelComponentFileData,
    ) -> ServiceResult<bool> {
        use component_file::service::delete::delete_component_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_component_file(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes suppliers a component by UUIDs.
    async fn delete_suppliers_component(
        &self,
        cxt: &Context<'_>,
        args: DelSuppliersComponentData,
    ) -> ServiceResult<usize> {
        use component_supplier::service::delete::del_suppliers_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_suppliers_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Attaches a standard to a component.
    async fn add_standard_to_component(
        &self,
        cxt: &Context<'_>,
        args: IptStandardToComponentData,
    ) -> ServiceResult<bool> {
        use component_standard::service::add::add_standard_to_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_standard_to_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Unpins a standard from a component.
    async fn delete_standards_component(
        &self,
        cxt: &Context<'_>,
        args: DelStandardToComponentData,
    ) -> ServiceResult<usize> {
        use component_standard::service::delete::del_standards_component;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_standards_component(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates a new modification for a component.
    async fn register_component_modification(
        &self,
        cxt: &Context<'_>,
        args: IptComponentModificationData,
    ) -> ServiceResult<Uuid> {
        use component_modification::service::register::create_component_modification;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_component_modification(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Updates modification's data of a component.
    async fn put_component_modification_update(
        &self,
        cxt: &Context<'_>,
        component_modification_uuid: Uuid,
        args: IptUpdateComponentModificationData,
    ) -> ServiceResult<usize> {
        use component_modification::service::update::update_modification_data;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        update_modification_data(
            &logged_user_uuid,
            &component_modification_uuid,
            &args,
            conn
        )
    }

    /// Removes a component modification.
    async fn delete_component_modification(
        &self,
        cxt: &Context<'_>,
        args: DelComponentModificationData,
    ) -> ServiceResult<Uuid> {
        use component_modification::service::delete::del_component_modification;

        // checking authorization and getting user uuid
        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_component_modification(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Adds new parameters with values ​​for a component modification.
    /// Updates the values ​​of existing component modification parameters if the provided parameter names already exist.
    async fn put_modification_params(
        &self,
        cxt: &Context<'_>,
        args: IptModificationParamData,
    ) -> ServiceResult<usize> {
        use component_modification::param::service::change::put_modification_params;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        put_modification_params(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Deletes parameters of a component modification.
    async fn delete_modification_params(
        &self,
        cxt: &Context<'_>,
        args: DelModificationParamData,
    ) -> ServiceResult<usize> {
        use component_modification::param::service::delete::del_modification_params;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_modification_params(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates preliminary files information for a component modification.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_modification_files(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFilesData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_modification::file::service::add::add_modification_files;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_modification_files(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Deletes a file of a component modification.
    async fn delete_modification_file(
        &self,
        cxt: &Context<'_>,
        args: DelModificationFileData,
    ) -> ServiceResult<bool> {
        use component_modification::file::service::delete::delete_modification_file;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        delete_modification_file(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates a set of files for component modification.
    /// The files necessary for the operation of a specific soft (CAD system) are loaded into file sets for this soft.
    /// Also file sets are used to configure integrations with various CAD/CAM systems etc.
    async fn register_modification_fileset(
        &self,
        cxt: &Context<'_>,
        args: IptFilesetProgramData,
    ) -> ServiceResult<Uuid> {
        use fileset_program::service::add::create_modification_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        create_modification_fileset(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes a set of files from a component modification.
    async fn delete_modification_fileset(
        &self,
        cxt: &Context<'_>,
        args: DelFilesetProgramData,
    ) -> ServiceResult<bool> {
        use fileset_program::service::delete::del_modification_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_modification_fileset(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Creates preliminary files information for a set of files from a component modification.
    /// Returns structures with a pre-signed URL for uploading a files.
    async fn upload_files_to_fileset(
        &self,
        cxt: &Context<'_>,
        args: IptModificationFileFromFilesetData,
    ) -> ServiceResult<Vec<UploadFile>> {
        use component_modification::fileset_for_program::file::service::add::add_files_of_modification_set;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        add_files_of_modification_set(
            &logged_user_uuid,
            &args,
            conn
        )
    }

    /// Removes files from the component modification fileset.
    async fn delete_files_from_fileset(
        &self,
        cxt: &Context<'_>,
        args: DelModificationFileFromFilesetData,
    ) -> ServiceResult<bool> {
        use component_modification::fileset_for_program::file::service::delete::del_file_from_fileset;

        let logged_user_uuid = get_logged_user_uuid(cxt, true)?;

        let conn: &mut PooledConnection = &mut get_conn(cxt)?;

        del_file_from_fileset(
            &logged_user_uuid,
            &args,
            conn
        )
    }
}
