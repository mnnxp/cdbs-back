use crate::graphql::service_model::IptServiceData;
use crate::schema::*;
use chrono::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = service_ref)]
pub(crate) struct Service {
    pub(crate) uuid: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) user_uuid: Uuid,
    pub(crate) company_uuid: Uuid,
    pub(crate) service_status_id: i32,
    pub(crate) region_id: i32,
    // pub(crate) is_delete: bool,
    pub(crate) created_at: NaiveDateTime,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = service_ref)]
pub(crate) struct InsertableService {
    uuid: Uuid,
    name: String,
    description: String,
    user_uuid: Uuid,
    company_uuid: Uuid,
    type_access_id: i32,
    service_status_id: i32,
    region_id: i32,
    is_delete: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl InsertableService {
    pub(crate) fn by_args(ipt_data: &IptServiceData, user_uuid: &Uuid) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            name: ipt_data.name.clone(),
            description: ipt_data.description.clone(),
            user_uuid: *user_uuid,
            company_uuid: ipt_data.company_uuid,
            type_access_id: 1,
            service_status_id: 1,
            region_id: ipt_data.region_id,
            is_delete: false,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ServicesArg {
    pub(crate) filter_services_uuids: Vec<Uuid>,
    pub(crate) company_uuid: Option<Uuid>,
    pub(crate) user_uuid: Option<Uuid>,
}

#[derive(Debug)]
pub(crate) struct ServiceFilesArg {
    pub(crate) service_uuid: Uuid,
    pub(crate) file_uuids: Vec<Uuid>,
}
