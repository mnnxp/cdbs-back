use crate::database::{get_conn, PooledConnection};
use crate::graphql::file::ShowFileRelatedData;
use crate::graphql::relate::attributes::{IptPaginate, IptSort};
use crate::models::company::model::ShowCompanyShort;
use crate::models::relate_ref::{
    file::model::DownloadFile, keyword::model::Keyword, language::get_set_language,
    region::model::RegionTranslateList, spec::model::SpecTranslateList,
};
use crate::models::search::order::{Paginate, Sort, TableName};
use crate::models::supplier_service::{
    model::{ServiceFilesArg, ServicesArg},
    param::model::ServiceParamWithTranslation,
    service_status::model::ServiceStatusTranslateList,
};
use crate::models::user::model::ShowUserShort;
use async_graphql::{Context, InputObject, Object};
use chrono::NaiveDateTime;
use uuid::Uuid;

/// Complete information about service (part) and related data.
/// Default sorting: `createdAt`. Sorting by `name`, `description`, `serviceStatusId`, `updatedAt` is available.
#[derive(Debug)]
pub(crate) struct ServiceAndRelatedData {
    /// UUID of the service on the platform
    pub(crate) uuid: Uuid,
    /// Service name
    pub(crate) name: String,
    /// Service description
    pub(crate) description: String,
    /// Data about the profile that uploaded the service
    pub(crate) owner_user: ShowUserShort,
    /// Data about the company that owns the service
    pub(crate) owner_company: ShowCompanyShort,
    /// Current status of the service (e.g., "in development")
    pub(crate) service_status: ServiceStatusTranslateList,
    /// Main region of application of the service
    pub(crate) region: RegionTranslateList,
    /// Date the service card was created
    pub(crate) created_at: NaiveDateTime,
    /// Date the service's master data was updated
    pub(crate) updated_at: NaiveDateTime,
}

#[Object]
impl ServiceAndRelatedData {
    /// Identifier of the service on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Service name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Service description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Data about the profile owning the service
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Data about the company owning the service
    async fn owner_company(&self) -> &ShowCompanyShort {
        &self.owner_company
    }

    /// Current status of the service (e.g. "in development")
    async fn service_status(&self) -> &ServiceStatusTranslateList {
        &self.service_status
    }

    /// Main region of application of the service
    async fn region(&self) -> &RegionTranslateList {
        &self.region
    }

    /// Date when the service profile was created
    async fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    /// Date of updating the service's master data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    // Связанные с услугой данные

    /// Data on service parameters (list).
    /// Default sorting: `paramId`. Sorting by `paramname` and `value` is available.
    async fn service_params(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ServiceParamWithTranslation> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::ParamTranslateList, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::ParamTranslateList));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ServiceParamWithTranslation::by_service_uuid(
            &self.uuid,
            &get_set_language(cxt),
            &s,
            &p,
            conn,
        )
        .expect("Error loading service parameters")
    }

    /// Returns the total number of params in the service (without filters)
    async fn params_count(&self, cxt: &Context<'_>) -> i64 {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        Paginate::get_count(&self.uuid, &TableName::ParamToService, conn)
            .expect("Error count items")
    }

    /// Files associated with the service. Default sorting: `createdAt`.
    /// Sorting by `revision`, `filename`, `size`, `updatedAt` is available.
    async fn files(
        &self,
        cxt: &Context<'_>,
        sort: Option<IptSort>,
        paginate: Option<IptPaginate>,
    ) -> Vec<ShowFileRelatedData> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let s = sort
            .map(|s| Sort::parsing(TableName::FileRef, &s.by_field, s.as_desc))
            .unwrap_or(Sort::set_by_table(TableName::FileRef));
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        ShowFileRelatedData::by_service_uuid(&self.uuid, &s, &p, conn)
            .expect("Error loading service files")
    }

    /// Returns the total number of files in the service (without filters)
    async fn files_count(&self, cxt: &Context<'_>) -> i64 {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        Paginate::get_count(&self.uuid, &TableName::FileToService, conn).expect("Error count items")
    }

    /// Catalogs to which the service is added
    async fn service_specs(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
    ) -> Vec<SpecTranslateList> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        SpecTranslateList::for_service_by_uuid(&self.uuid, &get_set_language(cxt), &p, conn)
            .expect("Error loading service keywords")
    }

    /// Service keywords (tags)
    async fn service_keywords(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
    ) -> Vec<Keyword> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        Keyword::for_service_without_check(&self.uuid, &p, conn)
            .expect("Error loading service keywords")
    }
}

/// Abbreviated data about the service
#[derive(Debug)]
pub struct ShowServiceShort {
    /// Identifier of the service on the platform
    pub(crate) uuid: Uuid,
    /// Service name
    pub(crate) name: String,
    /// Service description
    pub(crate) description: String,
    /// Data about the profile owning the service
    pub(crate) owner_user: ShowUserShort,
    /// Data about the company that owns the service
    pub(crate) owner_company: ShowCompanyShort,
    /// Current status of the service (e.g. "created")
    pub(crate) service_status: ServiceStatusTranslateList,
    /// Update date of the basic service data
    pub(crate) updated_at: NaiveDateTime,
}

#[Object]
impl ShowServiceShort {
    /// Identifier of the service on the platform
    async fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Service name
    async fn name(&self) -> &String {
        &self.name
    }

    /// Service description
    async fn description(&self) -> &String {
        &self.description
    }

    /// Data about the profile owning the service
    async fn owner_user(&self) -> &ShowUserShort {
        &self.owner_user
    }

    /// Data about the company owning the service
    async fn owner_company(&self) -> &ShowCompanyShort {
        &self.owner_company
    }

    /// Current status of the service (e.g. "created")
    async fn service_status(&self) -> &ServiceStatusTranslateList {
        &self.service_status
    }

    /// Update date of the basic service data
    async fn updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    /// Files (images by default) associated with the service
    async fn files(
        &self,
        cxt: &Context<'_>,
        paginate: Option<IptPaginate>,
        images: Option<bool>,
    ) -> Vec<DownloadFile> {
        let conn: &mut PooledConnection = &mut get_conn(cxt).expect("Error get conn to DB");
        let p = paginate
            .map(|p| Paginate::parsing_by_page(p.current_page, p.per_page))
            .unwrap_or_default();
        match images {
            Some(false) => DownloadFile::by_service_uuid(&self.uuid, &p, conn)
                .expect("Error loading service files"),
            _ => DownloadFile::service_image_files(&self.uuid, &p, conn)
                .expect("Error loading service image files"),
        }
    }
}

/// Data for registering a new service on the platform
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptServiceData {
    /// Service name
    pub(crate) name: String,
    /// Service description
    pub(crate) description: String,
    /// Identifier of the company owning the service
    pub(crate) company_uuid: Uuid,
    /// Identifier of the region of application (development) of the service
    pub(crate) region_id: i32,
}

/// Data for updating the service card.
/// The data is only updated for the specified values.
#[derive(Debug, Deserialize, Clone, InputObject)]
pub(crate) struct IptUpdateServiceData {
    /// Service name
    pub(crate) name: Option<String>,
    /// Service description
    pub(crate) description: Option<String>,
    /// Identifier of the region of application (development) of the service
    pub(crate) region_id: Option<i32>,
}

/// Arguments for filtering and searching by services
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptServicesArg {
    /// Filter by service UUID
    pub(crate) services_uuids: Option<Vec<Uuid>>,
    /// Filter by company owning the service
    pub(crate) company_uuid: Option<Uuid>,
    /// Filter by user owning the service
    pub(crate) user_uuid: Option<Uuid>,
}

impl ServicesArg {
    /// Returns a ServicesArg with the given arguments
    pub(crate) fn by_arg(data: Option<IptServicesArg>) -> Self {
        match data {
            Some(data) => Self {
                filter_services_uuids: data.services_uuids.unwrap_or_default(),
                company_uuid: data.company_uuid,
                user_uuid: data.user_uuid,
            },
            None => ServicesArg::default(),
        }
    }
}

/// Arguments for filtering and searching by service files
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptServiceFilesArg {
    /// Filter by service UUID
    pub(crate) service_uuid: Uuid,
    /// Filter by service UUID files
    pub(crate) files_uuids: Option<Vec<Uuid>>,
}

impl From<IptServiceFilesArg> for ServiceFilesArg {
    fn from(data: IptServiceFilesArg) -> Self {
        Self {
            service_uuid: data.service_uuid,
            file_uuids: data.files_uuids.unwrap_or_default(),
        }
    }
}

/// Arguments for change status of the state (readiness) of the service
#[derive(InputObject, Deserialize, Debug)]
pub(crate) struct IptServiceStatusArg {
    /// Filter by service UUID
    pub(crate) service_uuid: Uuid,
    /// Identifier of the status of the state (readiness) of the service
    pub(crate) service_status_id: i32,
}
